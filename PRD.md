# PRD — Agent-native Spreadsheet SDK

> Status: **Draft v3 — product direction settled; release baseline awaiting review.**
> Gate 1 artifact · Owner/approver: TBD · Updated: 2026-11-17.
> Basis: [FOUNDATION.md](FOUNDATION.md) and the recorded decisions below.

## 1. Product and settled decisions

Build a spreadsheet SDK that combines a capable embeddable human editor,
headless operation, and a built-in agent harness around shared workbook
semantics. Developers bring models, context, and application logic; the SDK
provides calculation, editing, structured controls, and versioned workspaces
for parallel tasks. Rust calculation and WebGL graphics are the intended
technical direction. SpreadJS is the spreadsheet SDK baseline; Univer informs
extensibility and agent workspace workflows.

The product boundary and harness direction are established. The release
proposal below makes their required behavior reviewable.

| Settled direction | Recorded basis |
| ----------------- | -------------- |
| This project delivers the SDK; Doow's CFO application is a downstream customer. | User's SDK-only clarification on 2026-11-17; Foundation §5. |
| Define the full SDK with a first-release proposal. | User selected “Full SDK + release proposal” during PRD planning. |
| Recommend an end-to-end core: editor, headless runtime, interchange, model integration, and parallel workspaces. | User selected “End-to-end core.” |
| Include a model-neutral runner, tools, and workspaces; hosts supply models, context, and application-specific orchestration. | User selected “Runner + tools + workspaces.” |
| Include revisions, attribution, diffs, restore, and task recovery first; propose full operation replay later. | User selected “Revisions and restore first.” |

Decision source: recovered session
`01a1ea88-7a68-7c91-8092-2eeb949496bd`. The planning selections were recorded
on 2026-11-17; the subsequent “go ahead then” authorized the rewrite.
Specific formula coverage, licensing, environments, and numeric budgets remain
release proposals. They are not attributed to user approval.

## 2. Customers and ownership

The primary customer is an engineering team building a spreadsheet product
used by humans and agents. It needs dependable embedding/customization,
programmatic control, parallel work, and recovery through public interfaces.

| Party | Owns |
| ----- | ---- |
| SDK | Workbook and editing semantics, calculation, editor, structured tools, generic task execution, workspace/revision mechanics, and enforcement of supplied policies. |
| Host/developer | Models and credentials, task context, domain tools and business logic, application-specific orchestration, identity/permissions, acceptance policy, storage, and collaboration transport. |
| Human or agent actor | Identified, authorized work through the SDK; humans can edit and review while agent tasks run. |

The host chooses providers and domain strategy. The SDK supplies the generic
task loop and spreadsheet controls. Host review flows use structured outcomes;
tool APIs do not require dialogs. Permissions apply to every entry point, and
workbook content cannot grant authority.

Storage and transport adapters must publish their guarantees. Accepted edits
can be unsaved; durability is acknowledged separately for a specific revision.
Hosts own external-service operation, costs, backup schedules, and availability.

## 3. Product terms

| Term | Meaning |
| ---- | ------- |
| Workbook | Authored sheets, cell content/formulas, supported presentation, and document metadata. |
| Revision / accepted state | An identified workbook state / the current state after authorized edits or integration; acceptance alone does not imply durable saving. |
| Workspace | An isolated draft based on an identified revision, unchanged in accepted state until integration. |
| Task / actor | A bounded, identified unit of agent work / the authorized human or agent responsible for actions. |
| Operation / change set | A supported spreadsheet edit / an attributable proposal integrated as one logical change. |
| Calculated result | A value or error evaluated under the declared calculation profile and context. |
| Source provenance | Available input-origin information, separate from calculation correctness. |

A task edits its workspace, produces a change set, and integrates it under host
policy after validation. Successful calculation means supported, current inputs
were evaluated correctly; it does not certify their real-world truth.
Workspaces provide Git-style branching and integration; backing storage and
synchronization algorithms remain architecture decisions.

## 4. Customer journeys

Requirements in §5 define the individual outcomes. These journeys identify the
customer workflows they must serve.

| ID | Actor and workflow | Supporting requirements |
| -- | ------------------ | ----------------------- |
| J-06 | Developer installs/configures the SDK, embeds independent editors, customizes controls, runs headlessly, resizes, and disposes instances. | FR-31, FR-32, FR-34–FR-36, FR-39, FR-41. |
| J-01 | Human creates sheets, enters/pastes values and formulas, fills, formats, organizes, saves, and reopens a workbook. | FR-01–FR-11, FR-15–FR-17, FR-29. |
| J-02 | Human or developer imports CSV/XLSX, inspects caches/limitations, edits, and exports supported content. | FR-09, FR-10, FR-18–FR-20. |
| J-09 | Developer or agent inspects structure/dependencies/results, batches edits, calculates, and handles events/errors without a UI. | FR-08–FR-11, FR-21, FR-26, FR-37–FR-39. |
| J-03 | Host connects a model and delegates bounded work; actors observe progress, handle failures/cancellation, and review proposals. | FR-21, FR-22, FR-27, FR-34, FR-40, FR-41. |
| J-04 | Two agents work in isolated drafts while a human edits; hosts compare and integrate valid changes and coordinate human clients. | FR-21–FR-26, FR-29, FR-30. |
| J-05 | Reviewer inspects results/history, restores permitted work, and recovers interrupted drafts/tasks; later, replays recorded operations. | FR-09–FR-11, FR-15–FR-17, FR-23, FR-25, FR-27–FR-29. |
| J-07 | Human creates a chart, changes its sources, saves, and exports findings. | FR-12, FR-13, FR-20. |
| J-08 | Developer later connects datasets, queries, connectors, or programming integrations to workbook analysis. | FR-13, FR-14, FR-41, FR-42. |

## 5. Prioritized functional requirements

- **P0 — required core:** release blockers for a usable end-to-end SDK.
- **P1 — recommended first-release additions:** useful extensions of that core;
  still included in the release proposal, but ranked after P0. Any deferral
  requires an explicit scope decision.
- **P2 — later:** broader capabilities proposed for subsequent releases.

P0 includes human editing, headless control, interchange, model execution,
parallel workspaces, and recovery together. This ranking is a release
recommendation, not a reduction of the full SDK direction. §6 contains the
release profiles; §7 contains quality guarantees.

### 5.1 Embedding and integration

| ID | Required behavior and observable outcome | Priority |
| -- | ---------------------------------------- | -------- |
| FR-31 | Developer can embed the editor or use headless APIs; equivalent supported operations agree on values, errors, status, and permissions without requiring a rendered page headlessly. | P0 |
| FR-32 | Developer can complete clean-install examples for editing, embedding, persistence, and agent tasks; published API/document versions and migrations preserve supported state or return explicit incompatibility errors. | P0 |
| FR-35 | Developer can create, configure, resize, and dispose independent instances; disposal releases owned resources/subscriptions and prevents later callbacks or scheduled work without disrupting another instance. | P0 |
| FR-36 | Developer can configure presentation/commands and contribute controls or review/error flows through public interfaces, without core patches or authorization bypass. | P0 |
| FR-41 | Developer can supply models, context, tools, storage, identity/permissions, acceptance policy, and transport adapters; replacement requires no core fork and unsupported combinations are documented. | P0 |
| FR-34 | Published terms permit free use, embedding, and sharing; examples require no paid SDK unlock and document external-service prerequisites/costs while protecting host credentials. | P0 |

### 5.2 Workbook and human editing

| ID | Required behavior and observable outcome | Priority |
| -- | ---------------------------------------- | -------- |
| FR-01 | Human/developer can create/open/name workbooks and manage sheets; reopen retains identities, order, and content, while rejected operations preserve existing valid state. | P0 |
| FR-02 | Human can edit typed values/dates and formulas through grid/formula controls; content round-trips under declared typing rules and invalid formulas produce diagnosable errors without unrelated loss. | P0 |
| FR-03 | Human can select/navigate by keyboard/pointer, copy/paste, and fill; relative/absolute/mixed references transform correctly and rejected paste leaves destinations unchanged. | P0 |
| FR-04 | Human can format numbers/dates, alignment/emphasis, and dimensions; presentation persists without changing stored values/formulas. | P0 |
| FR-05 | Human/developer can insert/delete rows/columns and change sheets; reference transformations follow the declared profile or expose invalid-reference errors. | P0 |
| FR-06 | Human can search, sort, and filter ranges; associated values/formulas/formatting follow the movement policy and clearing filters restores visibility without deletion. | P1 |
| FR-07 | Human can freeze panes and navigate named cells/ranges within supported sizes, preserving selection/edit state during scrolling. | P1 |
| FR-12 | Human/developer can create supported charts; source edits update values, configuration survives reopen, and blank/error/unsupported-value handling is explicit. | P1 |

Search/sort/filter, extended navigation, and charts are P1 because they expand
workbook organization and communication after the core editing workflow.

### 5.3 Calculation and programmatic control

| ID | Required behavior and observable outcome | Priority |
| -- | ---------------------------------------- | -------- |
| FR-08 | Every actor can calculate the declared profile through the shared Rust core; oracle-backed reference/function/spill fixtures match and changed inputs update dependent results. | P0 |
| FR-09 | Every actor receives explicit syntax/reference/cycle/unsupported outcomes; current supported inputs may compute successfully, while unevaluated dependencies cannot appear as successful current computation. | P0 |
| FR-10 | Every actor can distinguish provenance, current/pending/stale/unsupported status, imported caches, and detected comparison mismatches; caches are not presented as SDK-computed. | P0 |
| FR-11 | Relevant content/structure/context changes invalidate affected results; unaffected results retain valid status and stale results remain identifiable until recalculation. | P0 |
| FR-37 | Developer/agent can request bounded, permission-checked observations of structure/ranges/formulas/dependencies/results/provenance, identified by revision/workspace context. | P0 |
| FR-38 | Developer/agent can batch edits with declared atomicity; all-or-nothing failures identify the cause and leave targeted workbook/workspace state unchanged. | P0 |
| FR-39 | Developer can subscribe/unsubscribe to workbook/calculation/task/save events and receive correlated structured outcomes/errors; subscriptions end on disposal. | P0 |

### 5.4 Persistence and interchange

| ID | Required behavior and observable outcome | Priority |
| -- | ---------------------------------------- | -------- |
| FR-15 | Developer can save/reopen through storage adapters; success identifies the acknowledged durable revision and interrupted writes preserve the last acknowledgment under the adapter contract. | P0 |
| FR-16 | Human/developer can distinguish accepted edits, pending saves, and durable acknowledgments; interrupted drafts/task checkpoints recover within the declared boundary without silent external-call repetition. | P0 |
| FR-17 | Developer can retain and restore versioned documents and required history/task metadata without a vendor account, with documented retention/export and adapter requirements. | P0 |
| FR-18 | Human/developer can import CSV with controllable parsing/typing; malformed/oversized input reports a location/cause without partially replacing accepted work. | P0 |
| FR-19 | Human/developer can import the declared XLSX subset; supported content remains usable and structured reports identify unsupported features and cached results. | P0 |
| FR-20 | Human/developer can export declared CSV/XLSX content that reopens in supported readers; loss reports precede host confirmation, unsafe export is refused, and APIs require no dialog. | P0 |

### 5.5 Runner, workspaces, and collaboration

| ID | Required behavior and observable outcome | Priority |
| -- | ---------------------------------------- | -------- |
| FR-21 | Host grants actor/task-scoped read/write/tool authority; every entry point enforces it, including current permissions at integration and revoked/cross-workbook access. | P0 |
| FR-40 | Developer can use a model-neutral task loop with host context/adapters/tools; validated calls and structured observations/outcomes work through two different model adapters. | P0 |
| FR-22 | Host can run at least two identified agent tasks concurrently while humans edit; independent progress/attribution survives and overlapping proposals receive explicit handling. | P0 |
| FR-23 | Actors can draft from identified revisions and inspect diffs of content/structure/presentation/results; rejected drafts leave accepted state unchanged and integration follows host acceptance policy. | P0 |
| FR-24 | Integration detects competing writes, structural changes, stale read inputs, and permission changes; unsafe proposals are blocked or revalidated with reasons, preventing silently stale derived results. | P0 |
| FR-25 | Accepted changes identify actor/operation/revision and agent task/workspace context; rejected drafts remain distinguishable under declared retention/privacy boundaries. | P0 |
| FR-26 | Identified change sets integrate atomically; retry produces one logical accepted change and faults expose old or complete new state, with durability acknowledged separately. | P0 |
| FR-27 | Host observes progress and configures limits/cancellation; exhausted limits stop new calls, acknowledged cancellation stops further SDK scheduling/mutations, and failures retain usable accepted state with explicit in-flight/checkpoint treatment. | P0 |
| FR-29 | Authorized actors can inspect revisions and undo/restore edits/proposals as attributed changes, preserving intervening unrelated work or surfacing a conflict. | P0 |
| FR-30 | Developer connects authorized human clients through transport adapters; reconnect converges without duplicate changes and connection/save status stays visible. | P1 |
| FR-28 | Later replay uses a starting revision, recorded operations, captured inputs, and calculation/clock context; gaps/mismatches are reported without fresh external calls or assumed deterministic model generation. | P2 |

Multi-client human collaboration is P1; same-host human editing during parallel
agent work is P0. Full replay follows the selected revisions/restore-first scope.

### 5.6 Later analytical capabilities

| ID | Required behavior and observable outcome | Priority |
| -- | ---------------------------------------- | -------- |
| FR-13 | Developer can add selected connectors with preview/mapping/refresh, permissions, and timestamps; failed refresh retains prior data with explicit age/status and protected secrets. | P2 |
| FR-14 | Developer can enable approved programming-in-workbook integrations with typed inputs/outputs, dependencies, isolation, cancellation, and diagnosable failures. | P2 |
| FR-42 | Developer can connect structured datasets/queries with declared schemas, result placement, provenance, invalidation, and limits; failed analysis preserves accepted work. | P2 |

These add separate execution/data contracts after the end-to-end core.

Traceability: FR-01–FR-32 and FR-34 retain their purposes; FR-35–FR-42 are
additions from v2. FR-33 (standalone deployment) remains retired, with SDK
installation/recovery retained in FR-15–FR-17 and FR-32. No ID is repurposed.

## 6. Proposed release profiles

The first release recommends all P0 and P1 requirements; P2 follows later.
Broader formula coverage, richer charts/pivots, and offline collaboration are
also later proposals. Release dates and complete Excel parity are not promised.

| Profile | First-release recommendation | Remaining detail |
| ------- | ---------------------------- | ---------------- |
| Calculation | Arithmetic/comparison; relative/absolute/mixed and cross-sheet references; candidate functions below; basic dynamic-array spills; explicit cycle errors. | OQ-02: exact supported semantics, including locale/date system and reference movement. |
| Visualization | Bar and line charts. | OQ-05: supported chart behavior; richer charts/pivots later. |
| Interchange | CSV values for one selected sheet; XLSX supported sheets, typed values/formulas, basic formatting, and charts. Machine-readable compatibility/loss reporting. | OQ-03: readers, preservation guarantees, conversion rules, and limits. |
| Execution | Rust headless crate, browser WASM/npm editor/core, npm headless on an approved Node LTS; two latest stable desktop Chrome/Firefox/Safari releases at profile freeze. | OQ-09: version pins, migration and extension-support matrix. |

The candidate function list is retained from the earlier draft as a proposal:
`SUM`, `AVERAGE`, `MIN`, `MAX`, `COUNT`, `COUNTA`, `COUNTBLANK`, `IF`,
`IFERROR`, `AND`, `OR`, `NOT`, `SUMIF`, `SUMIFS`, `COUNTIF`, `COUNTIFS`,
`INDEX`, `MATCH`, `VLOOKUP`, `XLOOKUP`, `ABS`, `INT`, `MOD`, `ROUND`,
`ROUNDUP`, `ROUNDDOWN`, `DATE`, `YEAR`, `MONTH`, `DAY`, `TODAY`, `NOW`,
`LEFT`, `RIGHT`, `MID`, `LEN`, `CONCAT`, `TEXT`, `SEQUENCE`, `FILTER`,
`SORT`, and `UNIQUE`. Exact argument/coercion/error/spill/volatile-context rules
belong in oracle-backed specifications. This list is not an implementation claim.

## 7. Quality requirements

These apply to the release scope above. Correctness/isolation/recovery guarantees
are required core quality; numeric budgets remain proposals under OQ-10.
NFR-11 applies when multi-client collaboration is included; replay-specific
checks in NFR-12 apply with P2 replay.

| ID | Required quality and measurable boundary |
| -- | ---------------------------------------- |
| NFR-01 | 100% of approved supported-behavior corpus vectors pass with pinned oracle provenance/tolerances; no unresolved silent-wrong supported result. Track coverage breadth separately. |
| NFR-02 | 100% agreement across supported human/embedded/headless/programmatic fixtures for results, errors, status, authority, and accepted edits. |
| NFR-03 | Proposed 1M-formula first calculation <1.5 s and p95 broad 1M-dependent edit-to-recalculated result <150 ms; construction is separate and edit/invalidation overhead is included. |
| NFR-04 | Proposed p95 editor navigation/selection/edit feedback ≤100 ms at 100k populated cells; measure input-to-result separately, including concurrent tasks. |
| NFR-05 | Zero loss of adapter-acknowledged durable revisions in approved crash/write-fault tests; unsaved recovery and external backup guarantees are explicit. |
| NFR-06 | Publish workbook/import/resource limits for the proposed 8 GB desktop envelope; enforced-limit failures preserve durable state. |
| NFR-07 | Zero unauthorized operations in the approved permission matrix, including revocation/stale drafts/cross-workbook access; opening files does not execute macros/code. |
| NFR-08 | Credentials stay outside documents/logs/exports/history; external data/tool use follows host policy and workbook content cannot expand authority. |
| NFR-09 | Provider/timeout/import/resource failures cause no partial integration or damaged durable state; errors identify affected work and recovery. |
| NFR-10 | Proposed WCAG 2.2 AA for critical editing/review/recovery workflows, with keyboard completion and meaningful focus/screen-reader feedback. |
| NFR-11 | Authorized clients converge after concurrent edits/reconnect without duplicates; proposed p95 propagation ≤500 ms for three users at ≤50 ms RTT under declared adapters. |
| NFR-12 | 100% of accepted changes retain required attribution/revision/task records; restore/recovery retain metadata, and later replay matches within tolerances or reports gaps. |
| NFR-13 | Proposed install-to-working embed ≤15 minutes and workbook-open-to-successful agent task ≤10 minutes, with host/provider prerequisites reported separately. |
| NFR-14 | Packages/documents publish versions/support profiles; supported upgrades preserve state and incompatible versions fail explicitly. |
| NFR-15 | Mutation/task/save/provider/permission failures have correlated, sanitized diagnostics and declared retention. |
| NFR-16 | Measure workspace creation time and incremental memory/storage by workbook size and draft count; isolation/cleanup hold within approved limits. Numeric budgets remain open. |
| NFR-17 | Measure runner start/progress/cancellation acknowledgment separately from model latency; disclose in-flight call behavior. Numeric responsiveness budgets remain open. |

Timing/onboarding proposals come from PRD v1 and earlier Foundation drafts,
not user-approved performance commitments. The proposed reference is the
recorded i5-8257U/8 GB host with 100k-cell editor and 1M-formula core fixtures.
Declare distributions, copied/irregular and cross-sheet formulas, narrow/broad
edits, run counts, warm/cold conditions, and memory separately.

Existing [benchmark evidence](research/benchmarks/results.md) and the
[83/84 probe](research/corpus-probe/results.md) are feasibility observations.
They do not establish SDK performance or oracle-backed release compatibility.

## 8. Product acceptance and success

Individual requirements carry their observable outcomes. Acceptance must also
prove these workflows work together; these are future checks, not executed
runtime tests for this document edit.

| Scenario | Integrated acceptance | Traceability |
| -------- | --------------------- | ------------ |
| AC-01 | Clean-install, customize, resize, and dispose independent embeds; bounded inspection, batches, events, and errors expose definite headless outcomes. Supported upgrades preserve state; incompatible versions fail explicitly. | J-06, J-09; FR-31, FR-32, FR-35–FR-39, FR-41; NFR-02, NFR-14. |
| AC-02 | Complete keyboard/screen-reader editing, result inspection, review, and save/recovery. Equivalent human/headless edits and manual-input calculations agree; stale/unsupported/cache status remains explicit. | J-01, J-05; FR-01–FR-11, FR-15–FR-17; NFR-01, NFR-02, NFR-10. |
| AC-04 | Two model adapters complete the same task contract; invalid/denied calls, timeout, provider failure, limits, and cancellation have defined outcomes. | J-03; FR-21, FR-27, FR-40, FR-41; NFR-07–NFR-09, NFR-17. |
| AC-05 | Two isolated agents progress during human edits; disjoint work integrates, while write/structural/stale-read/permission conflicts are explained and retried integration produces one atomic change. Multi-client reconnect converges when included. | J-04; FR-21–FR-26, FR-30; NFR-07, NFR-11, NFR-12, NFR-16. |
| AC-07 | Save faults preserve acknowledged revisions; drafts/tasks recover without silently repeating external calls; attributed undo/restore handles intervening edits. | J-05; FR-15–FR-17, FR-25, FR-27, FR-29; NFR-05, NFR-09, NFR-12, NFR-15. |
| AC-08 | Supported CSV/XLSX content and charts round-trip; loss reports precede host confirmation and malformed input leaves accepted work intact. | J-02, J-07; FR-12, FR-18–FR-20. |
| AC-11 | Approved performance/resource budgets pass declared workloads; public docs support timed onboarding, free SDK use/sharing, and explicit external-service costs. | FR-34; NFR-03, NFR-04, NFR-06, NFR-13, NFR-16, NFR-17. |
| AC-12 | Later connector/code/analysis failures preserve work; recorded replay uses captured inputs, reports gaps, and causes no fresh external effects. | J-08, J-05; FR-13, FR-14, FR-28, FR-42. |

Scenario IDs retain their purposes. AC-03, AC-06, AC-09, AC-10, and AC-13
are consolidated into AC-01, AC-05, AC-02, AC-01, and AC-11 respectively.
Detailed fixtures and procedures belong in specifications and
[TEST_STRATEGY.md](TEST_STRATEGY.md).

Measure independent integration success, time to first embed/task, reliance on
core patches, repeat use by integrating teams, task acceptance/failure rates,
and recovery of acknowledged work. Adoption targets require a defined cohort.
Claims of improvement over alternatives require comparable integrations and
workloads; Rust/WebGL alone do not establish an advantage.

## 9. Remaining release decisions and handoff

These questions concern release breadth and detailed contracts. They do not
reopen the SDK-only boundary, built-in runner, parallel workspaces, host ownership,
or revisions/restore-before-replay decisions. Owners are roles until named
humans are assigned.

| ID | Remaining choice | Owner and checkpoint |
| -- | ---------------- | -------------------- |
| OQ-01 | Select early adopter/reference integrations and adoption measures; recommend an embedded human+agent host plus independent headless use. | Product owner; before success/onboarding targets freeze. |
| OQ-02 | Finalize §6's supported function/reference/date/locale/spill/cycle profile for FR-01–FR-11 and FR-37. | Product owner + calculation lead; Gate 1 breadth, Gate 2 semantic fixtures. |
| OQ-03 | Finalize CSV/XLSX readers/subset/conversion/loss policy and limits for FR-18–FR-20. | Product owner + interchange lead; Gate 1 scope, Gate 2 format contract. |
| OQ-04 | Specify acceptance authorization, stale-read/conflict revalidation, and retry rules for FR-21–FR-27/FR-29. Draft isolation and host-controlled acceptance are established; direct agent writes require a separate future proposal. | Product owner + integration lead; Gate 2 externally visible policy. |
| OQ-05 | Confirm P1 inclusion and chart profile for FR-06/FR-07/FR-12/FR-30; any deferral must be recorded. P2 priorities remain proposals for later work. | Product owner; Gate 1 release scope. |
| OQ-06 | Set license/source/distribution rights for FR-34; recommend Apache-2.0 subject to rights review, with host-supplied external services. | Product owner + distribution reviewer; Gate 1 terms and release rights review. |
| OQ-07 | Specify provenance vocabulary, history retention/export/privacy, and recoverable task checkpoints for FR-10/FR-16/FR-17/FR-25/FR-27. Replay timing is already settled. | Product owner + privacy/storage leads; Gate 2 history/recovery contract. |
| OQ-08 | Specify storage acknowledgment/failure, permission enforcement, and transport obligations for FR-15–FR-17/FR-21/FR-30/FR-41; retain SDK/host ownership in §2. | Storage/integration leads; Gate 2 adapter guarantees, Gate 3 mechanisms. |
| OQ-09 | Finalize §6's runtime/browser/package pins, extension support, and migration policy for FR-31/FR-32/FR-35/FR-41. | Product owner + SDK lead; Gate 1 support matrix, Gate 2 version contract. |
| OQ-10 | Accept/revise §7's numeric targets; define resource limits, fixture distributions, sample counts, and workspace/runner budgets. | Product owner + performance/verification leads; Gate 1 targets, Gate 2/3 measurement contracts. |

Gate 1 approval requires named ownership, an agreed release profile/priorities,
and resolution of release-defining choices above. Feature rules are specified
in [SPECIFICATIONS/](SPECIFICATIONS/); architecture and alternatives belong in
[SYSTEM_DESIGN.md](SYSTEM_DESIGN.md) and [ADRs/](ADRs/).
[AGENTS.md](AGENTS.md) governs approval and verification.
[IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) records progress; Gate 0,
ADR-0002, and final product approval remain pending.

Revision v3: clarified recorded decisions, ranked P0/P1/P2, removed repeated
scope/acceptance/governance material, and narrowed open questions to unresolved
release details. The Foundation remains unchanged.
