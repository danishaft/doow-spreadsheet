# Benchmarks — Formualizer Gate 0 Spike

> Observations only. Decisions go to `ADRs/`.
> Run: 2026-11-13 · Crate: `formualizer` 0.14.0 · Spike source: `spike/`

## 1. Environment

| Item | Value |
| ---- | ----- |
| Host | MacBook (Intel), `x86_64-apple-darwin` |
| CPU | Intel Core i5-8257U @ 1.40 GHz — 4 cores / 8 threads |
| RAM | 8 GB |
| OS | macOS 15.7.7 (24G720) |
| Toolchain | rustc 1.100.0, cargo 1.100.0 (stable) |
| Build | `cargo build --release`, single process, no parallelism flag |
| Engine config | `Workbook::new()`, `set_changelog_enabled(false)`, default features (`json`, `csv`, `system-clock`, `xlsx-recalc`) |

This is a laptop-class 4-core machine, not the vendor's 24-thread benchmark host.
Numbers are indicative, not statistically rigorous: no warm-up, no median, 3 runs
at 100k and 1 run at 1M.

## 2. Reproduce

```sh
cd research/benchmarks/spike
cargo build --release --locked
./target/release/fzspike 100k    # 3 × copied-formula case
./target/release/fzspike 1m      # 1M copied-formula case
./target/release/fzspike broad   # shared-parameter (broad invalidation) case
```

`Cargo.lock` is committed because the default resolution **does not build**
(see §6). Do not regenerate it blindly. A cold build compiles ~250 crates and
took ~7.5 min on the host; subsequent builds are seconds.

## 3. Scenarios

All scenarios: column A = values `1..n`; column B = `=A{i}*2` (copied case) or
`=A{i}*$E$1` (broad case); C1 = `=SUM(B1:Bn)`. "Results" are the first
calculation; `ok` means C1 matched the closed-form expected value exactly.

- **copied** — n copied formulas, no shared inputs. Edit A1, then recalc.
  Dirty set after edit: B1 and C1 only (narrow).
- **broad** — n copied formulas all reading one parameter cell `$E$1`. Edit E1,
  then recalc. Dirty set after edit: all n formulas + C1 (broad/realistic).
- **targeted** — `evaluate_cells([C1])` after the incremental edit.

## 4. Raw results

### 100k (3 runs)

| Run | Build ms | First calc ms | Edit ms | Narrow recalc ms | Targeted ms |
| --- | -------- | ------------- | ------- | ---------------- | ----------- |
| 1 | 323.16 | 250.52 | 0.03 | 2.33 | 310.75 |
| 2 | 307.24 | 239.24 | 0.02 | 1.58 | 306.26 |
| 3 | 308.64 | 263.41 | 0.03 | 2.03 | 303.89 |

Broad case (100k): build 272.67 / 277.75 ms, first calc 257.80 / 269.60 ms,
edit 5.14 / 5.93 ms, **recalc 22.99 / 27.19 ms**.

### 1M

| Case | Build ms | First calc ms | Edit ms | Recalc ms | Targeted ms |
| ---- | -------- | ------------- | ------- | --------- | ----------- |
| copied | 3659.51 | 2900.41 | 0.04 | 2.73 | 4017.01 |
| broad | 2813.13 / 2769.04 | 2872.89 / 3118.72 | 75.79 / 59.01 | **269.22 / 246.44** | — |

Peak RSS for the 1M broad run: **≈ 580 MB** (`/usr/bin/time -l`, max resident).

Every correctness check passed (`ok=true`) in all runs, at both sizes.

## 5. Interpretation

- **First calculation of 1M formulas ≈ 2.9 s** on this machine, i.e. roughly
  **345k formulas/s**. The vendor reports 1M formulas in **0.26 s**. We did not
  reproduce that: we are ~11× slower, on a much weaker CPU with a different
  workload. The claim is not falsified, but it is also not transferable as-is.
- **Narrow incremental recalc is very fast** (1.6–2.7 ms at both 100k and 1M),
  because editing A1 dirties only B1 and C1. This number should never be quoted
  as "edit latency" — it measures almost nothing.
- **Broad invalidation is the honest incremental figure**: 23–27 ms at 100k and
  **~250–270 ms for 1M dependent formulas** (plus 59–76 ms inside
  `set_value` itself). Vendor's "single input change 48 ms" is not reproduced;
  total observed edit→recalculated latency is ~315–345 ms at 1M.
- **`evaluate_cells` on C1 is slower than a full incremental `evaluate_all`**
  (4.0 s vs 2.7 ms at 1M). Either it bypasses the dirty-set path or it
  re-evaluates the transitive chain. Treat targeted evaluation as a separate,
  unoptimised path until its semantics are documented. Do not assume
  "evaluate one cell" is cheaper than "recalculate changed cells".
- Memory: ~580 MB peak for 1M values + 1M formulas + aggregate. Acceptable on a
  developer laptop, notable for an 8 GB machine if the product targets larger
  workbooks or multiple open documents.

## 6. Integration finding (build break)

`formualizer-workbook` 0.14.0 declares `zip = "7.2"`, and the code casts
`payload.data_start()` (now `Option<u64>` in newer `zip`) directly to `usize`.
Fresh resolution picks **zip 7.4.0**, which fails to compile:

```
error[E0605]: non-primitive cast: `std::option::Option<u64>` as `usize`
  --> formualizer-workbook-0.14.0/src/xlsx/package/rewrite.rs:66:16
```

zip **7.3.0 also fails**; **7.2.0 builds**. This is a real cost of tracking
Formualizer: a transitive minor-version bump breaks the build with no code
change on our side. It confirms the "upstream churn" risk and the need to pin
exact versions plus commit lockfiles.

## 7. Gaps / next probes

- Measure a *mixed* dirty set (some formulas depend on an edit, most do not),
  which is the common interactive case.
- Measure workbook load from XLSX (build time here is in-memory writes only).
- Run with parallel evaluation enabled and compare.
- Repeat on a stronger machine if these numbers are used for a decision.
- Memory scaling at 5M–10M formulas on an 8 GB host.
