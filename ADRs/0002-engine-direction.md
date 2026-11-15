# ADR-0002: Engine direction — vendor a Formualizer fork

- **Status:** Proposed — requires human acceptance at the Gate 0 review (M0.4)
- **Date:** 2026-11-13
- **Deciders:** project owners (team of five) — named gate owner TBD
- **Related:** `ADRs/0001-development-process.md`, `research/benchmarks/results.md`,
  `research/corpus-probe/results.md`, `research/architecture-comparisons/comparison.md`,
  `research/architecture-comparisons/diagnostics-model.md`

## Context

Gate 0 exists to prove the engine approach before product documentation becomes
a priority. The evidence now available:

- **Performance** (our hardware, i5-8257U, 8 GB): 1M copied formulas build plus
  first calculate in ~2.9 s; broad invalidation of 1M dependents recalculates in
  ~250–270 ms; peak RSS ~580 MB. Vendor claims (0.26 s / 48 ms) did not
  reproduce as stated, but the architecture is usable at this scale.
- **Fidelity** (84 vectors, no oracle): 83 pass. Dates, lookups, dynamic-array
  spill, error precedence and floating-point rounding agree with documented
  Excel behaviour. One known divergence: Excel's 1900 leap-year quirk.
- **Integration:** `formualizer-workbook` 0.14.0 fails to compile with its own
  declared `zip = "7.2"` once 7.3/7.4 resolve; pinning 7.2.0 fixes it. This is
  current, real churn.
- **Dependency landscape:** Quadratic's Rust core is no longer public (repo
  404); Equals' engine is in-house and closed; HyperFormula is GPLv3 or paid
  commercial; Recalc and other Rust engines are pre-release. Formualizer is
  Apache-2.0, ~211 stars, first released 2025-03, no named production adopters.
- **Peer practice:** the verifiable new-generation spreadsheet products
  (Quadratic, Equals) built their own engines — the engine was their product.
- **Team:** five people. Building a 400+ function, Excel-compatible engine from
  zero is a multi-year commitment before the agent layer ships.

## Proposed decision

1. **Start from Formualizer 0.14.0, vendored as an in-repository fork** under
   Apache-2.0 — not as a floating crates.io dependency, and not as a clean-room
   build.
2. **Isolate it behind our own `CalculationEngine` trait.** No upstream types
   (`LiteralValue`, `ExcelError`, `Workbook`) appear in product APIs. The
   diagnostics/evidence model in `research/architecture-comparisons/diagnostics-model.md`
   is the wrapper contract.
3. **Recalc granularity: region/formula-family tracking is primary; keep a
   cell-level path** for irregular formulas and as a correctness oracle. Do not
   treat targeted `evaluate_cells` as the cheap path — it measured ~1000× slower
   than incremental `evaluate_all` at 1M.
4. **Pin exact versions and commit lockfiles** (including `zip = 7.2.0` for now).
   Run `test-corpus/` on every engine upgrade; a red corpus blocks the upgrade.
5. **Scope compatibility claims to corpus coverage.** "Excel compatible" means
   "passes these vectors", with evidence labels, not "matches Excel".

This ADR does **not** decide the oracle, Office.js compatibility, or
collaboration model. Those are separate decisions (Gate 1–3).

## Alternatives considered

- **Depend on upstream Formualizer as-is.** Rejected: the `zip` break shows a
  transitive minor bump can break the build; no control over fixes; roadmap and
  longevity risk for the core of the product.
- **Build a clean-room engine core.** Rejected for now, not forever. It is the
  choice Quadratic and Equals made, and it is the right choice if calculation is
  our moat. It is wrong for a five-person team before the agent layer exists.
  Revisit if Gate 1 declares the engine strategic.
- **Use Quadratic's core.** Unavailable: the repository is no longer public.
- **Use HyperFormula.** Rejected: GPLv3 or paid commercial license, JS/runtime
  constraints, and it would still need integration work.
- **Depend on Recalc.** Rejected as a dependency (26 commits, no releases);
  its "never silently wrong" model is adopted as a design principle instead.
- **Depend on Mog.** Rejected: public repo and marketing site describe different
  products; unverifiable.

## Consequences

- **We own fork maintenance.** Upstream tracking, merge cost and divergence are
  real work. Mitigation: contribute fixes upstream where possible; keep the fork
  close to upstream; document every local patch.
- **License obligations** for Apache-2.0 (NOTICE/attribution) apply to the fork.
- **The `zip` pin is our patch** until upstream corrects `data_start()` handling.
- **The wrapper boundary and corpus become mandatory infrastructure**, not
  optional.
- **Revisit triggers:** expanded fidelity probe finds silent-wrong behaviour in
  commonly used functions; performance regresses below product needs; upstream
  changes license or model; or Gate 1 establishes calculation as the moat.
  In those cases this ADR is superseded by a build-or-fork-deeper decision.

## Approval

- [ ] Gate owner named (human)
- [ ] Benchmark and fidelity evidence reviewed
- [ ] Fork vs dependency decision accepted
- [ ] Revisit triggers accepted
