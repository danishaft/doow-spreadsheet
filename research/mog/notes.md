# Mog — Research Notes

> Observations only. Decisions go to `ADRs/`.
> Researched: 2026-11-13. Source: repo README, architecture docs, project site.

## Identity

- Repo: `fundamental-research-labs/mog` — https://github.com/fundamental-research-labs/mog
- Site: https://sheetmog.ai
- License: Apache-2.0 (repo README); site FAQ says MIT. **Inconsistent — verify.**
- Version: docs/site reference 0.6.0; 1.0 "being prepared". Pre-stable.

## Important: two different descriptions of Mog

The GitHub repo README and the marketing site describe **different products**,
and this matters for evaluation:

**Repo (current):** a **headless native CLI** that opens/edits/recalculates/saves
`.xlsx`, scripted through a subset of the **Office.js Excel API**, with
JavaScript running in-binary via **QuickJS**. No UI, no browser runtime, no
Node SDK.

**Site/architecture docs:** a full spreadsheet platform: 21-crate Rust compute
core, 582 Excel functions, **Yrs CRDT** collaboration, canvas rendering, binary
wire protocol, TypeScript SDK, N-API/WASM targets.

Treat these as different snapshots/lineages. Do not assume the repo contains the
rendering/CRDT/SDK stack described on the website. This divergence is itself a
risk signal for adopting it as a dependency.

## Architecture (repo, as documented)

1. **Compute core** (`compute/core`) — formula evaluation, sparse cells, UUID
   cell identities, workbook metadata. Source of truth for authored values and
   formulas.
2. **Compute API** (`compute/api`) — Rust workbook/sheet facade for native
   callers and the scripting host.
3. **Office.js host** (`compute/officejs`) — embeds QuickJS, exposes
   `Excel.run` / `load` / `sync` on the facade.
4. **CLI** (`mog`) — open `--input`, evaluate `--file`/`--eval`, save `--output`,
   optional background sessions.

Documented storage model details:
- Compact ranges share immutable value buffers; edits stay sparse.
- `SheetStore` owns the sparse bijection between authored cell IDs and stable
  `(RowId, ColId)` pairs.
- Inserting a row/column changes order **without rekeying surviving cells**.
- Blank slots have no entry; formula/metadata anchors get identities lazily.
- Undo/redo stores typed inverses at user-action grain; replay recalculates
  dependents rather than rebuilding the document.

## Notable design choices

1. **Stable UUID cell identity** instead of positional `(row, col)` addressing.
   Makes structural edits and (claimed) collaboration robust; costs indirection
   and a mapping layer.
2. **Office.js as the control surface** — agents script workbooks with a
   familiar Excel API (`Excel.run`, ranges, formulas, tables, names) rather than
   a bespoke DSL. Notably close to how an agentic product might expose operations.
3. **Sparse, identity-based storage** with shared immutable buffers.
4. **QuickJS in-binary** so scripting needs no Node/Excel/server.
5. **Calipers verification corpus** run in CI for workbook behavior.
6. (Site only) Yrs/Yjs CRDT collaboration and canvas rendering.

## Performance claims

- No hard numbers found in the repo README. Site claims native-speed wasm and
  60fps rendering without published benchmarks we verified.
- Treat as unverified.

## Strengths for us

- The **Office.js-compatible command surface** is a strong idea for an
  agent-driven product: agents get a well-known object model and a clear
  `load`/`sync` batching boundary.
- Stable cell identity is worth studying for structural edits and collaboration.
- Sparse storage + shared immutable buffers is a proven direction.
- Verification corpus discipline (Calipers) is the right instinct.

## Limitations / risks

- **Repo/site divergence** — hard to tell what actually exists vs marketing.
- Pre-stable; README says 1.0 does **not** mean full Excel/Office.js
  compatibility; function and Office.js coverage incomplete.
- Headless-only in repo: no UI/rendering to reuse.
- Office.js subset means partial compatibility is a permanent tax.
- License ambiguity (Apache-2.0 vs MIT).
- No published benchmark numbers to reproduce.

## Questions to resolve in Gate 0 spike

- Confirm what the current repo actually contains vs the site.
- Test Office.js subset coverage against the operations our product needs.
- Measure recalc/load on our workloads; compare with Formualizer/Recalc.
- Assess whether UUID identity helps or hurts our grid/UI and persistence model.
- Clarify license and versioning before any code reuse.
