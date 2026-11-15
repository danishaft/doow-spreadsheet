# Formualizer — Research Notes

> Observations only. Decisions go to `ADRs/`.
> Researched: 2026-11-13. Source: web docs + repo README.

## Identity

- Repo: `PSU3D0/formualizer` — https://github.com/PSU3D0/formualizer
- Docs: https://www.formualizer.dev
- License: MIT OR Apache-2.0
- Current version at research time: `formualizer` 0.14.0 (pub. 2026-11-08 per lib.rs);
  `formualizer-eval` 0.12–0.13. 38 releases, 11 breaking.
- Targets: Rust crate, Python (incl. Pyodide), WASM/JS.

## What it is

An embeddable spreadsheet engine: parse formulas, store workbook cells,
track dependencies, evaluate Excel-compatible functions. Explicitly positioned
for deterministic agent workflows and production pipelines.

## Architecture (as documented)

Layered crates:

```
formualizer              meta crate (re-exports)
  formualizer-workbook   sheets, cells, undo/redo, XLSX/CSV/JSON I/O
    formualizer-eval     calculation engine, dependency authority, built-ins
      formualizer-parse  tokenizer, parser, AST, pretty-printer
      formualizer-common shared types (values, errors, references)
  formualizer-sheetport  typed spreadsheet-as-API runtime
```

Request flow: write cell → parse to AST → evaluate refs/functions → workbook
graph tracks dependencies for incremental recalculation.

## Notable design choices

1. **Arrow-backed columnar storage.** Copied formulas are stored as "formula
   families" (one template + one dependency node) and evaluated in one pass over
   Arrow columns instead of N separate formulas.
2. **Region-level dependency tracking.** Editing one input touches only readers.
   Whole-row/column inserts move runs at once.
3. **Incremental recalc + cycle detection + optional parallel evaluation.**
   CSR edge format for the graph.
4. **Deterministic evaluation.** Injectable clock, timezone and random seed —
   important for reproducible tests and agent runs.
5. **Cell-by-cell engine retained as a test oracle** against the fast paths.
6. **Per-cell semantics preserved bit-for-bit**, including summation order,
   error precedence and number formats (per README claim).
7. **SheetPort**: typed input/output manifests (code or YAML) that turn a
   workbook into a validated function.
8. **Auditable change log + undo/redo**, transactional.
9. **Custom functions** from Rust/Python/JS plus sandboxed WASM plugins.

## Performance claims (vendor-reported, not verified by us)

- 1,000,000 formulas in **0.26 s** (native release, ephemeral, median of 3);
  separate load 2.58 s; single input change recalc **48 ms**.
- Against LibreOffice Calc 24.2 across 52 workloads: geometric-mean **0.184×**
  time, faster in 50 of 52.
- Two losses reported honestly: one model 1.2× and `Mesa_Model_1_19_01` 2.6×
  slower than LibreOffice.
- These are **vendor benchmarks on a shared 24-thread machine**; not independent.

## Gate 0 measurements (ours, 2026-11-13)

Full detail in `research/benchmarks/results.md`; spike source in
`research/benchmarks/spike/`. Host: i5-8257U, 4c/8t, 8 GB, macOS 15.7.7,
rustc 1.100.0, release build, default features.

| Case | Ours | Vendor claim | Result |
| ---- | ---- | ------------ | ------ |
| 1M formulas, build + first calc | ~2.9 s calc (build ~3.7 s) | 0.26 s | not reproduced (~11×) |
| 1M dependents, broad invalidation recalc | ~250–270 ms (+ ~60–76 ms in `set_value`) | 48 ms | not reproduced |
| 100k narrow edit (A1→B1→C1) | 1.6–2.3 ms | — | fast, but a trivial dirty set |
| Peak RSS, 1M values + 1M formulas | ≈ 580 MB | not stated | — |

All correctness checks passed (SUM matched closed form exactly at 100k and 1M).
Build requires `zip` pinned to 7.2.0; 7.3/7.4 fail to compile the crate.

## Gate 0 fidelity probe (ours, 2026-11-13)

Full detail in `research/corpus-probe/results.md`. 84 vectors, Engine 0.14.0,
no Excel/LibreOffice oracle available.

- **83 / 84 pass** across dates, lookups, dynamic arrays, error precedence and
  floating point.
- Dynamic arrays **spill correctly** (anchor + neighbouring cells), including
  `SEQUENCE`, `UNIQUE`, `SORT`, `FILTER`, `TRANSPOSE`, nesting; empty `FILTER`
  yields `#CALC!`.
- Excel's **15-significant-digit pre-rounding** is implemented (`ROUND(2.675,2)`
  → 2.68; `ROUND(37.785,2)` → 37.79).
- Lazy `IF`, `IFERROR`/`IFNA`, error taxonomy and coercion differences
  (`SUM` of text argument vs text in a range) agree with Excel.
- **One divergence:** `DATE(1900,2,29)` normalises to 1900-03-01 instead of
  Excel's phantom serial 60. Affects dates before 1900-03-01 only.
- Caveat: expectations are authored/documented, not oracle-confirmed. Two of our
  initial expectations were wrong and the engine was right.



## Strengths for us

- Directly relevant architecture: parse/eval/graph/workbook separation is the
  shape we need.
- Formula families + region dependency tracking is a concrete answer to the
  large-workbook performance problem.
- Determinism + change log + typed I/O align with an agentic product.
- Permissive license removes the HyperFormula AGPL problem.

## Limitations / risks

- Young and churning: 11 breaking releases; pin versions and run workbook
  regression tests on upgrade (their own advice).
- Broad API surface; leaning on the meta crate couples us to their model.
- Benchmark numbers are self-reported; the "fastest" claim needs independent
  reproduction on our workloads.
- Writing path uses `umya`; native reader is newer. XLSX round-trip fidelity is
  not the same problem as calculation correctness.
- Exact Excel bug-for-bug behavior is claimed but must be tested against our
  corpus, not assumed.
- Not a UI/rendering/collaboration engine — those are ours to build.

## Questions to resolve in Gate 0 spike

- Reproduce the million-formula and 100k-copied-formula cases on our hardware.
- Measure incremental recalc latency for realistic edit patterns.
- Test corpus fidelity on tricky functions (date/time, lookups, dynamic arrays,
  error precedence, floating-point summation order).
- Evaluate using `formualizer-eval` alone (own data model) vs the full workbook.
- Assess API stability cost of tracking upstream.
