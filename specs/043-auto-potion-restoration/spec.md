# Feature Specification: Auto-potion Restoration

**Feature Branch**: `codex/043-auto-potion-restoration`

**Created**: 2026-09-03

**Status**: Ready for Planning

**Input**: Work slice S043 implementing GitHub issue #25 after the S041 game-state and S042 quickslot foundations.

## User Scenarios & Testing

### User Story 1 - Trigger a verified potion safely (Priority: P1)

As a player, I can request auto-potion and have ESO Weave submit the configured quickslot input only when every safety precondition is currently satisfied.

**Why this priority**: The feature currently does not perform its primary function, while an unsafe false-positive input would be worse than no input.

**Independent Test**: Drive the controller with deterministic game, focus, beacon, menu, resource, quickslot, cooldown, and retry states, then verify that exactly one key-down and one key-up event occur only for the fully eligible combination.

**Acceptance Scenarios**:

1. **Given** auto-potion is requested, the game and beacon are active, the game is focused, input is permitted, a watched resource is freshly below its threshold, and the selected quickslot is an explicitly usable potion with no cooldown, **When** the controller evaluates the state, **Then** it submits exactly one complete quickslot input attempt and enters a bounded Triggered state.
2. **Given** any required precondition is missing, unknown, stale, or unsafe, **When** the controller evaluates the state, **Then** it submits no input.
3. **Given** an input attempt just occurred, **When** the controller evaluates again before the retry interval expires, **Then** it submits no duplicate input and reports the retry blocker.

---

### User Story 2 - See the truthful effective state (Priority: P2)

As a player, I can distinguish my requested auto-potion setting from its current effective runtime state and understand why it is not acting.

**Why this priority**: A single On or Off label conceals blockers and made the broken end-to-end path difficult to diagnose.

**Independent Test**: Feed every controller state into the application view model and verify that it produces a distinct, concise status for Off, dormant, blocked, Ready, and Triggered conditions without requiring verbose logs.

**Acceptance Scenarios**:

1. **Given** the user has not requested auto-potion, **When** the status is rendered, **Then** it reads Off.
2. **Given** auto-potion is requested but the game is inactive or unfocused, **When** the status is rendered, **Then** it identifies the corresponding dormant reason without clearing the request.
3. **Given** auto-potion is requested but a runtime precondition blocks input, **When** the status is rendered, **Then** it identifies the current blocker, including beacon unavailable, input suspended, game context blocked, no watched resource, watched resources unavailable, quickslot unavailable, no potion selected, selected potion unavailable, potion cooldown, or retry interval.
4. **Given** every safety precondition except a low-resource threshold is satisfied, **When** the status is rendered, **Then** it reads Ready.
5. **Given** an input attempt was submitted, **When** the status is rendered before the next evaluation, **Then** it reads Triggered and identifies the resource that caused the attempt.

---

### User Story 3 - Recover cleanly across lifecycle changes (Priority: P3)

As a player, I can leave or refocus the game without the application forgetting that I requested auto-potion or sending input during an unsafe transition.

**Why this priority**: Game, focus, launcher, and beacon transitions are normal operation and must fail closed without turning a temporary condition into a settings change.

**Independent Test**: Toggle game activity, focus, beacon availability, menu gates, and suspension around a requested controller, then verify that the request remains selected, effective state follows the lifecycle, and input resumes only after fresh eligible observations return.

**Acceptance Scenarios**:

1. **Given** auto-potion is requested, **When** the game exits or the beacon signal is lost, **Then** the request remains selected and input is blocked immediately.
2. **Given** the game or beacon later returns, **When** fresh eligible observations arrive, **Then** the controller can become Ready or Triggered without requiring the user to re-enable it.
3. **Given** the game loses focus, a blocking menu opens, or application input is suspended, **When** a low resource is observed, **Then** no input is submitted and the effective state names that blocker.
4. **Given** the effective state changes, **When** normal operational logging is enabled, **Then** one change-only diagnostic is emitted without logging raw gameplay data above debug level.

### Edge Cases

- If multiple watched resources are low simultaneously, the controller selects a deterministic resource cause while still submitting only one input attempt.
- If one watched resource is unavailable and another fresh watched resource is above threshold, the feature remains Ready but does not trigger.
- If all watched resources are unavailable, the feature reports Resources unavailable and does not trigger.
- If no resources are selected for watching, the feature reports No watched resource and does not trigger.
- Unknown, empty, non-potion, depleted, blocked, or otherwise unusable quickslot observations must never authorize input.
- Cooldown readiness remains independent from item identity. A non-potion item with zero cooldown cannot authorize input.
- Key submission is a complete down/up pair. The retry interval begins with the submitted attempt so repeated evaluations cannot produce an input storm.
- A signal loss after the request is enabled must not mutate the request itself.

## Requirements

### Functional Requirements

- **FR-001**: The system MUST keep requested enablement separate from the effective auto-potion runtime state.
- **FR-002**: The requested setting MUST default to Off for every application launch and MUST remain session-only.
- **FR-003**: Temporary loss of game activity, focus, beacon availability, permitted game context, or input availability MUST block input without clearing requested enablement.
- **FR-004**: The controller MUST submit quickslot input only when requested enablement is On, the game is active and focused, the beacon signal is available, input is not suspended, the current game context permits input, the retry interval has elapsed, at least one watched resource is fresh and below threshold, and the selected quickslot is explicitly classified as a usable potion with cooldown ready.
- **FR-005**: Missing, stale, unknown, ambiguous, or unsupported observations MUST fail closed and MUST NOT synthesize input.
- **FR-006**: Each eligible evaluation MUST submit at most one key-down and one key-up event using the existing configured quickslot binding and input backend.
- **FR-007**: The controller MUST enforce a minimum retry interval after every submitted attempt.
- **FR-008**: The effective state MUST distinguish Off, dormant game inactive, dormant unfocused, blocked beacon unavailable, blocked input suspended, blocked game context, blocked no watched resource, blocked resources unavailable, blocked quickslot unavailable, blocked no potion selected, blocked selected potion unavailable, blocked potion cooldown, blocked retry interval, Ready, and Triggered.
- **FR-009**: Ready MUST mean the required runtime preconditions are satisfied and at least one watched resource is fresh, but no fresh watched resource is currently below its configured threshold.
- **FR-010**: Triggered MUST be a bounded transient state that identifies a deterministic resource, observed percentage, and configured threshold associated with the submitted attempt.
- **FR-011**: The main application view MUST show both requested enablement and the concise effective state or blocker without requiring verbose logging.
- **FR-012**: Operational diagnostics MUST be emitted only when effective state changes, and raw gameplay observations MUST remain debug-only.
- **FR-013**: Existing Health, Magicka, Stamina, quickslot classification, cooldown, menu gate, focus, suspension, and game-state contracts MUST remain the authoritative inputs rather than introducing parallel detection logic.
- **FR-014**: Automated tests MUST cover the complete trigger conjunction, every blocker category, lifecycle recovery, request preservation, retry behavior, deterministic trigger cause, and exact down/up input sequence.
- **FR-015**: The implementation MUST retain the current Skills behavior and MUST NOT change the pixel-beacon protocol or addon contract.

### Key Entities

- **Auto-potion request**: The session-only user preference indicating whether automatic potion use is desired.
- **Effective auto-potion state**: The truthful current outcome of evaluating lifecycle, safety, observation, cooldown, and retry conditions.
- **Trigger cause**: The watched resource, fresh observed percentage, and configured threshold that authorized a submitted attempt.
- **Input attempt**: One complete quickslot key-down and key-up submission followed by retry suppression.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Deterministic tests exercise every required blocker and show zero synthesized input for all ineligible or unknown combinations.
- **SC-002**: The fully eligible deterministic scenario produces exactly one key-down and one key-up event, followed by retry suppression.
- **SC-003**: Game exit, focus loss, beacon loss, menu gating, and input suspension preserve requested enablement while immediately preventing input.
- **SC-004**: Every effective state has a user-visible status, and each state transition produces no more than one normal-level diagnostic.
- **SC-005**: The repository formatting, lint, unit, integration, and documentation gate passes with zero warnings.
- **SC-006**: Post-release Windows client verification confirms Health, Magicka, and Stamina threshold crossings each trigger once when eligible, while empty, non-potion, depleted, cooldown, menu, focus, and signal-loss cases produce no input.

## Assumptions

- S041 remains the source of truth for game activity and focus.
- S042 remains the source of truth for explicit quickslot identity, usability, and cooldown.
- The existing configured quickslot binding and input backend remain appropriate for S043.
- User-requested enablement remains intentionally non-persistent and defaults to Off on every launch.
- Fresh-release field verification for issue #24 is deferred at the user's direction. The equivalent real-client matrix in SC-006 is also deferred until a build containing S043 is released, so this pull request will reference rather than automatically close issue #25.

## Scope Boundaries

- No pixel-beacon protocol or addon changes.
- No new process, launcher, memory-reading, or accessibility detection paths.
- No persistent auto-potion enablement.
- No redesign of the Skills section or broader main-view layout.
- No release publication or claim of completed real-client verification.

## Architectural Deviation

The original S039 behavior cleared the enabled setting when beacon input was lost. S043 intentionally replaces that behavior because a temporary lifecycle failure is not a user preference change. The controller now preserves requested enablement and exposes signal loss as an effective blocker. This is a corrective deviation required for truthful state, lifecycle recovery, and downstream diagnostics.
