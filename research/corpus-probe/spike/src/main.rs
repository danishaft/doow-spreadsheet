use std::collections::BTreeMap;
use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};

use formualizer::{LiteralValue, Workbook};
use serde::Deserialize;

const SHEET: &str = "S";
const TARGET: &str = "Z1";

#[derive(Deserialize, Debug, Clone)]
struct Case {
    id: String,
    area: String,
    source: String,
    #[serde(default)]
    notes: String,
    #[serde(default)]
    cells: Vec<CellVal>,
    formula: String,
    expected: Expected,
    #[serde(default = "default_tol")]
    tolerance: f64,
}

fn default_tol() -> f64 {
    1e-9
}

#[derive(Deserialize, Debug, Clone)]
struct CellVal {
    addr: String,
    kind: String,
    value: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
struct Expected {
    kind: String,
    value: serde_json::Value,
}

fn parse_addr(addr: &str) -> (u32, u32) {
    let mut letters = String::new();
    let mut digits = String::new();
    for ch in addr.chars() {
        if ch.is_ascii_alphabetic() {
            letters.push(ch.to_ascii_uppercase());
        } else if ch.is_ascii_digit() {
            digits.push(ch);
        }
    }
    let mut col = 0u32;
    for b in letters.bytes() {
        col = col * 26 + (b - b'A' + 1) as u32;
    }
    (digits.parse().expect("row number"), col)
}

fn cell_literal(kind: &str, value: &serde_json::Value) -> LiteralValue {
    match kind {
        "number" => LiteralValue::Number(value.as_f64().expect("numeric cell")),
        "text" => LiteralValue::Text(value.as_str().expect("text cell").to_string()),
        "bool" => LiteralValue::Boolean(value.as_bool().expect("bool cell")),
        other => panic!("unknown cell kind {other}"),
    }
}

fn as_number(v: &LiteralValue) -> Option<f64> {
    match v {
        LiteralValue::Int(i) => Some(*i as f64),
        LiteralValue::Number(n) => Some(*n),
        _ => None,
    }
}

fn render(v: &Option<LiteralValue>) -> String {
    match v {
        None => "<none>".to_string(),
        Some(LiteralValue::Error(e)) => e.to_string(),
        Some(other) => other.to_string(),
    }
}

fn spill_probe(wb: &Workbook, row: u32, col: u32) -> String {
    let mut below = Vec::new();
    let mut right = Vec::new();
    for d in 1..=4 {
        if let Some(v) = wb.get_value(SHEET, row + d, col) {
            if !matches!(v, LiteralValue::Empty) {
                below.push(format!("{d}:{}", render(&Some(v))));
            }
        }
        if let Some(v) = wb.get_value(SHEET, row, col + d) {
            if !matches!(v, LiteralValue::Empty) {
                right.push(format!("{d}:{}", render(&Some(v))));
            }
        }
    }
    format!(
        "below=[{}] right=[{}]",
        below.join(","),
        right.join(",")
    )
}

enum Verdict {
    Pass,
    Wrong(String),
    ErrorInstead(String),
    ErrorMismatch(String),
    Shape(String),
}

fn compare(expected: &Expected, got: &Option<LiteralValue>, tol: f64) -> Verdict {
    match expected.kind.as_str() {
        "number" => {
            let want = expected.value.as_f64().expect("numeric expectation");
            match got {
                Some(v) => match as_number(v) {
                    Some(n) => {
                        if (n - want).abs() <= tol {
                            Verdict::Pass
                        } else {
                            Verdict::Wrong(format!("number {n} != {want}"))
                        }
                    }
                    None => match v {
                        LiteralValue::Error(e) => Verdict::ErrorInstead(e.to_string()),
                        other => Verdict::Shape(format!("expected number, got {other:?}")),
                    },
                },
                None => Verdict::Wrong("cell has no value".to_string()),
            }
        }
        "text" => {
            let want = expected.value.as_str().expect("text expectation");
            match got {
                Some(LiteralValue::Text(s)) if s == want => Verdict::Pass,
                Some(LiteralValue::Text(s)) => Verdict::Wrong(format!("text {s:?} != {want:?}")),
                Some(LiteralValue::Error(e)) => Verdict::ErrorInstead(e.to_string()),
                other => Verdict::Shape(format!("expected text, got {}", render(&other.clone()))),
            }
        }
        "bool" => {
            let want = expected.value.as_bool().expect("bool expectation");
            match got {
                Some(LiteralValue::Boolean(b)) if *b == want => Verdict::Pass,
                Some(LiteralValue::Boolean(b)) => Verdict::Wrong(format!("bool {b} != {want}")),
                Some(LiteralValue::Error(e)) => Verdict::ErrorInstead(e.to_string()),
                other => Verdict::Shape(format!("expected bool, got {}", render(&other.clone()))),
            }
        }
        "error" => {
            let want = expected.value.as_str().expect("error expectation");
            match got {
                Some(LiteralValue::Error(e)) => {
                    let got_err = e.to_string();
                    if got_err == want {
                        Verdict::Pass
                    } else {
                        Verdict::ErrorMismatch(format!("{got_err} != {want}"))
                    }
                }
                other => Verdict::Shape(format!(
                    "expected error {want}, got {}",
                    render(&other.clone())
                )),
            }
        }
        "array" | "spill" => {
            let want_rows: Vec<Vec<f64>> = serde_json::from_value(expected.value.clone())
                .expect("array of rows of numbers");
            let want_shape = (want_rows.len(), want_rows.first().map_or(0, Vec::len));
            match got {
                Some(LiteralValue::Array(rows)) => {
                    let got_shape = (rows.len(), rows.first().map_or(0, Vec::len));
                    if got_shape != want_shape {
                        return Verdict::Shape(format!(
                            "shape {:?} != {:?}",
                            got_shape, want_shape
                        ));
                    }
                    for (ri, row) in rows.iter().enumerate() {
                        if row.len() != want_rows[ri].len() {
                            return Verdict::Shape(format!(
                                "row {ri} length {} != {}",
                                row.len(),
                                want_rows[ri].len()
                            ));
                        }
                        for (ci, cell) in row.iter().enumerate() {
                            let want = want_rows[ri][ci];
                            match as_number(cell) {
                                Some(n) if (n - want).abs() <= tol => {}
                                Some(n) => {
                                    return Verdict::Wrong(format!(
                                        "element[{ri}][{ci}] {n} != {want}"
                                    ))
                                }
                                None => {
                                    return Verdict::Wrong(format!(
                                        "element[{ri}][{ci}] not numeric: {cell:?}"
                                    ))
                                }
                            }
                        }
                    }
                    Verdict::Pass
                }
                Some(LiteralValue::Error(e)) => Verdict::ErrorInstead(e.to_string()),
                other => Verdict::Shape(format!(
                    "expected array, got {}",
                    render(&other.clone())
                )),
            }
        }
        other => panic!("unknown expected kind {other}"),
    }
}

fn classify(verdict: Verdict) -> (&'static str, String) {
    match verdict {
        Verdict::Pass => ("pass", String::new()),
        Verdict::Wrong(d) => ("wrong_value", d),
        Verdict::ErrorInstead(e) => {
            if e == "#N/IMPL!" || e == "#NAME?" || e == "#N/A" {
                ("unsupported_or_error", e)
            } else {
                ("error_instead_of_value", e)
            }
        }
        Verdict::ErrorMismatch(d) => ("error_mismatch", d),
        Verdict::Shape(d) => ("type_or_shape", d),
    }
}

fn run_case(c: &Case) -> (&'static str, String, String) {
    let mut wb = Workbook::new();
    wb.set_changelog_enabled(false);
    wb.add_sheet(SHEET).expect("add sheet");

    for cell in &c.cells {
        let (row, col) = parse_addr(&cell.addr);
        wb.set_value(SHEET, row, col, cell_literal(&cell.kind, &cell.value))
            .expect("set cell");
    }
    let (row, col) = parse_addr(TARGET);
    if let Err(e) = wb.set_formula(SHEET, row, col, &c.formula) {
        return ("engine_reject", e.to_string(), String::new());
    }
    if let Err(e) = wb.evaluate_all() {
        return ("engine_reject", e.to_string(), String::new());
    }
    let got = wb.get_value(SHEET, row, col);
    let rendered = render(&got);
    // Excel spill semantics: the anchor cell holds the top-left value and the
    // rest of the block lands in neighbouring cells. Reconstruct the block.
    let compare_value = if c.expected.kind == "spill" {
        let want_rows: Vec<Vec<f64>> = serde_json::from_value(c.expected.value.clone())
            .expect("spill rows of numbers");
        let rows = want_rows.len();
        let cols = want_rows.first().map_or(0, Vec::len);
        let mut block: Vec<Vec<LiteralValue>> = Vec::with_capacity(rows);
        for r in 0..rows {
            let mut line = Vec::with_capacity(cols);
            for cc in 0..cols {
                line.push(
                    wb.get_value(SHEET, row + r as u32, col + cc as u32)
                        .unwrap_or(LiteralValue::Empty),
                );
            }
            block.push(line);
        }
        Some(LiteralValue::Array(block))
    } else {
        got
    };
    let (kind, detail) = classify(compare(&c.expected, &compare_value, c.tolerance));
    let detail = if (c.expected.kind == "array" || c.expected.kind == "spill") && kind != "pass" {
        format!("{detail}; spill {}", spill_probe(&wb, row, col))
    } else {
        detail
    };
    (kind, detail, rendered)
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("usage: corpus_probe <vectors.jsonl>");
    let body = fs::read_to_string(&path).expect("read vectors");
    let mut cases = Vec::new();
    for (lineno, line) in body.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let case: Case = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("line {}: {e}", lineno + 1));
        cases.push(case);
    }

    let mut totals: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut failures = 0usize;

    for case in &cases {
        let outcome = catch_unwind(AssertUnwindSafe(|| run_case(case)));
        let (kind, detail, rendered) = match outcome {
            Ok(v) => v,
            Err(_) => ("panic", "probe panicked".to_string(), "<panic>".to_string()),
        };
        let entry = totals.entry(case.area.clone()).or_insert((0, 0));
        entry.0 += 1;
        if kind == "pass" {
            entry.1 += 1;
        } else {
            failures += 1;
            println!(
                "FAIL {} area={} source={} what={} got={} detail={}{}",
                case.id,
                case.area,
                case.source,
                kind,
                rendered,
                detail,
                if case.notes.is_empty() {
                    String::new()
                } else {
                    format!(" notes={}", case.notes)
                }
            );
        }
    }

    println!();
    for (area, (total, pass)) in &totals {
        println!("AREA {area} total={total} pass={pass} fail={}", total - pass);
    }
    println!(
        "TOTAL cases={} pass={} fail={}",
        cases.len(),
        cases.len() - failures,
        failures
    );
}
