# Research Findings

> Observations only — not decisions. Decisions go to `ADRs/`.

## Index

| Area | Status | Notes |
| ---- | ------ | ----- |
| Formualizer | Researched (desk) | `research/formualizer/notes.md` |
| Mog | Researched (desk) | `research/mog/notes.md` — repo/site diverge |
| Recalc | Researched (desk) | `research/recalc/notes.md` |
| Competitive landscape | Drafted | `research/competitive-landscape.md` — two camps, matrix, candidate positions |
| Univer harness | Researched (desk) | `research/univer-harness/notes.md` — agent-native incumbent |
| Architecture comparisons | Drafted | `research/architecture-comparisons/comparison.md` |
| Benchmarks | Measured (Gate 0 spike) | `research/benchmarks/results.md` — reproducible numbers |
| Fidelity probe | Measured (Gate 0 spike) | `research/corpus-probe/results.md` — 83/84, no oracle |

## Synthesis (observations, not decisions)

- All three separate parse → value/coercion → dependency graph → engine → API.
  This layering is the consensus shape and is independently testable.
- Formualizer's "formula families" + region-level dependency tracking is the
  most concrete published answer to large-workbook recalc; its performance
  claims are vendor-reported and must be reproduced.
- Recalc's "never silently wrong" model (sentinel errors + diagnostics +
  evidence labels) is the strongest correctness idea across all three.
- Mog's stable UUID cell identity and Office.js-style command surface are
  interesting for structural edits and agent control, but the public repo and
  the marketing site describe different products; treat as unverified.
- No project guarantees full-fidelity XLSX write-back; that is a separate problem
  from calculation correctness and must be scoped explicitly.
- None should be adopted wholesale without a Gate 0 benchmark prototype on our
  own workloads.
- Measured on our laptop (i5-8257U, 8 GB): 1M copied formulas build + first
  calculate in ≈ 2.9 s (vendor: 0.26 s), broad-invalidation recalc of 1M
  dependents ≈ 250–270 ms (vendor: 48 ms). The architecture works and is fast on
  this hardware, but vendor numbers do not reproduce as stated.
- Narrow dirty sets recalc in single-digit milliseconds — quoting that as "edit
  latency" is misleading. Broad invalidation is the honest metric.
- `evaluate_cells` for one aggregate was ~1000× slower than incremental
  `evaluate_all`; its semantics need documentation before we rely on it.
- Formualizer 0.14.0 currently needs `zip` pinned to 7.2.0 to compile; newer
  transitive resolution breaks the build. Version pinning is mandatory.
- **Fidelity (84 vectors, no oracle): 83 pass.** Dates, lookups, dynamic-array
  spill, error precedence and Excel's 15-significant-digit rounding all agree.
  Only divergence: Excel's phantom 1900-02-29 (engine normalises to 1900-03-01).
- Two of our own expectations were wrong and the engine was right: `ROUND(2.675,2)`
  is 2.68 in Excel, and dynamic arrays spill via neighbouring cells rather than
  returning an array in the anchor. Authored expectations are not evidence.
- **Peer practice points the other way from pure reuse:** Quadratic and Equals
  built their own engines. Quadratic's core is no longer public; HyperFormula is
  GPL/commercial. Formualizer is the first permissive Rust option and is young
  (211 stars, no named adopters). Proposed direction: vendor a fork, not a
  floating dependency (ADR-0002, Proposed).
- **Univer already occupies the "multi-agent spreadsheet" position**: "The
  Office Harness for AI Agents", human-agent co-editing over OT, parallel agents
  in isolated worktrees with human review, Apache-2.0 OSS core. Its core is
  **TypeScript**, with a hybrid formula engine (frontend TS + server-side Rust);
  it is not Rust-first. Multi-agent capability is the category, not a
  differentiation by itself.
- Univer's commercial edges (XLSX exchange, Server SDK, pro AI extensions,
  worktree modules) and its own admission that the OSS formula engine is slower
  than the commercial one are the practical constraints on adopting it as a
  base.
- The open strategic question is now **build on Univer vs compete with it**, and
  if build-on, what the product layer adds. Our diagnostics/evidence model is a
  candidate gap in Univer's operation-level history.
- **Head-to-head at 100k (Univer OSS 1.4.0 TS engine vs Formualizer 0.14.0):**
  first calc 11.1 s vs 0.25 s; narrow edit 4.93 s vs ~2 ms; broad parameter edit
  did not complete in 10 min vs ~25 ms. Univer's Pro Rust engine is licensed and
  untested. The Rust-first performance wedge is real on headless/batch
  workloads, but does not by itself win the product.

## Open questions

- Build on a dependency vs own the engine core? (Decision after Gate 0.)
- Recalc granularity: cell, region/run, or both?
- Is an independent Excel oracle (pinned build) worth the corpus cost?
- How much Excel/Office.js API compatibility is a product requirement vs a
  control-surface convenience?
- Collaboration model: CRDT vs OT vs none in MVP?
- Why is targeted `evaluate_cells` slower than full incremental recalc — is it
  a different code path, and does it matter for UI-driven partial evaluation?
- What mixed dirty-set distribution represents our target interactive workload?
- Is Excel's pre-1900-03-01 date behaviour in scope? (Decides whether the one
  red fidelity vector is a bug or an accepted divergence.)
- What does maintaining a Formualizer fork cost over 12 months, versus building
  a scoped core? (Needed to accept or reject ADR-0002.)

## Rules

- Record what we observed, with source links and versions.
- Separate observed facts from our opinions.
- Anything that becomes a product decision moves to an ADR.
