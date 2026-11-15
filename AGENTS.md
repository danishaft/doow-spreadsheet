# AGENTS.md — Engineering Operating Model

Project: **doow-spreadsheet** (working name)
Status: **Binding.** Read this before any task. It is the contract for how work is
proposed, approved, built and verified in this repository.

---

## 0. What this repository is

A spreadsheet product built around a Rust calculation engine, with an agentic
application layer. This section will be replaced with a precise description once
Gate 1 (Product) is approved. Until then, treat product behavior as undefined and
do not invent it.

---

## 1. Development process

**Research-backed Spec-Driven Development (SDD) + Test-Driven Development (TDD)
+ bounded agent implementation, under human approval.**

This is not a waterfall. Specifications, design and implementation may expose
problems that force us to revisit earlier decisions. The rule is that changes are
**explicit and approved**, never silently invented by an implementation agent.

### Gates

| Gate | Name         | Passed when                                                             | Owner |
| ---- | ------------ | ----------------------------------------------------------------------- | ----- |
| 0    | Feasibility  | Research notes + benchmark prototype prove the engine approach          | TBD   |
| 1    | Product      | `FOUNDATION.md` and `PRD.md` approved                                   | TBD   |
| 2    | Requirements | `SPECIFICATIONS/` entries are testable and approved                     | TBD   |
| 3    | Architecture | `SYSTEM_DESIGN.md` and relevant `ADRs/` approved                        | TBD   |
| —    | Acceptance   | Tests + behavior corpus pass, CI green, change human-reviewed           | TBD   |

- No gate is passed because an agent produced an artifact.
- Every artifact has a **named human approver**.
- Approved gate artifacts are frozen until a change is proposed (see §7).

---

## 2. Hard rules (non-negotiable)

1. **Specs before implementation.** Agents must not invent product behavior.
2. **Research before architectural decisions.** Important choices require
   evidence, benchmarks, or an explicit trade-off analysis.
3. **Tests are the specification.** Acceptance criteria must be executable, not
   vague statements.
4. **Small, bounded tasks.** One subsystem per task is too large.
5. **Human review on critical changes.** Engine, storage, security and API
   contracts require human review. Agent review is supplementary, not
   independence.
6. **No silent scope changes.** Changing approved behavior means editing the spec
   and the decision log in the same change.
7. **Measure performance.** Rust does not automatically mean fast. Benchmarks or
   it did not happen.
8. **Human ownership.** Humans approve product direction, architectural
   trade-offs and releases.

---

## 3. Correctness is defined by the behavior corpus

For the calculation core, **`test-corpus/` is the executable specification.**
Prose defines UX, flows and intent; the corpus defines calculation correctness.

- Any engine change must keep the corpus green.
- New behavior lands with new corpus vectors.
- Disagreement between prose and corpus is resolved explicitly and recorded.

---

## 4. Documentation hierarchy

Keep these artifacts current; do not create parallel shadow documents.

| Artifact                    | Purpose                                        | Change when                  |
| --------------------------- | ---------------------------------------------- | ---------------------------- |
| `FOUNDATION.md`             | Vision, principles, scope                      | Direction or scope changes   |
| `PRD.md`                    | Requirements and user journeys                 | Product requirements change  |
| `SPECIFICATIONS/`           | Detailed, testable feature behavior            | Feature behavior changes     |
| `SYSTEM_DESIGN.md`          | Architecture and subsystem boundaries          | Architecture changes         |
| `ADRs/`                     | Decisions + alternatives considered            | A significant decision is made |
| `IMPLEMENTATION_PLAN.md`    | Milestones, dependencies, tasks                | Work progresses              |
| `TEST_STRATEGY.md`          | Testing and verification standards             | Quality strategy changes     |
| `research/`                 | What we observed elsewhere                     | Research progresses          |
| `test-corpus/`              | Executable correctness vectors                 | Behavior is added or fixed   |

`research/` records **observations**; `ADRs/` record **our decisions**. Never
present someone else's implementation as our design.

---

## 5. Agent roles

These are responsibilities, not necessarily separate agents. Introduce parallel
execution only where tasks have clean boundaries.

- **Product / requirements** — challenge assumptions, find missing requirements,
  draft specs.
- **Research / architecture** — investigate repos, evaluate alternatives, prepare
  evidence-backed proposals.
- **Implementation** — bounded tasks (Rust calculation, TypeScript APIs, UI,
  tests) that follow approved contracts.
- **Review** — check work against the spec, probe edge cases, flag architectural
  violations.
- **Verification** — run tests, check acceptance criteria, report missing
  evidence.

An implementation agent is never the sole reviewer of a critical change.

---

## 6. Verification

"Tests pass" is a command, not a claim. CI must enforce, and agents must run
locally before submitting:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --workspace`
- the behavior-corpus runner
- `cargo criterion`-based benchmarks with regression thresholds

> These commands become authoritative once the workspace is scaffolded in Gate 0.
> Until then, this section defines the intended policy.

---

## 7. Change control

- Approved artifacts change through an explicit edit plus a decision entry.
- Record architectural decisions as ADRs (`ADRs/NNNN-title.md`).
- OpenSpec is **deferred** until there is an established codebase and a real
  change proposal. Do not install it now.

---

## 8. Deferred decisions

Not part of the current process. Revisit only by explicit decision:

- OpenSpec for evolving changes
- Symphony or any multi-agent orchestrator
- Parallel agent execution beyond clean task boundaries

---

## 9. Repository map

```
AGENTS.md               This file — operating model
FOUNDATION.md           Vision, principles, scope
PRD.md                  Product requirements
SPECIFICATIONS/         Testable feature behavior
SYSTEM_DESIGN.md        Architecture
ADRs/                   Decision records
IMPLEMENTATION_PLAN.md  Milestones and tasks
TEST_STRATEGY.md        Testing standards
research/               What we observed elsewhere
test-corpus/            Executable correctness vectors
```
