# IMPLEMENTATION PLAN — Milestones and Tasks

> Status: **Living document.** Not a gate artifact.
> Owner: TBD · Last updated: 2026-11-13

## 1. Milestones

| Milestone | Goal | Exit criteria | Depends on |
| --------- | ---- | ------------- | ---------- |
| M0        | Gate 0 — feasibility spike | Research notes + benchmark prototype | — |
| M0.1      | Desk research: Formualizer, Mog, Recalc | Notes recorded | — |
| M0.2      | Benchmark prototype on our workloads | Reproducible numbers on our hardware | M0.1 |
| M0.3      | Corpus probe: tricky formula semantics | Fidelity findings recorded | M0.1 |
| M0.4      | Gate 0 review | Human approval + ADR on engine direction | M0.2, M0.3 |

## 2. Epics and tickets

Each task is bounded, references the spec it implements, and states its test.

| ID   | Task | Spec | Tests | Est. | Status |
| ---- | ---- | ---- | ----- | ---- | ------ |
| M0.1a | Desk research — Formualizer | — | sources cited | — | Done |
| M0.1b | Desk research — Mog | — | sources cited | — | Done |
| M0.1c | Desk research — Recalc | — | sources cited | — | Done |
| M0.1d | Architecture comparison | — | — | — | Done |
| M0.2a | Reproduce Formualizer million-formula / 100k-copied-formula benchmarks | — | benchmark harness | — | Done — not reproduced; see `research/benchmarks/results.md` |
| M0.2b | Measure incremental recalc latency on realistic edits | — | benchmark harness | — | Done — broad invalidation ~250–270 ms at 1M |
| M0.3a | Corpus probe: date/time, lookups, dynamic arrays, error precedence, FP summation | — | `test-corpus/` | — | Done — 83/84 on 84 vectors |
| M0.3b | Evaluate Recalc sentinel/diagnostics model for our engine | — | design note | — | Done — `research/architecture-comparisons/diagnostics-model.md` |
| M0.3c | Verify corpus expectations against a pinned LibreOffice/Excel oracle | — | oracle run, provenance labels | — | Todo |
| M0.4  | Gate 0 review + ADR-0002 engine direction | — | — | — | Todo — ADR-0002 drafted (Proposed) |
| M0.5  | Competitive evaluation: Univer head-to-head, SDK/API, closed-app matrix | — | benchmark + matrix | — | In progress — Univer OSS engine measured (`research/benchmarks/univer-oss-results.md`); fidelity, SDK and app matrix Todo |

> All Gate 0 code lives in throwaway spikes under `research/*/spike/`, not in the
> product workspace.

## 3. Dependencies

## 4. Risks

| Risk | Impact | Mitigation |
| ---- | ------ | ---------- |
| Vendor benchmarks don't reproduce | Wrong engine choice | Reproduce on our hardware before Gate 0 approval |
| Upstream churn (Formualizer breaking releases) | Integration cost | Pin versions; isolate behind our own trait boundary |
| Mog repo/site divergence | Adopting vaporware | Verify current repo contents before reuse |
| Recalc too early to depend on | Dead dependency | Reuse ideas, not code, unless proven |
| No full-fidelity XLSX write-back anywhere | Product gap | Scope explicitly; treat as separate subsystem |
| Corpus/oracle effort underestimated | Weak correctness signal | Budget for it in M0.3 and TEST_STRATEGY |

## 5. Status log

Append-only, one line per notable change.

- 2026-11-13 — M0.1 desk research completed for Formualizer, Mog, Recalc; comparison drafted.
- 2026-11-13 — M0.2 benchmark spike completed. Rust toolchain installed; Formualizer 0.14.0 measured on host hardware. First calc of 1M formulas ~2.9 s, broad invalidation ~250–270 ms, peak RSS ~580 MB. Vendor figures not reproduced. Transitive `zip` resolution breaks the build (pinned 7.2.0). See `research/benchmarks/results.md`.
- 2026-11-13 — M0.3a fidelity probe completed: 84 vectors, 83 pass; only Excel's 1900 leap-day quirk diverges. Dynamic-array spill and 15-significant-digit rounding confirmed. Two of our own expectations were corrected — no oracle yet. See `research/corpus-probe/results.md`.
- 2026-11-13 — M0.3b diagnostics/evidence model design note written (adapting Recalc's "never silently wrong").
- 2026-11-13 — ADR-0002 (engine direction) drafted as **Proposed**: vendor a Formualizer fork behind our own trait boundary. Awaiting human Gate 0 review.
- 2026-11-14 — M0.5 started. Univer OSS 1.4.0 head-to-head at 100k: first calc 11.1 s, narrow edit 4.93 s, broad parameter edit did not complete in ~10 min; Formualizer 0.25 s / ~2 ms / ~25 ms. Univer Pro Rust engine untested. See `research/benchmarks/univer-oss-results.md`.
- 2026-11-14 — Product direction set: **app + SDK, free for people and developers**, competing with the new-age spreadsheets. FOUNDATION v0 drafted (unapproved). Univer confirmed TypeScript core + hybrid Rust formula engine, not Rust-first.

