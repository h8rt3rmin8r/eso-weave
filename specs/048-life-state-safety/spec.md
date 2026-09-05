# Feature Specification: Life State Safety

**Feature Branch**: `codex/048-life-state-safety`

**Created**: 2026-09-05

**Status**: Implemented

**Input**: Issues #53, #54, #55, and #58. Add a truthful player life-state signal,
use it as a hard automation gate, and make the renamed System and State panel an
accessible persisted disclosure.

## Clarifications

All routine choices were resolved under the build-phase autopilot policy.

- Existing and new users start with System and State expanded. A deliberate user
  choice is stored in the UI settings section and restored on the next launch.
- `Alive` is the only actionable life state. `Dead`, `Reincarnating`, `Unknown`,
  missing blocks, corrupt samples, and signal loss all fail closed.
- A blocked physical weave key passes through to the game. A queued weave action
  is discarded at execution. Fishing and potion deadlines are not replayed after
  recovery; each controller must observe a new eligible condition.
- World-transition, roll-dodge, sprint, and effect-database detection remain in
  their existing atomic issues.

## User Scenarios & Testing

### User Story 1 - Never automate while the character cannot act (Priority: P1)

As a player, I want every synthesized-input feature to stop while my character is
dead, reincarnating, or not authoritatively known alive, so the application does
not press keys into invalid or transitional game states.

**Independent Test**: Drive every life-state value through routing and assert that
weaving, fishing, and auto-potion synthesize only when the value is Alive.

**Acceptance Scenarios**:

1. **Given** a fresh Alive signal, **When** an otherwise eligible weave, fishing
   action, or potion trigger occurs, **Then** its existing behavior remains enabled.
2. **Given** Dead, Reincarnating, or Unknown, **When** the same work becomes due,
   **Then** no synthesized key or mouse operation is emitted.
3. **Given** work was blocked by life state, **When** Alive returns, **Then** no
   blocked work is replayed and only a new eligible event can synthesize.
4. **Given** the block is absent, corrupt, or lost with the heartbeat, **When**
   automation evaluates, **Then** it remains blocked as Unknown.

---

### User Story 2 - See the character's truthful life state (Priority: P1)

As an operator, I want the Live HUD to distinguish Alive, Dead, Reincarnating, and
Not detected so I can understand why automation is running or blocked.

**Independent Test**: Decode each wire value and invalid evidence, route it into
the shared model, and assert the matching accessible Live HUD text.

**Acceptance Scenarios**:

1. **Given** ESO reports the player alive, dead, or reincarnating, **When**
   PixelBeacon renders B21, **Then** the companion publishes the matching typed state.
2. **Given** the game is active but B21 is unavailable, **When** the view renders,
   **Then** Life state reads Not detected and automation remains blocked.
3. **Given** the ESO process is inactive, **When** the view renders, **Then** Life
   state uses the existing Game not active dormant presentation.

---

### User Story 3 - Keep system details available without permanent bulk (Priority: P2)

As an operator, I want the renamed System and State panel to collapse elegantly so
future state fields do not permanently crowd the main interface.

**Independent Test**: Activate the disclosure by mouse and keyboard in a headless
rendered frame, persist the preference, recreate the application, and verify the
collapsed and restored layouts.

**Acceptance Scenarios**:

1. **Given** a default configuration, **When** the dashboard opens, **Then** the
   panel is named System and State and begins expanded.
2. **Given** the expanded panel, **When** its full header is activated, **Then**
   every body row is hidden, the chevron and accessibility state change, and the
   dashboard reclaims the height without overlap or blank space.
3. **Given** the user selected collapsed, **When** the application restarts, **Then**
   the panel remains collapsed while Skills remains unchanged.

## Edge Cases

- An older installed addon keeps B21 absent. The companion reports Unknown and
  offers the existing addon Update action through the manifest version bump.
- Death can occur between physical interception and worker execution. Both the
  input classifier and weave worker gate, so the queued action is discarded.
- Reincarnation can transition through an API state before the alive event. The
  periodic re-baseline and activation event converge without assuming event order.
- Signal recovery can deliver Heartbeat before B21. Heartbeat alone never opens
  the life gate.
- Collapsing in the wide layout must not force the Live HUD column to overlap,
  and collapsing in the narrow layout must reduce intrinsic height.

## Requirements

### Functional Requirements

- **FR-001**: The dashboard heading MUST be `System and State` everywhere,
  including visual text, accessibility labels, tests, and documentation.
- **FR-002**: The complete System and State body MUST be controlled by one full-row
  disclosure header with visible open and closed affordances.
- **FR-003**: The disclosure MUST support pointer and keyboard activation and expose
  its label and expanded state to assistive technology.
- **FR-004**: The disclosure MUST default open and MUST persist a deliberate open
  or closed preference in the existing `ui` configuration section.
- **FR-005**: Collapsed layout MUST remove the body from layout, recompute the
  intrinsic extent, and preserve Skills and dashboard breakpoint safety.
- **FR-006**: PixelBeacon MUST publish B21 as a marker- and checksum-validated life
  state with distinct Alive, Dead, and Reincarnating payloads.
- **FR-007**: PixelBeacon MUST derive state from ESO's player death and
  reincarnation primitives, respond to life-cycle events, re-baseline after player
  activation, and retain a periodic convergence backstop.
- **FR-008**: The addon and companion MUST state and test the same block count,
  marker, payload values, and checksum rule.
- **FR-009**: The companion MUST model `Unknown`, `Alive`, `Dead`, and
  `Reincarnating` as one typed value and clear to Unknown on invalid or lost signal.
- **FR-010**: Reader routing MUST deliver every life-state transition to the input
  classifier, weave worker, fishing controller, auto-potion controller, and view model.
- **FR-011**: Only `Alive` MAY authorize synthesis. Every other life state MUST
  block physical weave interception, queued weave execution, fishing timer output,
  and auto-potion output.
- **FR-012**: Toggle hotkeys MUST remain exempt from the physical interception
  gate, matching manual suspend and game-surface gating.
- **FR-013**: A blocked weave action or autonomous deadline MUST NOT be retained for
  replay when Alive returns.
- **FR-014**: Auto-potion and fishing MUST surface a concise life-state blocker
  without changing the operator's requested toggle.
- **FR-015**: Automated tests MUST cover all valid and invalid wire states, signal
  loss, cross-language agreement, event registration, every synthesis path,
  recovery without replay, persisted disclosure behavior, accessibility, and
  narrow and wide dashboard geometry.
- **FR-016**: S048 MUST NOT implement world transition, roll dodge, sprint,
  potion-cooldown enhancement, or effect database behavior.

### Key Entities

- **LifeState**: Unknown, Alive, Dead, or Reincarnating. Only Alive is actionable.
- **Life gate**: The derived `LifeState != Alive` decision enforced at each
  synthesis boundary.
- **B21 life block**: The marker, red payload, and blue complement that transport
  the authoritative addon state.
- **System and State disclosure**: The persisted, accessible open or closed UI
  preference and its body layout.

## Success Criteria

- **SC-001**: All four non-Alive or unavailable cases emit zero operations from
  weave, fishing, and auto-potion tests.
- **SC-002**: Recovery tests emit zero stale operations and resume only after a new
  eligible action or observation.
- **SC-003**: All valid B21 values round-trip and every corrupt marker, checksum,
  or payload decodes to Unknown.
- **SC-004**: The addon and companion contract test agrees on exactly 22 payload blocks.
- **SC-005**: Accessibility tests find one System and State disclosure in both
  expanded and collapsed states, and preference round-trip tests preserve the state.
- **SC-006**: Existing Skills behavior and all prior input safety tests remain green.

## Assumptions

- The supported ESO API provides `IsUnitDead`, `IsUnitReincarnating`,
  `EVENT_PLAYER_DEAD`, `EVENT_PLAYER_ALIVE`, and `EVENT_PLAYER_ACTIVATED`.
- Reincarnating takes precedence over dead when both primitives are true during a
  transition.
- The negotiated layout introduced in S045 has capacity for one additional payload
  block without protocol-version change.
