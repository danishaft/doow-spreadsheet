# FOUNDATION — Agent-native Spreadsheet SDK

> Status: Draft v2 — rewrite approved by the user; review pending.
> Owner: TBD · Updated: 2026-11-17

## 1. Product thesis

Build a spreadsheet SDK in which humans and agents can inspect, edit,
calculate, and develop workbook changes through a shared runtime.

The SDK combines a capable embeddable editor, headless operation, and
a built-in agent workspace harness. Agents work through structured
spreadsheet controls and versioned workspaces, with Git-style
branching, change inspection, and integration workflows.

Developers bring their models, context, and application logic. The SDK
supplies the spreadsheet environment those applications operate on.

Rust calculation and WebGL graphics are the intended technical
direction for efficient computation and responsive interaction.
Detailed architecture remains subject to design and validation.

## 2. Primary customer and problem

The primary customer is an engineering team building a spreadsheet
product used by humans and agents.

That team needs to provide ordinary spreadsheet editing while giving
agents reliable ways to understand workbook structure, change it,
observe calculation results, and coordinate parallel work.

These responsibilities span calculation semantics, editing operations,
task state, document revisions, conflict handling, and recovery.
When they are assembled inconsistently, developers carry the
integration burden and users encounter unpredictable behavior.

Our opportunity is to provide these capabilities as a coherent,
reusable SDK that developers can embed and extend.

## 3. Product position

The product competes as spreadsheet infrastructure.

SpreadJS establishes the baseline for a capable, customizable
spreadsheet SDK. Univer establishes a relevant baseline for
extensibility and agent workspace workflows. Other projects inform
specific architecture and compatibility questions documented in
research/competitive-landscape.md.

Our defining bet is that human editing, calculation, and versioned
agent work should be integrated around consistent workbook behavior.
This should give developers stronger control over automation and
parallel work while preserving a complete human editing experience.

The intended advantage includes lower integration effort, useful
agent controls, dependable change management, and better performance
on representative workloads. These advantages require evidence from
working integrations and comparative measurements.

## 4. Product principles

1. Humans and agents operate through consistent spreadsheet semantics.
   Their interfaces may differ; the meaning of an operation stays coherent.

2. Agent work is observable and controlled.
   Tasks, changes, calculation state, and conflicts can be inspected.
   The host application determines permissions and acceptance policies.

3. Parallel work preserves independent progress.
   Versioned workspaces support attributable changes and explicit
   integration when other work has intervened.

4. Spreadsheet quality remains fundamental.
   Human interaction, calculation correctness, and document handling
   receive the same attention as automation.

5. Developers retain control.
   Model selection, context, domain logic, and application policy are
   supplied through supported integration boundaries.

6. Performance and compatibility are demonstrated.
   Claims are tied to defined behavior and representative workloads.

## 5. Strategic boundaries

This project delivers the SDK: its editor, headless runtime, agent
workspace capabilities, public interfaces, documentation, and examples.

Standalone consumer applications and domain-specific products are
outside this project's scope. Example hosts demonstrate integration.

The SDK owns reusable spreadsheet and agent-workspace mechanics.
Host applications own their business meaning and agent strategies.

The SDK is intended to be free to use, embed, and share. Exact license,
source-access, and distribution terms remain to be settled.

Release features and compatibility profiles belong in PRD.md.
Implementation choices belong in SYSTEM_DESIGN.md and ADRs.

## 6. Outcomes that show the product is working

- Engineering teams integrate the SDK into real products and use its
  documented interfaces for both human editing and automation.
- Humans complete useful spreadsheet work while agent tasks execute.
- Agents understand relevant workbook state, make changes, calculate,
  and inspect results through structured controls.
- Parallel tasks produce useful, attributable changes that can be
  integrated with explicit handling of conflicts and stale inputs.
- Developers recover interrupted work and understand failed operations.
- Comparative evidence demonstrates an advantage in developer effort,
  interaction, calculation, memory use, or agent task execution.

Numeric targets and acceptance conditions belong in the PRD.

## 7. Assumptions to validate

- Engineering teams value an integrated spreadsheet and agent-workspace
  SDK enough to adopt it over existing alternatives.
- Our execution and graphics architecture can deliver meaningful
  improvements while meeting required spreadsheet behavior.
- The compatibility and customization needed by early adopters are
  achievable within the product's development capacity.
- Distribution terms can fulfill the commitment to free use,
  embedding, and sharing.
