# Competitive Landscape — AI-native spreadsheets and agent harnesses (2026)

> Observations only. Decisions go to `ADRs/`. Pricing/claims are vendor or
> third-party reported and may change; verify before relying on them.
> Researched: 2026-11-13.

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
