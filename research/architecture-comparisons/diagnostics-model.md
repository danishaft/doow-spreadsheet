# Diagnostics and Evidence Model — adapting Recalc's "never silently wrong"

> Design note, not a decision. Proposed for `SYSTEM_DESIGN.md` §4 and Gate 3.
> Source observation: `research/recalc/notes.md`. No Recalc code is used.

## 1. Why this matters

Our benchmarks and fidelity probe measure an engine we do not control. The
failure mode that matters is not a crash — it is a plausible number that is
wrong: an unsupported function silently mapped to `0`, a cached value presented
as computed, an approximation that looks like a result. Recalc's strongest idea
is to make every result carry **how it was obtained**, and to refuse rather than
guess.

## 2. Proposed result contract

Every evaluated cell returns:

```
CellResult {
  value:      Value,            // displayed value, may be an error sentinel
  evidence:   Evidence,         // how we know it
  diagnostics: Vec<Diagnostic>, // why, when it is not a clean computation
  engine:     EngineStamp,      // engine version + corpus id
}
```

Evidence labels (closed set, no free text):

| Label | Meaning |
| ----- | ------- |
| `computed` | Evaluated by our engine in this session |
| `imported_cached` | Value came from the file's cached slot, not recalculated |
| `oracle_matched` | Computed value matched a pinned oracle for this vector |
| `refused_unsupported` | Construct or function not implemented — no value claimed |
| `refused_blocked` | Construct disallowed by policy (external calls, macros) |
| `excel_error` | Excel error produced by evaluation (`#DIV/0!`, `#N/A`, …) |
| `mismatch` | Computed value disagrees with the pinned oracle |

Rules:

1. No value may be emitted without exactly one evidence label.
2. A refusal is never converted into a plausible value — not `0`, not empty,
   not a nearby approximation.
3. `imported_cached` is never presented as `computed`. Opening a file is not
   evidence that we can calculate it.
4. `mismatch` is a first-class outcome, not a test failure to be hidden.

## 3. Mapping Formualizer's outputs

| Engine surface | Our classification |
| -------------- | ------------------ |
| Normal value | `computed` |
| `#DIV/0!`, `#N/A`, `#VALUE!`, `#NUM!`, `#REF!`, `#NAME?`, `#NULL!` | `excel_error` (with diagnostic; `#NAME?` only when Excel semantics actually call for it) |
| `#N/IMPL!`, unknown-function errors | `refused_unsupported` — **never** relabel as a wrong-but-plausible value |
| `#CIRC!` | `refused_unsupported` + cycle diagnostic (cells in the cycle) |
| `#SPILL!`, `#CALC!` | `excel_error` + spill/array diagnostic |
| Cell with no evaluation attempted | `imported_cached` or refusal — never `computed` |

The wrapper is the only place that knows the upstream engine. Product code sees
`CellResult`, not `LiteralValue` or `ExcelError`.

## 4. Agent-facing contract

The agent layer must receive evidence labels with every value, and tool results
should include them by default:

- An agent may say "the workbook computes X" only for `computed` or
  `oracle_matched`.
- An agent may say "the file contains X" for `imported_cached`.
- An agent must surface a refusal as a refusal, with the diagnostic.
- Mutating actions return the recomputed `CellResult` for affected cells.

This directly serves the product goal of auditable agent behaviour: a run can be
reviewed by evidence label, not by trusting prose.

## 5. CI and corpus reporting

The corpus runner reports four counts per run: `computed` matches,
`oracle_matched`, `refused`, and `mismatch`. "Green" means no `mismatch` and no
unexplained refusal. Cached matches do not count as passing calculation.

## 6. What we deliberately do not copy

- Recalc's exact sentinel spellings (`#UNSUPPORTED!`, `#BLOCKED!`). Our
  user-facing representation is a PRD/UX decision; internally we use the labels
  above.
- Its crate layout or code. Only the philosophy and the evidence vocabulary.
- Its narrow scope (no UI, no collaboration, no write-back) as a product limit.

## 7. Open questions

- User-facing display: show an Excel-like error plus diagnostics, or a distinct
  sentinel? (PRD, Gate 1.)
- Diagnostics storage and cost at 1M cells: per-cell or per-range aggregation?
- Does an unsupported function in an unrelated cell block a sheet-level
  "computed" claim? (Proposed: yes for that sheet's aggregate result.)
- How do we version and stamp the oracle/corpus used for `oracle_matched`?
