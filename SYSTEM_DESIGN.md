# SYSTEM DESIGN — Architecture and Subsystem Boundaries

> Status: **Draft — not approved.** Gate 3 artifact.
> Depends on: Gate 0 findings, `PRD.md` · Owner: TBD · Approved by: TBD · Last updated: —

## 1. System overview

Context diagram and the one-paragraph architecture story.

## 2. Subsystems

For each: responsibility, boundary, inputs/outputs, owned data.

- Rust calculation engine
- Application layer
- Agent runtime and orchestration
- Storage
- Collaboration
- API boundaries
- UI

## 3. Data flow

Recalculation path, persistence path, collaboration path, agent invocation path.

## 4. Calculation engine

Formula representation, dependency graph, recalculation strategy, error
propagation, performance model. Must cite Gate 0 evidence.

## 5. Interfaces and contracts

Link to technical specs beside each feature spec. No contract lives only in code.

## 6. Cross-cutting concerns

Concurrency, consistency, failure handling, observability, security.

## 7. Key decisions

Link to ADRs. Every significant choice has an ADR with alternatives considered.

## 8. Open questions

## Approval

- [ ] Subsystem boundaries agreed
- [ ] Interfaces defined or linked
- [ ] Important ADRs accepted
