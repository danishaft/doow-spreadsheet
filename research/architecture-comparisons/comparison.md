# Architecture Comparison — Formualizer vs Mog vs Recalc

> Observations only. Decisions go to `ADRs/`.
> Researched: 2026-11-13.

## At a glance

| Dimension | Formualizer | Mog | Recalc |
| --- | --- | --- | --- |
| License | MIT/Apache-2.0 | Apache-2.0 repo / MIT site (verify) | Apache-2.0 |
| Maturity | 0.14.0, 38 releases, churning | Pre-stable (0.6.0 docs); repo vs site diverge | ~26 commits, no release |
| Scope | Engine + workbook + typed I/O + agent layer | Headless CLI via Office.js subset (repo) | Calc + verification toolkit |
| Language targets | Rust/Python/WASM | Rust core + QuickJS host (repo) | Rust (+ planned bindings) |
| Formula coverage | 400+ | 582 (site) / incomplete (repo) | Function modules, corpus-driven |
| Storage | Apache Arrow columnar, formula families | Sparse cells, stable UUID identity | Value model + graph |
| Recalc | Incremental, region deps, cycles, parallel | Documented dependency graph | Dirty marking, topo, cycles |
| Determinism | Injectable clock/timezone/seed | Not documented | Evidence-labeled results |
| UI/rendering | None | None in repo (site claims canvas) | None (by design) |
| Collaboration | None | Site claims Yrs CRDT; not in repo | None |
| Failure model | Excel errors; claims exact semantics | Excel errors; partial coverage | **Sentinels + diagnostics, never silently wrong** |
| Verification | Cell-by-cell oracle + benchmarks | Calipers corpus | User-supplied corpus + oracle |

## Patterns worth adopting

1. **Layered crates**: parse → value/coercion → graph → engine → workbook → API.
   Keep each independently testable and benchmarkable.
2. **Region/run-level dependency tracking** (Formualizer formula families).
   Tracking 100k individual formula nodes is the naive design we should avoid.
3. **Value model + coercion tables as a shared contract** (Recalc `xl-value`).
   Error propagation and coercion semantics belong in one place, tested once.
4. **Explicit refusal + diagnostics** (Recalc). Every unsupported construct must
   yield a distinguishable error plus a diagnostic, never a plausible guess.
5. **Evidence labels** (Recalc). Distinguish computed, cached, baseline-matched
   and refused results in verification reports.
6. **Executable corpus as the spec** (all three). A cell-by-cell reference
   implementation can serve as an oracle for fast paths.
7. **Determinism controls** (Formualizer). Injectable clock/timezone/seed make
   NOW/TODAY/RAND reproducible for tests and agents.
8. **Stable cell identity** (Mog) is worth evaluating for structural edits and
   collaboration, but it adds indirection to a positional grid/UI.
9. **Office.js-style command surface** (Mog) is a candidate way to expose
   workbook operations to agents: familiar object model with explicit
   `load`/`sync` batching.

## Patterns to treat cautiously

- Vendor benchmark claims (Formualizer, Mog) — reproduce on our hardware.
- "Bug-for-bug Excel compatible" — only a corpus can prove it.
- Marketing/repo divergence (Mog) — verify what is actually shippable.
- UUID identity (Mog) — attractive but a deep model commitment; test against our
  grid, persistence and collaboration needs before adopting.
- Full write-back fidelity — none of the three guarantees it; it is a separate
  problem from calculation.

## Open questions for Gate 0

- Do we build on a dependency or own the engine core? (See ADR needed after
  spike; no decision yet.)
- Which recalc granularity do we need: cell, region/run, or both?
- Do we require an independent Excel oracle, and if so which pinned build?
- How much Office.js/Excel API compatibility is a product requirement vs a
  control-surface convenience?
- What is our collaboration model (CRDT vs OT vs none in MVP)?
