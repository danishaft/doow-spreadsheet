# Competitive Landscape — AI-native spreadsheets and agent harnesses (2026)

> Observations only. Decisions go to `ADRs/`. Pricing/claims are vendor or
> third-party reported and may change; verify before relying on them.
> Initial landscape: 2026-11-13. SDK research roster updated: 2026-11-17.

## Current project direction

The user clarified that **this project is the spreadsheet SDK**: an embeddable
human editor and headless runtime, with Rust calculation, a WebGL graphics
direction, and a built-in harness for parallel agent work. Developers bring
their models, context and application logic. A future Doow application is a
customer reference, not a deliverable of this project.

This records the user's direction. It does not select dependencies, approve
architecture, settle release scope, or approve the existing product drafts.
FOUNDATION/PRD reconciliation is a separate task.

## SDK research questions and provisional priorities

The roster is an inventory of references, not a plan to investigate every
project exhaustively. Give each reference a specific question. Reduce research
depth first; remove candidates only after examining their distinctive behavior.
Category overlap, implementation language and project age do not establish
architectural inferiority.

| Question | References | Current position |
| -------- | ---------- | ---------------- |
| What must a competitive spreadsheet SDK expose? | SpreadJS | Established capability/API baseline; our comparative inspection remains pending. |
| How should commands, agent workspaces and review fit together? | Univer; targeted Mog inspection | Strong relevance. Verify implementation and open-source/commercial boundaries. |
| What calculation behavior must we reproduce? | Excel; GRID benchmark; targeted Recalc inspection | Behavioral and testing references. GRID's published score measures formula-result agreement, not speed or full workbook fidelity. |
| Which Rust calculation techniques fit our workloads? | Formualizer, IronCalc, Titan | Comparison remains open. Formualizer has local measurements; that does not prove superiority over the alternatives. |
| Which editor and embedding architecture fits the SDK? | OpenCalc, GridCraft, ZedSheet; Quadratic for graphics/interaction | Comparison remains open. A bounded source examination must precede ranking the inspectable implementations. Quadratic's restricted core limits code-level comparison. |
| How should structured data, external work and result inspection behave? | Grist, Rows, Equals, ACE, Werkbook | Targeted references when the corresponding capability is being designed; this research priority does not exclude product capabilities. |

The earlier proposed five-project shortlist was premature. The first three
questions have clear reference roles; the engine and editor pools remain open.

## Project roster and evidence status

"README inspected" means documented claims were read, not that source behavior,
benchmarks, package availability or production readiness were verified.

| Reference | Research purpose | Evidence currently available |
| --------- | ---------------- | ---------------------------- |
| [SpreadJS](https://developer.mescius.com/spreadjs) | Embedding, APIs, customization, editing, formulas, charts, interchange and developer experience. | Baseline identified; comparative API/behavior/performance inspection pending. |
| [Univer](https://github.com/dream-num/univer) | Plugins, commands, visual/headless APIs, collaboration, agent drafts, history, review and merge. | [Existing research](univer-harness/notes.md), including a scoped OSS calculation benchmark; commercial engine and full harness not benchmarked. |
| [Quadratic](https://www.quadratichq.com/technology) | Rust/Wasm and WebGL architecture, grid interaction, code/data computation. | Public technology and product documentation inspected; core implementation not evaluated. |
| [Rows](https://rows.com) | Connectors, asynchronous spreadsheet operations, refresh and execution state. | Earlier desk research; internal architecture and numerical claims require verification. |
| [GRID](https://grid.is) | Embedded calculation capability, formula edge cases and compatibility methodology. | [Published benchmark repository](https://github.com/GRID-is/spreadsheet-benchmark) and [analysis](https://grid.is/blog/grid-spreadsheet-benchmark) inspected; not independently rerun here. |
| [Grist](https://github.com/gristlabs/grist-core) | Record identity, relationships, column formulas, permissions, SQLite documents and Python execution. | Official README inspected; source architecture and behavior not tested here. |
| [Equals](https://equals.com) | Formula/query/source lineage, result inspection and refresh semantics. | Current product page inspected; internal calculation architecture not established. |
| [Recalc](https://github.com/recalc-engine/recalc-engine) | Value/coercion contracts, diagnostics, unsupported evaluation, computed/cache/baseline evidence. | [Existing research](recalc/notes.md); no local comparative implementation validation. |
| [SheetMog / Mog](https://github.com/fundamental-research-labs/mog) | Office.js-style operations, batching, embedded scripting, sparse storage and stable identity. | [Existing research](mog/notes.md) records repository/site divergence; verify the exact version and supported surface. |
| [Formualizer](https://github.com/PSU3D0/formualizer) | Columnar storage, formula families, region dependencies, incremental evaluation, deterministic context and workbook APIs. | [Research](formualizer/notes.md), [local measurements](benchmarks/results.md) and [fidelity probe](corpus-probe/results.md); the probe lacks an independent pinned oracle. |
| [OpenCalc](https://github.com/CasualOffice/opencalc) | Rust/Wasm engine, embeddable editor, shared editor/MCP operations, OT, determinism and document preservation. | README inspected on 2026-11-17; it declares alpha status. Source, package and behavior validation pending. |
| [ZedSheet](https://github.com/xiujunma/zedsheet) | Rust/Wasm canvas/DOM editor, geometry, interaction, document features and npm/JS embedding. | README inspected on 2026-11-17; describes an x-spreadsheet port. Lineage alone does not establish inferior architecture. |
| [GridCraft](https://github.com/storytold/gridcraft) | Rust document/calculation/editor implementation, rendering, snapshots, commands, XLSX, CLI and MCP. | README and published GRID compatibility results inspected; source architecture and local behavior/performance comparison pending. |
| [IronCalc](https://github.com/ironcalc/IronCalc) | Rust calculation, reference/value semantics, XLSX and language bindings. | README and published GRID compatibility results inspected; remains an engine countercandidate, not a proven inferior option. |
| [AngelList ACE](https://www.angellist.com/blog/teaching-a-spreadsheet-engine-to-teach-itself) | Versioned workbook templates, named connections, recalculation triggers, extracted outputs and API orchestration. | Official engineering article inspected on 2026-11-17. ACE is the platform around Werkbook; reported production/performance figures are vendor claims. |
| [Werkbook](https://github.com/jpoz/werkbook) | Go headless library, XLSX processing/calculation, structured CLI and real-world compatibility feedback. | jpoz/werkbook README and AngelList article inspected on 2026-11-17. A separate werkbook/werkbook repository also exists; pin the intended repository/version before comparison. |
| [Titan Engine](https://github.com/podraven/titan-engine) | Chunked struct-of-arrays storage, bytecode, lazy ranges, string interning, batching, change events and Wasm typed-array views. | README inspected on 2026-11-17; browser-performance emphasis and compatibility tradeoffs are documented, not locally measured. |
| Excel / Google Sheets | Excel for declared compatibility behavior; Sheets for browser interaction and collaboration reference behavior. | An independent pinned Excel oracle remains needed for our calculation corpus. Sheets is a separate behavioral reference, not interchangeable with Excel. |
| [HyperFormula](https://github.com/handsontable/hyperformula) | Headless JavaScript API, dependencies, value semantics and comparative engine measurements. | Official README inspected; GPLv3/commercial licensing and intended usage require evaluation before reuse. |

## How to narrow the open pools

For Formualizer/IronCalc/Titan and OpenCalc/GridCraft/ZedSheet, perform a bounded
source examination before assigning a primary implementation reference:

1. Pin the repository version and identify what is implemented, packaged and
   supported, distinguishing documentation from code and verified behavior.
2. Inspect data ownership/layout, reference transformations, evaluation and
   invalidation, rendering/interaction boundaries, runtime bindings and agent
   capabilities as applicable.
3. Compare meaningful tradeoffs on the same question; a vectorized engine and a
   bytecode engine may optimize different workloads.
4. Run matched correctness/performance probes where they decide the choice.
   Formula coverage, pooled compatibility scores and rendering speed are
   separate measures.
5. Record observations here or in the existing project notes. Dependency and
   architectural decisions, with alternatives, belong in ADRs.

## Historical landscape notes — 2026-11-13

The remaining sections preserve the earlier app-versus-infrastructure analysis.
They are historical research, not current scope or an approved strategy. Their
app deliverables, open "app or SDK" choice, categorical rankings, adoption and
pricing claims must not override the SDK-only direction or the provisional
comparisons above. Recheck facts before using them as evidence.

## The field splits into two camps

1. **AI-native spreadsheet apps** — the grid is the product; AI is a feature.
   Quadratic, Equals, Rows, Paradigm, plus Excel/Sheets with assistants.
2. **Embeddable spreadsheet/agent infrastructure** — the runtime is the product.
   Univer is the clear example; Formualizer, HyperFormula and Recalc are engine
   libraries in the same supply chain.

We have to choose which war we are in. Competing with both camps with five people
is not a strategy.

## Matrix

| Product | Camp | Engine | Agent harness | Collaboration | License / openness | Price signal |
| ------- | ---- | ------ | ------------- | ------------- | ------------------ | ------------ |
| **Quadratic** | App | Own Rust/WASM/WebGL core; Python/SQL/JS in cells; ~100M rows claim | AI writes code/formulas; MCP server + REST so external agents read/write | Real-time multiplayer, version history | Closed / source-available; self-host licensed (free ≤3 users) | Free; $18 Pro; $36 Business /user/mo |
| **Equals** | App | In-house JS engine (custom formula language) | AI Analyst; MCP; "verify any number" cell-level lineage | Versioned snapshots, audit-friendly | Closed | ~$24k–60k/yr; no free plan |
| **Rows** | App | Excel/Sheets-compatible engine (in-house; unverified) | AI Analyst, AI functions, checkpoints | Real-time, guests, embeds | Closed | Free; ~$8 Plus; ~$79 Pro + seats |
| **Paradigm** | App (niche) | Not disclosed | Per-cell agents that research/enrich | Web | Closed | Free tier + limits |
| **Univer** | Infrastructure + SDK | **TypeScript monorepo core** (models, plugins, commands, rendering); formula engine hybrid — frontend TS + server-side **Rust**; OSS engine admitted slower than commercial | "Office Harness for AI Agents": worktrees, tool API/CLI, agent loop | OT co-editing; operation history; human review/merge | Apache-2.0 core; Server SDK, pro AI, XLSX exchange commercial; eval watermarks | Pro modules paid; pricing not public |
| **Excel / Sheets** | Incumbent apps | Closed | Single assistant (Copilot / Gemini), MCP-adjacent | Mature multiplayer | Closed | Bundled |
| **Formualizer** | Engine library | Rust, Arrow, 400+ functions | None (embeddable) | None | Apache-2.0 | Free |
| **HyperFormula** | Engine library | JS, ~400 functions | None | None | GPLv3 or paid commercial | Free/paid |
| **Recalc** | Engine + verification | Rust, pre-release | None | None | Apache-2.0 | Free |

## What each one actually threatens

- **Quadratic** — the strongest "Rust-fast, code-in-cells, agent-accessible"
  app. It already owns the performance + AI-code story, with MCP for external
  agents. Threat to any pitch that is "Rust + agents".
- **Equals** — owns **trust in finance**: lineage, "verify any number", audit.
  This is the closest existing product to our evidence/audit idea. Closing the
  gap requires more than lineage: computation evidence and refusal semantics.
- **Rows** — owns **cheap, connected, non-technical** workflows. Less depth in
  calculation, but low price and fast time-to-value.
- **Paradigm** — owns the **per-cell agent** UX (each cell researches/fills).
  Niche, but a sharp agent-native story.
- **Univer** — owns the **category definition and embeddable harness**. Open
  core, wide plugin surface, Rust engine, OT, worktrees. Its weaknesses are
  commercial edges (XLSX, Server SDK, pro AI), a TS core, an OSS engine it says
  is slower than its commercial one, and worktree isolation rather than live
  multi-agent editing.
- **Excel/Sheets** — distribution and trust, but single-assistant, not
  multi-agent-native.

## Which axis each competitor defines

Do not conflate these; they are different benchmarks.

| Axis | Benchmark | Why |
| ---- | --------- | --- |
| **Engine / new-age architecture** | **Quadratic** — Rust/WASM/WebGL core | It is the Rust-native product; closed, so compare published claims and equivalent workloads, not code |
| **Harness / SDK / distribution** | **Univer** — TypeScript core, hybrid Rust engine, agent harness | The only testable OSS SDK + app + agent positioning; not Rust-first |
| **Trust / audit in finance** | **Equals** — lineage, "verify any number" | Closest existing trust story; closed, expensive |
| **Cheap, connected, non-technical** | **Rows** | Price and connector breadth, not calculation depth |
| **Per-cell agents** | **Paradigm** | Niche agent-native UX |

Univer's Rust formula engine is a **component**, not a Rust-first core. Treating
it as the architecture to beat would misread the market: Quadratic owns that
axis, Univer owns the harness axis.

## The uncomfortable synthesis

Every axis we have claimed is already occupied:

| Our claim | Already occupied by |
| --------- | ------------------- |
| Rust, fast engine | Quadratic, Univer (Rust engine) |
| Agentic harness | Univer (category leader), Quadratic/Equals (MCP) |
| Auditable / verifiable results | Equals ("verify any number", lineage) |
| Open / self-hosted | Univer OSS core, Quadratic source-available self-host |
| Cheap / connected data | Rows |

Therefore "Rust + agents + audit" is not a position. A position is an
**intersection** that a specific buyer cannot get elsewhere.

## Candidate positions

### A. Harness-first infrastructure (compete with Univer)

**Pitch:** an open, all-Rust engine + agent runtime with evidence-labeled
results and deterministic replay, embeddable in other products and self-hosted.

- Exploits: our engine work, diagnostics model, team size (no UI/connector
  surface), Univer's commercial edges and TS core.
- Requires: SDK/API, headless runtime, collaboration/op log, XLSX path, plugin
  or extension surface.
- Risk: Univer is entrenched and 30k-star; infrastructure sales are slow;
  developers may prefer the incumbent's ecosystem.

### B. Agent-native app for a vertical (compete with Quadratic/Equals/Rows)

**Pitch:** the auditable spreadsheet for a specific workflow (e.g. FP&A,
revenue ops, underwriting), where every agent-produced number carries evidence
and can be replayed.

- Exploits: a clear buyer and a concrete "wrong number" pain; Equals proves
  buyers pay for trust.
- Requires: UI, connectors, auth, sharing, imports/exports — a much larger
  surface than the harness.
- Risk: five people against Quadratic's Rust team and Equals' domain depth;
  integration breadth becomes the bottleneck.

### C. Both (open-core harness + reference app)

- Attractive narrative; usually means two under-resourced products. Only sane
  after one side has traction.

## What must be true to win

1. **A benchmark win on a workload the buyer feels** — not generic "fast". Same
   machine, same workbook, Quadratic/Univer where measurable.
2. **Fidelity parity on the functions that matter to the chosen buyer**, proven
   by corpus, not marketing.
3. **The trust claim must exceed Equals**: not just lineage, but
   computed-vs-cached-vs-refused evidence and deterministic replay of agent runs.
4. **An openness/licensing edge** Univer cannot match without cannibalising its
   pro tier (XLSX, server, AI extensions).
5. **One beachhead**, one buyer, one workflow.

## Open questions

- Which camp are we in: harness or app? (This is Gate 1, and it determines the
  entire roadmap.)
- What does "open" mean for us: Apache-2.0 core? self-host? no watermark?
- Is XLSX interchange in scope at launch, or an explicit later differentiator?
- Does "multi-agent" mean isolated drafts with human merge (Univer's model) or
  live concurrent editing on one sheet (harder, potentially unique)?
- Can we measure Quadratic at all, given it is closed/source-available?
