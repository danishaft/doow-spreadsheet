# Fidelity Probe — Formualizer vs Excel Semantics (Gate 0)

> Observations only. Decisions go to `ADRs/`.
> Run: 2026-11-13 · Engine: `formualizer` 0.14.0 · Host: i5-8257U, 8 GB, macOS 15.7.7, rustc 1.100.0
> Vectors: `test-corpus/vectors/gate0-fidelity.jsonl` · Harness: `research/corpus-probe/spike/`

## 1. What this probe is and is not

It runs 84 formula vectors covering date/time, lookups, dynamic arrays, error
precedence and floating-point behaviour, and compares the engine's result with
the expected Excel-semantic result.

**It is not an oracle test.** No Excel or LibreOffice is installed on this host,
so expectations are encoded from documented Excel behaviour (`source: excel`),
IEEE double arithmetic (`source: ieee`), or our own reasoning
(`source: authored`). A green run means "agrees with our encoded understanding",
not "agrees with Excel". The 1900 quirk below and the corrected ROUND vector
show why that distinction matters.

## 2. Reproduce

```sh
cd research/corpus-probe/spike
cargo build --release --locked
./target/release/corpus_probe ../../../test-corpus/vectors/gate0-fidelity.jsonl
```

The lockfile pins `zip` 7.2.0 for the same reason as the benchmark spike
(see `research/benchmarks/results.md` §6).

## 3. Results

**83 / 84 pass.** Breakdown:

| Area | Vectors | Pass | Fail |
| ---- | ------- | ---- | ---- |
| Date/time | 19 | 18 | 1 |
| Lookups | 15 | 15 | 0 |
| Dynamic arrays | 11 | 11 | 0 |
| Error precedence | 23 | 23 | 0 |
| Floating point | 16 | 16 | 0 |
| **Total** | **84** | **83** | **1** |

Provenance: 76 Excel-documented, 4 IEEE-derived, 4 authored.

## 4. The one failure

`date-017` — Excel's 1900 leap-year bug.

- Excel: `DATE(1900,2,29)` returns serial **60**, a day that never existed
  (Microsoft documents this as intentional Lotus 1-2-3 compatibility).
- Formualizer: normalises to **1900-03-01** (`19000301`).
- Impact: dates before 1900-03-01 are off by one day relative to Excel. For a
  modern product this is almost certainly out of scope, but it is a **conscious
  divergence**, not an accident, and should be recorded as such in the corpus
  policy rather than "fixed" silently.

## 5. Corrections the probe forced on us

These are as important as the passes, because they show the method working:

1. **`ROUND(2.675,2)`** — our first expectation was 2.67 (naive IEEE). Excel
   actually returns **2.68**: it rounds to 15 significant digits first, then to
   the requested places (documented by Microsoft; same for `ROUND(37.785,2)` →
   37.79). The engine matched Excel; **our vector was wrong**. Fixed, and a
   second vector added.
2. **Dynamic arrays** — our first harness required the anchor cell to hold an
   array value. The engine spills correctly: the anchor holds the top-left
   scalar and the block lands in neighbouring cells. **The engine was right;
   the probe's contract was wrong.** The corpus now expresses spill regions
   (`kind: spill`), which is Excel's actual semantics.

## 6. What passed, specifically

- **Dates:** leap day, day-zero normalisation, month overflow, `EDATE`/`EOMONTH`
  month-end clamping, `DATEDIF` Y/M/D, `DATEVALUE`, `TIME`, `HOUR`, `WEEKDAY`,
  `NETWORKDAYS`, `YEARFRAC`, `DAYS`.
- **Lookups:** `VLOOKUP` exact/approximate/miss, text-vs-number non-coercion,
  case-insensitivity, wildcards, `HLOOKUP`, `INDEX`+`MATCH`, `XLOOKUP` including
  `if_not_found`.
- **Dynamic arrays:** `SEQUENCE` (1-D/2-D), `UNIQUE` order, `SORT`, `FILTER`
  (including empty → `#CALC!`), `TRANSPOSE`, nesting, array consumption by
  `SUM`, `SUMPRODUCT` with array constants, `LET`.
- **Errors:** all Excel error kinds exercised (`#DIV/0!`, `#N/A`, `#NUM!`,
  `#VALUE!`, `#REF!`, `#NAME?`), `IFERROR`/`IFNA`, `ISERROR`/`ISNA`, lazy `IF`,
  error propagation, text-in-range vs text-argument `SUM` difference.
- **Floating point:** binary representation (`0.1+0.2`), comparison `FALSE`,
  15-significant-digit rounding, half-away-from-zero, `INT`/`TRUNC`/`MOD`
  signs, sequential-summation loss, overflow to `#NUM!`.

## 7. Coverage limits — do not over-read this

84 vectors is a smoke test against 400+ claimed functions. Not covered:

- Locale/date parsing, 1904 date system, pre-1900-03-01 dates.
- `#SPILL!` collision handling, implicit intersection, whole-column refs.
- Tables/structured references, defined names, 3-D refs.
- Volatile functions and deterministic clock/seed behaviour in practice.
- XLSX import/export fidelity (separate subsystem).
- Large-range interactions between recalculation and dynamic arrays.
- Any function outside the five probed areas (financial, statistical, text…).

## 8. Verdict for Gate 0

Fidelity evidence is **positive on this sample** and materially stronger than
"vendor says it's compatible". It is **not** evidence of full Excel
compatibility, and it does not remove the need for an oracle-backed corpus in
Gate 2. Recommended reading: engine direction is viable; compatibility claims
must stay scoped to corpus coverage.

## 9. Next probes

- Install a pinned LibreOffice and an Excel baseline; re-run all 84 vectors
  against a real oracle and mark provenance `confirmed`.
- Expand to 500–1,000 vectors across the remaining function families.
- Add `#SPILL!` collision and implicit-intersection vectors.
- Probe XLSX load/save round-trip separately from calculation.
