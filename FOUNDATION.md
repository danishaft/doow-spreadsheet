# FOUNDATION — Vision, Principles, Scope

> Status: **Draft v0 — not approved.** Gate 1 artifact.
> Basis: direction set 2026-11-13/14 — compete with the new-age spreadsheets
> (Quadratic, Equals, Rows, Univer) by doing what they do, better.
> It is **both an app and an SDK**, free for people and developers to use,
> embed and share.
> Owner: TBD · Approved by: TBD · Last updated: 2026-11-14
>
> Items marked **A1–A5** are assumptions I made to unblock drafting. Correct
> them; nothing here is approved.

## 1. Vision

A spreadsheet **app and SDK** where a team of agents does the work and a human
can verify every number. Rust-fast calculation, Excel-faithful results,
real-time collaboration — free for people and developers to use, embed and
self-host.

## 2. Problem

- **Incumbent spreadsheets (Excel, Sheets):** one assistant, one conversation.
  They are not built for several agents working a workbook, and their engines
  are closed and slow at scale.
- **AI-native apps (Quadratic, Equals, Rows):** strong AI and data connections,
  but closed. Trust is post-hoc lineage ("where did this come from"), not
  computation evidence. Equals is priced for enterprises; Quadratic is
  source-available at best.
- **Agent harnesses (Univer):** the closest to our vision, but the open core is
  TypeScript-centric, the OSS formula engine is admitted to be slower than their
  commercial one, and XLSX/Server/AI modules are paid. Agent changes are isolated
  drafts merged by humans, not verified computation.
- **No one combines** speed, Excel fidelity, multi-agent execution, verifiable
  results and open self-hosting. That combination is the product.

## 3. Target users

**A1 — Primary:** finance, operations and analytics teams doing multi-step work
where a wrong number is costly and auditable results matter.

**A2 — Primary, equal weight:** developers and product teams embedding an
agentic spreadsheet into their own products. The SDK is a first-class product,
not a by-product of the app.

Excluded for now: casual personal use, docs/slides suites, BI dashboard
replacement.

## 4. Differentiation — three bets, everything else at parity

**B1. Engine.** A **Rust-first core end to end** — engine, harness and headless
runtime — not a Rust formula component bolted onto a TypeScript stack. Fast at
1M+ formulas, Excel-faithful, proven by an executable corpus. Speed is measured
head-to-head, not claimed.

**B2. Multi-agent harness.** Named agent roles, isolated drafts, review/merge,
live co-editing with per-agent attribution. Agents are first-class clients, not
a chatbot sidebar — and the harness lives in the same Rust core, usable from the
app and the SDK.

**B3. Verifiable computation.** Every value carries evidence: `computed`,
`imported_cached`, `refused_unsupported`, `mismatch`. Every agent run is
replayable (deterministic clock/seed), every change attributable and reversible.
We refuse rather than guess.

Supporting: open core, self-host, no watermark; open XLSX interchange.

**B4. Free distribution.** The app and the SDK are free for people and
developers to use and share. Univer charges for XLSX, Server SDK and pro AI
modules; Quadratic is closed; Rows gates AI tasks. We compete on access: one
core, exposed two ways, free at the point of use.

## 5. Product principles

1. Correctness before features.
2. Every agent action is attributable, reviewable and reversible.
3. Evidence over claims; `test-corpus/` is the executable spec.
4. Measure performance; "Rust" is not a performance claim.
5. Open by default; no commercial trap on core interchange.
6. One beachhead, then breadth.

## 6. Scope

**MVP in:** grid and core formulas; XLSX read + CSV import (**A3**); multi-agent
drafts and review; real-time collaboration with attribution; audit/replay;
self-hostable core.

**SDK in:** headless runtime, embeddable grid, typed API, WASM/npm packages and
Rust crates, examples and docs. Same core as the app — no separate engine.

**Out:** docs/slides/BI, VBA/macros, plugin marketplace, native mobile, full
connector catalogue.

**Deferred:** XLSX write-back fidelity, data connectors, billing/plans,
marketplace.

**A4** — concurrency starts with isolated agent drafts + human merge; live
multi-agent editing on the same cells is a later differentiator.

**A5** — connectors are post-MVP; the first wedge is calculation + agents, not
data integration.

**A6** — licensing: free for people and developers to use and share;
open-source vs source-available is not yet decided. No paid tiers at launch;
monetisation is deferred (managed hosting/support later, not a launch feature).

## 7. Success metrics (proposed targets)

- 1M-formula first calculation under 1.5 s on reference hardware; p95 recalc of
  a broad 1M-cell invalidation under 150 ms.
- ≥99% fidelity on a 1,000-vector corpus covering the target function set.
- 100% of agent-produced values carry an evidence label; 100% of agent changes
  attributable and replayable.
- First successful agent task within 10 minutes of opening a workbook.
- SDK: package install to an embedded working sheet in under 15 minutes; weekly
  embeds, downloads and docs-to-first-API-call completion.
- Weekly active teams, retained agent tasks per team.

## 8. Terminology

- **Agent** — a scoped actor with identity, permissions and a role.
- **Core** — the shared Rust engine + agent harness used by both the app and the
  SDK.
- **SDK** — the embeddable, headless form of the core: typed API, WASM/npm
  packages and Rust crates, usable without our UI.
- **Draft** — an isolated set of agent changes before merge.
- **Evidence label** — how a value was obtained (`computed`, `imported_cached`,
  `refused_unsupported`, `mismatch`).
- **Replay** — deterministic re-execution of an agent run against the same
  inputs and clock/seed.
- **Corpus** — executable correctness vectors in `test-corpus/`.

## Approval

- [ ] Problem and users agreed
- [ ] Differentiation agreed
- [ ] MVP boundary agreed
- [ ] Success metrics agreed
- [ ] App + SDK, free distribution confirmed
- [ ] Assumptions A1–A6 confirmed or corrected
