# Feature Specification: Data Addon Workflow Recovery

**Feature Branch**: `codex/s125-data-addon-workflow-recovery`
**Created**: 2026-10-11
**Status**: Specified
**Input**: Owner-authorized S125, issues #265 through #271, end-to-end autopilot with automatic push and official PR, at most two Codex review rounds, and owner final review/merge.

## User Scenarios & Testing

### User Story 1 - Read and act on desktop guidance (Priority: P1)

A player can read every essential instruction and heading in Data Details, System and State, Catalog Update and Encounter History, then reach the appropriate recording or catalog action.

**Why this priority**: Hidden instructions and unreadable headings prevent use of the existing features.
**Independent Test**: Render actual empty/selected/busy/error surfaces in both themes, narrow/wide layouts, enlarged text and reduced heights. Inspect painted foreground/background, text completeness and clipping, and exercise reachable controls.
**Acceptance Scenarios**:
1. Given any addon lifecycle state or reload prefix, when its next step is shown, then the full concise action is readable without hover, focus disclosure or horizontal scrolling.
2. Given a modal surface heading, when rendered in either theme, then enabled normal text meets 4.5:1 contrast against its actual composited background; filled-control foreground remains correct.
3. Given short view height, when guidance overflows, then deliberate vertical scrolling reaches the instruction and close/action controls without collisions.

### User Story 2 - Deliberately record and receive replies (Priority: P1)

A player starts single or continuous recording with one explicit command, sees current activity and the applicable next action, and stops deliberately without losing recordings.

**Why this priority**: Debug interception currently suppresses player replies and command setup is unnecessarily indirect.
**Independent Test**: Registered production commands and lifecycle callbacks run with independent system-chat and blocked-debug sinks, covering initialization, fresh/waiting/active/retained/interrupted/failed states and malformed input.
**Acceptance Scenarios**:
1. Given a fresh controller, when the player enters /ewencounter start live single (or pts/continuous), then valid environment and mode are selected, recording starts and visible prefixed chat explains waiting or incomplete mid-combat capture.
2. Given enabled recording, when start is repeated, then current activity is reported and recording remains enabled. Repeated stop remains off and preserves results.
3. Given unavailable chat initialization, when essential guidance is generated, then it is retained for eventual system-chat delivery without duplicate replies.
4. Given invalid arguments or retained data, when start is attempted, then a concise correction or exact existing blocker is explained without silent deletion. Existing aliases remain compatible.
5. Given catalog commands, when help/status/progress/completion/failure occurs, then visible guidance describes catalog definitions and the actual desktop collection controls.

### User Story 3 - Inspect the saved source and recover imports (Priority: P1)

A player can inspect recording availability before the first import and distinguish last saved information from current in-game activity.

**Why this priority**: The initialized never-started file currently gets generic stop/save advice and detailed source state requires successful import.
**Independent Test**: Bounded read-only inspections and service/worker/UI fixtures cover every source and diagnostic state while retaining immutable imported data and duplicate behavior.
**Acceptance Scenarios**:
1. Given initialized never-started data, when inspected, then configure/start recording is the next action and no source or history write occurs.
2. Given a valid saved recording, when inspected, then selected path, modification time, environment and finished/partial counts are identified as last saved information. Active continuous sessions with retained terminal fights remain importable.
3. Given missing, inaccessible, unsupported, malformed or wrong-environment input, or a store failure, when inspection/import occurs, then controlled distinct recovery guidance is shown without raw private values.
4. Given a changed source selection, when an older asynchronous response arrives, then it cannot replace the selected source's state. Source refresh and imported-history refresh have distinct labels and effects.
5. Given an empty or entirely duplicate import, when completed, then the result and applicable action are distinct from failures and existing history is preserved.

### User Story 4 - Follow one complete recording/import path (Priority: P1)

A first-time player moves directly from Data Details to a compact ordered recording guide, with exact individually copyable commands and expected visible results. Maintainer catalog collection is disclosed separately.

**Why this priority**: Explaining terms alone has not made the current workflow usable.
**Independent Test**: Interaction fixtures follow Data Details to Encounter History, inspect commands/expected results, and cover candidate acknowledgment selection and refresh transitions.
**Acceptance Scenarios**:
1. Given installed files, when Record a fight / Open Encounter History is chosen, then enable, environment/mode, start, fight, save, import and view steps are reachable at point of use.
2. Given an empty, PTS-only or incompatible catalog selection, when Catalog Update opens, then no unusable acknowledgment checkbox appears and next actions are explained.
3. Given a compatible selected Live candidate, when acknowledgment is used, then it is beside candidate details/install controls, bound to content identity, and never transfers to changed contents. Busy/uncheck/selection changes retain the established service behavior.
4. Given any entry point, when guidance is read, then addon fight recordings, native Encounter.log (unsupported here), catalog definitions and application diagnostics have clear purposes. Bundled/public help agrees with commands and controls.

### Edge Cases

- Missing/unmanaged/outdated/unavailable addon; API/environment mismatch; reload prefix combinations.
- Empty and busy catalog discovery; PTS-only/incompatible candidates; changed candidate contents; keyboard acknowledgment and install intent.
- Chat before initialization, intercepted debug messages, repeated commands, extra/unknown arguments and old aliases.
- Never-started, waiting, active with completed retained fights, stopped, interrupted and failed source states; legacy supported captures; unknown versions; invalid UTF-8; malformed input; oversized/unstable/link source; denied reads; absent/invalid store.
- Source selection changes during asynchronous inspection; empty imports, reimports and existing history.
- Retained sessions still block beginning a different session until explicit clear. No automatic overwrite, automatic clear or claimed desktop knowledge of live game state.

## Requirements

### Functional Requirements

- **FR-001**: Repair and account for every surface-label emphasis call in the scoped UI audit using readable text roles, preserving filled-control text and enforcing actual rendered contrast in required tests (#265).
- **FR-002**: Shorten and audit every lifecycle next-step/reload combination, fully display essential shared-row instructions with content-derived height and reachable scrolling, and retain geometry/accessibility behavior (#267).
- **FR-003**: Deliver all addon user replies to visible system chat, separate diagnostics, and handle initialization without lost/duplicate essential messages (#268).
- **FR-004**: Provide explicit idempotent start/stop, concise state-aware help/status, exact argument correction and compatible aliases without discarded recording data (#270).
- **FR-005**: Inspect bounded saved-source data in the background without importing or mutating it; expose path/time/environment/counts and last-saved scope before first import (#271).
- **FR-006**: Preserve controlled typed read/version/schema/environment/store diagnostics and state-specific recovery, distinct empty/duplicate outcomes and continuous-session terminal-record import (#271).
- **FR-007**: Provide directly reachable, ordered, copyable recording/import actions and expected replies, matching state presentations and consistent public/bundled guidance (#269).
- **FR-008**: Separate maintainer catalog collection guidance and distinguish native logs, diagnostics, definitions and addon recordings at relevant entry points (#269).
- **FR-009**: Show acknowledgment only for an eligible selected Live candidate beside install controls; explain its purpose and preserve candidate-identity, busy and compatibility behavior (#266).
- **FR-010**: Preserve four-file managed addon ownership, independent module clearing, exact bounded capture, local-only private data, atomic duplicate imports and all input/lifecycle safety-critical coverage.
- **FR-011**: Complete all seven issue acceptance criteria through regression tests and review; update chronological planning and changelog; satisfy full local/hosted gates on the final PR head; request no more than two Codex rounds.

### Key Entities

- **Source inspection**: selected path/environment, inspection outcome, disk modification time, validated saved-controller summary and terminal counts, independent from imported history and live activity.
- **User reply**: purpose-prefixed actionable text delivered once through system chat, with bounded initialization buffering.
- **Workflow step**: exact command/control, expected observable result, applicable state and next action.
- **Candidate acknowledgment**: explicit install intent for one compatible Live content identity.

## Success Criteria

- **SC-001**: Every scoped emphasis and remediation branch is repaired or explicitly justified in an audit.
- **SC-002**: Enabled normal text in actual scoped fixtures meets 4.5:1 contrast and all essential instructions remain fully reachable in both themes and supported layouts.
- **SC-003**: Every tested user command/lifecycle response reaches the user sink despite blocked debug output; start never stops an active recorder and stop preserves data.
- **SC-004**: All listed source states and controlled failure categories have distinct applicable actions, inspection changes no data, and reimports preserve existing history.
- **SC-005**: All seven issues have complete mapped implementation/test evidence in one reviewed PR, green final-head checks and zero unresolved findings at owner handoff.

## Assumptions

- Native Encounter.log ingestion (#272) and multiple retained sessions (#273) remain separate features.
- Source inspection adds no live control channel or proof-of-import prerequisite; existing capture/retention semantics remain intact.
- No release/tag/publication or owner field-verification request belongs to S125. Owner performs final merge.

## Clarifications

### Session 2026-10-11

- Q: Should start rotate retained sessions? A: No, #273 owns that change. Validate all arguments before any mutation and report existing blockers.
- Q: How does source refresh differ from history refresh? A: A separate Refresh saved source action performs bounded read-only inspection; Refresh imported history reads stored copies. Inspect automatically when opening the workflow or changing environment, with stale-response rejection.
- Q: Should acknowledgment authenticate the supplier? A: No, preserve the existing content-bound confirmation and explain that file validation does not establish origin.
- Q: Should command feedback fall back to debug output? A: No, essential replies await the system-chat route and diagnostics remain separate.
- Q: Does this authorize release or owner merge? A: No, automatic branch/PR publication and at most two requested Codex rounds are authorized; final merge remains with the owner.
