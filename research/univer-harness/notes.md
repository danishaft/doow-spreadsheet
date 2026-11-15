# Univer — Research Notes

> Observations only. Decisions go to `ADRs/`.
> Researched: 2026-11-13. Sources: univer.ai homepage, docs.univer.ai overview,
> `dream-num/univer` README. Secondary: search summaries (marked).

## Identity

- Repo: `dream-num/univer` · site: https://univer.ai · docs: https://docs.univer.ai
- License: **Apache-2.0 for the open-source packages**. Commercial Web SDK
  plugins, Server SDK, and commercial AI SDK extensions are licensed separately.
  Free evaluation tier has watermarks and feature limits.
- Positioning: **"The Office Harness for AI Agents"** — spreadsheets, docs,
  slides, canvas, relational tables. Not just a spreadsheet component.
- Scale signal: ~30–39k GitHub stars including Luckysheet heritage (secondary
  source, not independently verified here).

## Stack reality (corrected 2026-11-14)

- The repository is a **TypeScript monorepo** (Turbo, TypeScript checks). The
  content model, plugin/command architecture and Canvas rendering/layout are
  TypeScript.
- The formula engine is **hybrid**: frontend calculation plus a **server-side
  Rust** engine. It is not a Rust-first core.
- Luckysheet lineage (same team, dream-num) — older architecture behind the new
  agent-harness positioning. (Secondary source.)
- **Implication:** Univer is closest to us on **harness and distribution**
  (SDK + app + agent positioning), not on engine architecture. Quadratic is the
  Rust-native benchmark; Univer is the SDK/harness benchmark.

## What it does for agents (verified from univer.ai / docs)

- **Human–agent co-editing** on the same file: "People and agents edit the same
  content live." OT resolves concurrent edits; offline edits sync on reconnect.
- **Operation-level history**: "Trace each change, see who made it, and restore a
  chosen version."
- **Parallel agents via worktrees**: each task gets an isolated draft; main
  content stays untouched; humans compare, preview and merge (or reopen).
- **Agent loop**: "Agents call Univer's Office tools through an API or CLI and
  get results and checks back" — read → edit + calculate → inspect → revise.
- **Headless**: calculation runs in Node without the editor; Rust formula
  engine. Façade APIs (`FWorkbook`, `FWorksheet`, `FRange`) shared between
  browser and headless.
- **Server SDK** supplies collaboration and file conversion; the backend supplies
  identity, permissions and storage. History, comments and Worktree are optional
  modules.

## Architecture (primary docs + secondary summary)

- Plugin-based: UI plugins separated from feature/data plugins; 100+ plugins
  across core → product → feature layers (secondary).
- Command / mutation / operation separation powers undo/redo, macros and
  collaboration (secondary; the overview page only confirms plugins + Façade).
- Collaboration is **OT**, not CRDT.
- XLSX import/export is **not** in the OSS version: snapshot persistence is
  separate from file conversion, which uses separately distributed packages.

## Formula engine

- Rust, headless, claimed 500+ formulas and up to 10M cells (marketing claim;
  not verified by us).
- Their own FAQ states the OSS formula engine **"is slower than the commercial
  one"** — so the open-source engine is not the performance ceiling.

## Why this matters to us

1. **The category already has a named incumbent.** "Multi-agentic by nature" is
   Univer's positioning, not a differentiator on its own. A product must beat or
   complement it on something specific.
2. **The worktree model is not concurrent agent co-editing.** Agents work in
   isolated drafts; humans merge. If our bet is several agents editing the same
   live workbook with conflict resolution, that is a different (harder) claim
   than Univer makes.
3. **It is a candidate base, not just a competitor.** Apache-2.0 core, headless
   Rust engine, Façade API, agent tooling. Building this stack from Formualizer
   plus our own collaboration layer would be re-creating a large, adopted
   project.
4. **Commercial edges matter.** XLSX exchange, Server SDK and pro AI extensions
   are paid. If XLSX interchange is core to the product, budget the license or
   build conversion.
5. **Our evidence model may be a genuine gap.** Univer tracks who changed what
   and version history. It does not appear to carry per-value computation
   evidence (`computed` / `cached` / `refused` / `mismatch`) like Recalc's model
   or our diagnostics note. That is a plausible trust-layer differentiator for
   agent work.

## Gate 0 head-to-head (OSS, 100k)

Measured 2026-11-14, Node 26.13.0, Univer OSS 1.4.0, same host as our spike.
Full detail: `research/benchmarks/univer-oss-results.md`.

- First calculation of 100k copied formulas: **11.1 s** (Formualizer: ~0.25 s).
- Narrow edit: **4.93 s** (Formualizer: ~2 ms).
- Broad parameter edit over 100k dependents: **did not complete in ~10 min**;
  killed (Formualizer: ~25 ms).
- All completed results were correct.
- The OSS engine is TypeScript, not the Pro Rust engine; the Pro engine is
  licensed separately and was not tested.

## Open questions

- Actual pricing and terms for Server SDK, Worktree, AI extensions, XLSX exchange.
- Is the Rust formula engine usable in the OSS headless packages, and how does it
  perform on our 1M-formula workload?
- Fidelity: run our 84-vector corpus against Univer's engine.
- Does the OSS core include collaboration/OT, or only the Server SDK?
- Concurrent-agent semantics: worktrees only, or is live multi-agent merge
  supported?
- Can we integrate our own calculation engine behind their Façade if needed?
- Lock-in: how coupled is product data to Univer snapshots/plugins?
