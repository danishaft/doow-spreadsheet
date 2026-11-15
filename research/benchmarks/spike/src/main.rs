use std::time::Instant;

use formualizer::{LiteralValue, Workbook};

const SHEET: &str = "Bench";

fn build_copied(n: u32) -> Workbook {
    let mut wb = Workbook::new();
    wb.set_changelog_enabled(false);
    wb.add_sheet(SHEET).expect("add sheet");

    let t_build = Instant::now();

    let values: Vec<Vec<LiteralValue>> = (1..=n)
        .map(|i| vec![LiteralValue::Number(i as f64)])
        .collect();
    wb.set_values(SHEET, 1, 1, &values).expect("set A values");

    let formulas: Vec<Vec<String>> = (1..=n)
        .map(|i| vec![format!("=A{i}*2")])
        .collect();
    wb.set_formulas(SHEET, 1, 2, &formulas).expect("set B formulas");

    wb.set_formula(SHEET, 1, 3, &format!("=SUM(B1:B{n})"))
        .expect("set C1 formula");

    let build_ms = t_build.elapsed().as_secs_f64() * 1000.0;
    let t_calc = Instant::now();
    wb.evaluate_all().expect("evaluate_all");
    let calc_ms = t_calc.elapsed().as_secs_f64() * 1000.0;

    let expected = 2.0 * (n as f64) * ((n as f64) + 1.0) / 2.0;
    let got = wb.get_value(SHEET, 1, 3);
    let ok = matches!(got, Some(LiteralValue::Number(v)) if (v - expected).abs() < 1e-6);

    println!(
        "case=copied n={n} build_ms={build_ms:.2} first_calc_ms={calc_ms:.2} expected={expected:.0} got={got:?} ok={ok}"
    );
    wb
}

fn incremental(n: u32) {
    let mut wb = build_copied(n);

    let t_edit = Instant::now();
    wb.set_value(SHEET, 1, 1, LiteralValue::Number(2.0))
        .expect("edit A1");
    let edit_ms = t_edit.elapsed().as_secs_f64() * 1000.0;

    let t_inc = Instant::now();
    wb.evaluate_all().expect("incremental evaluate_all");
    let inc_ms = t_inc.elapsed().as_secs_f64() * 1000.0;

    let expected = 2.0 * (n as f64) * ((n as f64) + 1.0) / 2.0 + 2.0;
    let got = wb.get_value(SHEET, 1, 3);
    let ok = matches!(got, Some(LiteralValue::Number(v)) if (v - expected).abs() < 1e-6);

    println!(
        "case=incremental n={n} edit_ms={edit_ms:.2} recalc_ms={inc_ms:.2} expected={expected:.0} got={got:?} ok={ok}"
    );

    let t_target = Instant::now();
    let v = wb.evaluate_cells(&[(SHEET, 1, 3)]).expect("targeted eval");
    let target_ms = t_target.elapsed().as_secs_f64() * 1000.0;
    println!(
        "case=targeted n={n} target_eval_ms={target_ms:.2} value={v:?}"
    );
}

/// Broad-invalidation case: every formula reads a shared parameter cell, so a
/// single edit dirties all n formulas plus the aggregate.
fn broad(n: u32) {
    let mut wb = Workbook::new();
    wb.set_changelog_enabled(false);
    wb.add_sheet(SHEET).expect("add sheet");

    let t_build = Instant::now();
    wb.set_value(SHEET, 1, 5, LiteralValue::Number(3.0))
        .expect("set parameter");
    let values: Vec<Vec<LiteralValue>> = (1..=n)
        .map(|i| vec![LiteralValue::Number(i as f64)])
        .collect();
    wb.set_values(SHEET, 1, 1, &values).expect("set A values");
    let formulas: Vec<Vec<String>> = (1..=n)
        .map(|i| vec![format!("=A{i}*$E$1")])
        .collect();
    wb.set_formulas(SHEET, 1, 2, &formulas).expect("set B formulas");
    wb.set_formula(SHEET, 1, 3, &format!("=SUM(B1:B{n})"))
        .expect("set C1 formula");
    let build_ms = t_build.elapsed().as_secs_f64() * 1000.0;

    let t_calc = Instant::now();
    wb.evaluate_all().expect("evaluate_all");
    let calc_ms = t_calc.elapsed().as_secs_f64() * 1000.0;

    let sum = (n as f64) * ((n as f64) + 1.0) / 2.0;
    let expected = 3.0 * sum;
    let got = wb.get_value(SHEET, 1, 3);
    let ok = matches!(got, Some(LiteralValue::Number(v)) if (v - expected).abs() < 1e-6);
    println!(
        "case=broad n={n} build_ms={build_ms:.2} first_calc_ms={calc_ms:.2} expected={expected:.0} got={got:?} ok={ok}"
    );

    let t_edit = Instant::now();
    wb.set_value(SHEET, 1, 5, LiteralValue::Number(4.0))
        .expect("edit parameter");
    let edit_ms = t_edit.elapsed().as_secs_f64() * 1000.0;

    let t_inc = Instant::now();
    wb.evaluate_all().expect("broad recalc");
    let recalc_ms = t_inc.elapsed().as_secs_f64() * 1000.0;

    let expected2 = 4.0 * sum;
    let got2 = wb.get_value(SHEET, 1, 3);
    let ok2 = matches!(got2, Some(LiteralValue::Number(v)) if (v - expected2).abs() < 1e-6);
    println!(
        "case=broad_recalc n={n} edit_ms={edit_ms:.2} recalc_ms={recalc_ms:.2} expected={expected2:.0} got={got2:?} ok={ok2}"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("all");

    println!(
        "formualizer_spike host={}-{} pid={}",
        std::env::consts::ARCH,
        std::env::consts::OS,
        std::process::id()
    );

    if mode == "all" || mode == "100k" {
        for i in 1..=3 {
            println!("-- run {i} --");
            incremental(100_000);
        }
        broad(100_000);
    }
    if mode == "all" || mode == "1m" {
        println!("-- 1m --");
        incremental(1_000_000);
        broad(1_000_000);
    }
    if mode == "broad" {
        broad(100_000);
        broad(1_000_000);
    }
}
