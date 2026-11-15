# ADR-0001: Development process and engineering operating model

- **Status:** Accepted
- **Date:** 2026-11-13
- **Deciders:** project owners (team of 5)
- **Related:** `AGENTS.md`

## Context

We are starting a spreadsheet product with a Rust calculation engine and an
agentic application layer. Before writing a PRD or system design, we needed to
settle how the product will be designed, validated, built and changed — otherwise
we risk comprehensive-looking documents that do not govern actual work.

The dominant risk in this product is not documentation. It is the calculation
engine: formula semantics, recalculation correctness and performance. Product
docs written before that feasibility is proven may be premature.

We also intend to use coding agents, which raises the stakes on explicit specs,
bounded tasks and enforced verification.

## Decision

Adopt a **research-backed Spec-Driven Development + TDD + bounded agent
implementation** process, with human approval gates.

Specifics:

1. **Gate 0 — Feasibility first.** Research Formualizer, Mog and recalc, then
   build a benchmark prototype proving the engine approach before the PRD.
2. **Behavior corpus is the executable spec for the core.** `test-corpus/` holds
   Excel/LibreOffice/HyperFormula-derived vectors; prose covers UX and intent.
3. **One primary workflow.** GitHub Spec Kit is the primary SDD framework.
   Matt Pocock's interrogation approach is used as a discovery prompt/skill, not
   a framework. OpenSpec and Symphony are deferred.
4. **Trimmed artifact set (~6 top-level docs).** `FOUNDATION.md`, `PRD.md`,
   `SPECIFICATIONS/`, `SYSTEM_DESIGN.md` + `ADRs/`, `IMPLEMENTATION_PLAN.md`,
   `TEST_STRATEGY.md`. Technical specs live beside each feature spec.
5. **Independent review defined.** Critical changes (engine, storage, security,
   API contracts) require human review; agent review is supplementary.
6. **Executable rules.** The model lives in `AGENTS.md` and is enforced by CI
   (fmt, clippy, tests, benchmark regression).
7. **Lightweight change control.** Spec edit + decision/changelog entry; full
   OpenSpec ceremony only when the codebase warrants it.

Gates: 0 Feasibility, 1 Product, 2 Requirements, 3 Architecture, plus acceptance
(tests + corpus green, CI green, human-reviewed).

## Alternatives considered

- **Write the full document hierarchy first (vision through technical specs).**
  Rejected: documentation theater for a team of five; specs for an unproven
  engine.
- **Adopt Spec Kit + OpenSpec + Pocock skills simultaneously.** Rejected:
  overlapping command sets produce contradictory agent behavior before we have a
  codebase.
- **Test-driven development without a behavior corpus.** Rejected: spreadsheet
  correctness is defined by thousands of formula vectors, not hand-written
  examples alone.
- **Agent-review as independent review.** Rejected: not independent; critical
  changes require a human.

## Consequences

- Gate 0 must complete before product documentation becomes a priority.
- The corpus runner and CI gates are required infrastructure, not optional.
- OpenSpec and multi-agent orchestration are explicitly out of scope for now and
  require a new ADR to introduce.
- Every approved artifact needs a named human owner; the team of five must
  assign gate owners.
