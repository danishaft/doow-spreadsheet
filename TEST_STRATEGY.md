# TEST STRATEGY — Verification Standards

> Status: **Draft — not approved.** Gate 2/3 artifact.
> Owner: TBD · Approved by: TBD · Last updated: —

## 1. Principle

Tests are the specification. The behavior corpus defines calculation
correctness; prose defines UX and intent.

## 2. Test layers

| Layer       | What it covers | Tooling | Gate |
| ----------- | -------------- | ------- | ---- |
| Unit        | …              | Rust    | —    |
| Corpus      | Calculation correctness | `test-corpus/` runner | — |
| Integration | Boundaries     | —       | —    |
| Property    | Invariants     | —       | —    |
| Performance | Recalc, load   | Criterion + thresholds | —    |
| End-to-end  | User journeys  | —       | —    |

## 3. Corpus policy

Format, sources (Excel / LibreOffice / HyperFormula), how vectors are added,
how discrepancies are recorded.

## 4. CI gates

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --workspace`
- corpus runner
- benchmark regression

## 5. Performance

Targets, measurement method, regression thresholds, hardware assumptions.

## 6. Review rules

Critical changes require a human reviewer. Enumerate which subsystems are
critical.

## 7. Evidence policy

An agent's claim that something works is not evidence. Commands, logs and
benchmark output are evidence.
