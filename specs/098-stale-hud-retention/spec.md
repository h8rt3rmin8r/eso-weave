# Feature Specification: Stale HUD Retention

**Feature Branch**: `codex/s098-stale-hud-retention`

**Created**: 2026-09-15

**Status**: Ready for Planning

**Input**: Work slice S098 implements GitHub issue #171 after S097 completed Auto Potion request persistence.

## User Scenarios & Testing

### User Story 1 - Inspect the last coherent HUD (Priority: P1)

As an operator, I can briefly inspect the last coherent player-state values after ESO becomes inactive, loses focus, or loses its live signal.

**Why this priority**: A transient loss currently destroys the context an operator opened ESO Weave to inspect.

**Independent Test**: Establish a coherent HUD snapshot, cause each covered loss independently, and verify the same values remain visible with a stale reason and age until the configured deadline.

**Acceptance Scenarios**:

1. **Given** coherent live HUD values and retention above zero, **when** ESO becomes inactive, **then** those values remain visible and are marked stale because the game became inactive.
2. **Given** coherent live HUD values and retention above zero, **when** ESO loses focus, **then** those values remain visible and are marked stale because focus was lost.
3. **Given** coherent live HUD values and retention above zero, **when** PixelBeacon signal is lost, **then** those values remain visible and are marked stale because signal was lost.
4. **Given** a retained snapshot, **when** the monotonic deadline is reached, **then** the snapshot is discarded once and the existing dormant or unavailable presentation appears.

---

### User Story 2 - Recover fresh presentation (Priority: P1)

As an operator, I see newly observed values as soon as the game and signal recover, without waiting for the stale deadline.

**Why this priority**: Retention is useful only if fresh evidence immediately supersedes it.

**Independent Test**: Enter stale presentation, recover before expiry with different coherent values, and verify the new values replace the retained snapshot and remain live after the old deadline.

**Acceptance Scenarios**:

1. **Given** stale values before expiry, **when** fresh coherent observations return, **then** they replace the stale values and cancel the pending wipe.
2. **Given** a later loss after recovery, **when** retention begins again, **then** its deadline is based on the later loss and its snapshot contains the later coherent values.

---

### User Story 3 - Configure bounded retention (Priority: P2)

As an operator, I can choose a whole-second retention interval from 0 through 999, with 120 seconds as the default.

**Why this priority**: Operators need both a useful default and an explicit immediate-clearing mode.

**Independent Test**: Load missing, valid, and invalid UI configuration, edit the numeric control at both bounds, save, reload, and verify exact values and safe fallback.

**Acceptance Scenarios**:

1. **Given** no saved value, **when** settings load, **then** stale retention is 120 seconds.
2. **Given** a value from 0 through 999, **when** settings are saved and reloaded, **then** it round-trips exactly.
3. **Given** an invalid value, **when** settings load, **then** it falls back to 120 and surfaces an invalid-value notice.
4. **Given** retention is zero, **when** any covered loss occurs, **then** existing dormant or unavailable presentation appears immediately.

---

### User Story 4 - Preserve immediate safety response (Priority: P1)

As an operator, I can trust stale display values never to keep weaving, Fishing, Auto Potion, or another input-producing path authorized.

**Why this priority**: Presentation memory must remain incapable of becoming action evidence.

**Independent Test**: Establish retained display values for every loss cause and prove the independent input and controller gates close immediately while the display remains stale.

**Acceptance Scenarios**:

1. **Given** a retained snapshot, **when** its underlying loss occurs, **then** authoritative game, focus, signal, life, world, travel, roll, menu, and controller paths retain their current immediate fail-closed behavior.
2. **Given** visible stale resources or quickslot state, **when** Auto Potion evaluates, **then** it reads only current controller evidence and cannot act from the retained presentation.

### Edge Cases

- A loss before any coherent snapshot shows the existing unavailable or dormant state immediately.
- Changing the configured interval while a snapshot is stale re-evaluates the original loss time and never restarts or extends the deadline.
- Changing from a positive interval to zero clears an active retained snapshot on the next view projection.
- A second loss cause during one stale interval updates the visible cause but does not extend the original deadline.
- Unknown runtime or focus evidence is reported as unavailable rather than mislabeled as a confirmed inactive or unfocused state.
- Closing and reopening ESO Weave starts with no retained HUD snapshot because the cache is never serialized.
- Repeated view projection after expiry remains idempotent and does not recreate stale data.

## Requirements

### Functional Requirements

- **FR-001**: The persisted UI settings section MUST include `stale_retention_seconds` as a whole number from 0 through 999 inclusive.
- **FR-002**: A missing value MUST default to 120 seconds without a notice.
- **FR-003**: A value outside the inclusive range or of the wrong type MUST fall back to 120 seconds and surface an invalid-value notice.
- **FR-004**: Valid values MUST round-trip through the existing settings form and `config.json` without a top-level schema-version change.
- **FR-005**: Settings MUST present a bounded numeric field that supports precise keyboard entry and increment or decrement adjustment.
- **FR-006**: The application MUST retain one in-memory snapshot of the last coherent player-state presentation when a covered loss occurs and retention is above zero.
- **FR-007**: The retained snapshot MUST cover resource, Ultimate, combat, movement, roll-dodge, life, weapon-bar, quickslot, menu, world, travel, and skill-cooldown presentation values.
- **FR-008**: A coherent snapshot MUST be eligible only when runtime is active, focus is positively focused, beacon freshness is fresh, and surface evidence is available.
- **FR-009**: Covered loss causes MUST distinguish game inactive, runtime unavailable, focus lost, focus unavailable, and signal unavailable without inventing evidence.
- **FR-010**: Retention MUST use the `AppModel` injected monotonic clock and one snapshot-level loss timestamp and deadline, not wall time or widget timers.
- **FR-011**: Stale presentation MUST visibly and accessibly state that the values are stale, the current loss cause, and whole-second age while leaving the retained values readable.
- **FR-012**: Fresh coherent observations before expiry MUST replace the snapshot, clear stale status, and cancel the old deadline.
- **FR-013**: At or after expiry, the retained snapshot MUST be discarded once and the existing dormant or unavailable projection MUST be returned.
- **FR-014**: A zero interval MUST bypass retention and return existing dormant or unavailable presentation immediately.
- **FR-015**: Changing a stale interval MUST compute expiry from the original loss timestamp and MUST NOT extend retention by restarting the timer.
- **FR-016**: The retained snapshot and deadline MUST remain process-local and MUST NOT be stored in configuration, session state, logs, or other durable data.
- **FR-017**: Reader routing, `GameState`, `WeaveEngine`, `FishingController`, `AutoPotionController`, `InputEngine`, and every input authorization gate MUST continue to consume only current evidence.
- **FR-018**: Retained presentation types MUST NOT expose decoded telemetry back to controllers or input paths.
- **FR-019**: Tests MUST cover defaults, valid and invalid bounds, exact round trips, every loss cause, no-prior-snapshot behavior, recovery, later loss, expiry idempotence, zero retention, interval changes, and immediate automation separation.
- **FR-020**: Canonical settings, configuration, interface, architecture, test-strategy, and troubleshooting documentation MUST describe stale retention directly and preserve the distinction between presentation and authority.
- **FR-021**: S098 MUST NOT change PixelBeacon, ESO Weave Data, the pixel protocol, sampling cadence, telemetry decoding, controller rules, or input submission.

### Key Entities

- **HUD presentation snapshot**: A process-local clone of already-derived display types. It carries no controller reference and no action-authorizing telemetry API.
- **Stale interval**: The period beginning at the first covered loss and ending at the original loss time plus the current configured duration.
- **Stale cause**: The truthful current reason the live presentation is unavailable.
- **Coherent presentation**: A projection made while runtime, focus, freshness, and surface availability all positively support live display.

## Success Criteria

### Measurable Outcomes

- **SC-001**: The same coherent HUD values remain visible for each covered loss at every tested instant before the configured deadline.
- **SC-002**: The first projection at or after the deadline returns existing dormant or unavailable values, and all later projections remain identical.
- **SC-003**: Fresh observations before expiry replace every retained field and remain live after the superseded deadline.
- **SC-004**: Every stale display test confirms an immediate closed or inactive authoritative input/controller state independent of the retained values.
- **SC-005**: All integers from 0 through 999 are representable, both bounds round-trip exactly, and invalid inputs fall back to 120 with a notice.
- **SC-006**: Formatting, strict linting, complete locked tests, release build, documentation gates, and hosted CI and review complete without unresolved findings.

## Clarification Decisions

- Use the existing numeric `DragValue` pattern with an explicit inclusive range and unit suffix. It supports direct keyboard entry plus bounded increment/decrement controls without adding a custom widget.
- Cache rendered presentation types at the `AppModel` boundary, after all authoritative subsystems have already handled an event. This makes the safety separation structural rather than conditional.
- The first covered loss owns the loss timestamp. A later cause may update the explanation but cannot prolong the interval.
- Age means elapsed stale duration, not time since the most recent cause change.
- Freshness requires available surface evidence as well as heartbeat freshness because an unavailable surface cannot yield a coherent Game Context.

## Scope Boundaries

- No durable live player-state storage.
- No widget-specific timer, background worker, or new clock.
- No change to state clearing inside the weave engine or any controller.
- No change to what constitutes fresh input or automation evidence.
- No addon, protocol, transport, encounter, catalog, or data-store work.

## Assumptions

- The egui application continues to request frames often enough for the visible whole-second age and deadline transition to update.
- Current rendered view types remain display-only and cloneable.
- Existing game process and Pixel Bus loops remain the source of runtime, focus, freshness, and surface observations.
