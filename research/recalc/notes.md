# Recalc — Research Notes

> Observations only. Decisions go to `ADRs/`.
> Researched: 2026-11-13. Source: repo README (fetched).

## Identity

- Repo: `recalc-engine/recalc-engine` — https://github.com/recalc-engine/recalc-engine
- Site: https://recalc.elektraset.com
- License: Apache-2.0
- Status: very early — created 2026-07-20, ~26 commits, 0 stars at research time.
  No tagged registry packages; build from source.

## What it is

A **headless calculation engine and local verification toolkit** in Rust. Opens
`.xlsx`/`.xlsm`, rebuilds the formula dependency graph, computes values, and
reports **explicit evidence labels**. It does not render, and does not execute
VBA.

## Core philosophy

> **Never silently wrong.**

When Recalc cannot safely compute a construct it returns a distinguishable
sentinel (`#UNSUPPORTED!`, `#BLOCKED!`) and records a diagnostic. A cached-value
match is labeled as *cached evidence*, not as an independent Excel result.
Evidence categories: computed, matched stored value, matched supplied baseline,
or refused.

This is the strongest single idea in the three projects for our product: a
correctness model that makes uncertainty visible instead of emitting a
plausible-but-wrong number.

## Architecture

| Crate | Role |
| --- | --- |
| `recalc-engine` | Public Rust facade |
| `xl-io` | OOXML reader, hardened against malformed input |
| `xl-ast` | Formula lexer + parser (A1/R1C1, 3-D refs, structured refs, `LET`/`LAMBDA`) |
| `xl-value` | Value model + coercion tables (shared contract) |
| `xl-graph` | Dependency graph: dirty marking, topological recalc, cycle detection |
| `xl-fn` | Function library, one module per function |
| `xl-engine` | Orchestration: load → graph → recalc → report |
| `xl-ffi` | Thin Python/Node/WASM bindings (off by default) |
| `xl-bench` | Conformance harness (corpus + oracle supplied by the user) |

Bindings are promised as identical operation sets across Rust/Python/Node/WASM
but artifacts are not yet published.

## Notable design choices

1. **Explicit refusal over silent approximation** with sentinels + diagnostics.
2. **Evidence-scoped compatibility claims** — distinguishes "computed" from
   "matched cached value" from "matched an Excel build baseline".
3. **Value model and coercion tables as a first-class shared contract**
   (`xl-value`), separate from evaluation.
4. **Conformance harness expects the user to supply corpus + oracle.** No
   built-in Excel oracle; independent pinned-Excel evidence is a separate,
   explicitly labeled workstream.
5. **Calculation-only scope, stated bluntly** (no render/charts, no macro
   execution, no external workbook/network access).
6. **Clean-room, dependency-minimal** contributing rules.

## Explicit v1 limits (documented)

- No rendering, layout or charts.
- No VBA execution (parsed only to trace dependencies).
- No external workbooks; `WEBSERVICE`/`RTD`/`STOCKHISTORY` return `#BLOCKED!`.
- No non-en-US locale semantics; no Google Sheets mode.
- No full-fidelity write-back: computed values go into cached-value slots only.

## Strengths for us

- The **"never silently wrong" model + evidence labels** is directly applicable
  to our calculation engine and to how agents report results.
- Clean crate boundaries (io / ast / value / graph / fn / engine) are a good
  reference for our workspace layout.
- User-supplied corpus + oracle matches our plan to make `test-corpus/` the
  executable spec.
- Permissive license; clean-room posture reduces provenance risk.

## Limitations / risks

- Extremely early: 26 commits, no releases, 0 adoption. Not a dependency bet.
- Deliberately narrow: no UI, no collaboration, no write-back fidelity, no
  locale support.
- Conformance depends on a corpus/oracle **we** must supply — a large effort.
- Bus factor and longevity are unknown.
- Being "bug-for-bug Excel-compatible" is a claim that only a corpus can prove.

## Questions to resolve in Gate 0 spike

- Reuse the **philosophy** (sentinel errors + diagnostics + evidence labels),
  not necessarily the code.
- Compare `xl-value` coercion tables and `xl-graph` recalc design against our
  needs.
- Decide whether an external Excel oracle (pinned build) is worth the cost.
- Evaluate whether to vendor any crate or treat the repo as a design reference.
