# Feature Specification: Persistent Auto Potion Request

**Feature Branch**: `codex/s097-auto-potion-persistence`

**Created**: 2026-09-15

**Status**: Ready for Planning

**Input**: Work slice S097 implements GitHub issue #172 after S096 completed the persistent data-addon program.

## User Scenarios & Testing

### User Story 1 - Restore the operator's request (Priority: P1)

As an operator, I can enable or disable Auto Potion, close ESO Weave normally,
and have that requested state restored on the next launch.

**Why this priority**: The Auto Potion request is the only desktop application
toggle that currently forgets explicit operator intent across restarts.

**Independent Test**: Toggle Auto Potion through the application model, flush
session state, load it into a fresh model, and verify the request is restored in
both enabled and disabled cases.

**Acceptance Scenarios**:

1. **Given** Auto Potion is enabled, **when** session state is flushed and loaded
   into a fresh model, **then** the request is enabled.
2. **Given** Auto Potion is disabled, **when** session state is flushed and
   loaded into a fresh model, **then** the request remains disabled.
3. **Given** Auto Potion is toggled through either the UI or F3, **when** the
   save-settle interval elapses, **then** both paths persist the same model-owned
   request value.

---

### User Story 2 - Restore without bypassing safety (Priority: P1)

As an operator, I can trust a restored Auto Potion request to remain dormant or
blocked until every existing runtime and telemetry gate positively authorizes an
input attempt.

**Why this priority**: Persistence must not convert remembered intent into a
startup input path independent of the existing safety controller.

**Independent Test**: Restore an enabled request into a fresh model with unknown
or closed startup evidence, verify its effective state is dormant or blocked,
and prove that no input attempt is possible until all existing gates become
eligible.

**Acceptance Scenarios**:

1. **Given** an enabled stored request and default startup evidence, **when** the
   model restores it, **then** Auto Potion reports a truthful dormant or blocked
   state and emits no input.
2. **Given** an enabled stored request, **when** any game, focus, signal, context,
   life, world, travel, movement, resource, quickslot, cooldown, or retry gate is
   closed, **then** the existing controller remains authoritative and emits no
   input.
3. **Given** an enabled stored request and later fully eligible fresh evidence,
   **when** the existing controller evaluates it, **then** it may act under the
   unchanged Auto Potion trigger contract.

---

### User Story 3 - Upgrade old or malformed state safely (Priority: P2)

As an operator upgrading ESO Weave, I retain existing session facts while old
state files default Auto Potion to disabled and malformed state falls back to
the complete safe default.

**Why this priority**: The additive field must preserve previous state without
manufacturing new input authority.

**Independent Test**: Deserialize legacy state without the new field, current
state with true and false values, and malformed values, then verify defaults,
notices, round trips, and preservation of unrelated session facts.

**Acceptance Scenarios**:

1. **Given** a version 1, 2, or 3 state file without the new field, **when** it is
   loaded, **then** Auto Potion defaults to disabled and every existing valid
   field is preserved.
2. **Given** a current state file with a Boolean Auto Potion request, **when** it
   is loaded and saved, **then** the value round-trips exactly.
3. **Given** a malformed Auto Potion request or otherwise invalid state file,
   **when** it is loaded, **then** the complete session falls back to safe
   defaults and a notice is surfaced.

### Edge Cases

- A missing state file starts Auto Potion disabled without a warning.
- Restoring a disabled request must explicitly leave a reused controller off.
- Restoration must not mark state dirty or create a save loop.
- Repeating the same requested value may coalesce a write but must not diverge
  the controller and persisted authority.
- Window geometry, API-version cache, suspension, and Fishing intent must survive
  the schema upgrade unchanged.
- Normal close must flush the latest request even if the settle deadline has not
  elapsed.

## Requirements

### Functional Requirements

- **FR-001**: The session-state model MUST store requested Auto Potion enablement
  as a Boolean separate from configuration and effective runtime state.
- **FR-002**: The current session-state schema version MUST advance to represent
  the new persisted field.
- **FR-003**: State files without the field MUST default it to `false` while
  preserving every other valid field.
- **FR-004**: Missing state MUST default Auto Potion to disabled without a notice.
- **FR-005**: A malformed field MUST make the state invalid, use complete safe
  defaults, and surface the existing invalid-state notice.
- **FR-006**: Saving current state MUST serialize the requested Boolean and retain
  UTF-8 without BOM, LF endings, pretty JSON, and a trailing newline.
- **FR-007**: Model restoration MUST apply the stored request to the existing
  `AutoPotionController`; no parallel authority or startup action path may be
  introduced.
- **FR-008**: Restoration MUST apply `false` as well as `true`, so a reused model
  cannot retain stale enabled state.
- **FR-009**: Restoring the request MUST NOT itself synthesize input, tick the
  controller, relax a gate, reset a gate, or fabricate telemetry.
- **FR-010**: Every existing Auto Potion runtime, telemetry, context, lifecycle,
  cooldown, and retry gate MUST remain unchanged and authoritative.
- **FR-011**: UI and F3 toggle paths MUST both change the model-owned controller
  request and mark session state for coalesced persistence.
- **FR-012**: `current_session_state` and the normal-close flush MUST capture the
  latest controller request.
- **FR-013**: Restoration MUST NOT mark session state dirty merely because a
  stored value was applied.
- **FR-014**: Configuration MUST continue to store Auto Potion thresholds,
  watches, key, and retry interval, but MUST NOT duplicate requested enablement.
- **FR-015**: Canonical Auto Potion, settings, first-launch, configuration,
  status, and troubleshooting prose MUST describe persistence directly and
  remove statements that Auto Potion always starts off or is session-only.
- **FR-016**: Tests MUST cover enabled and disabled round trips, legacy missing
  data, malformed data, UI and F3 convergence, close-time flush, and startup
  fail-closed behavior.
- **FR-017**: The slice MUST NOT change Pixel Bus, addon, input-backend, trigger,
  retry, resource, quickslot, or runtime-gate contracts.

### Key Entities

- **Requested Auto Potion enablement**: The operator-owned Boolean intent stored
  in `state.json` and mirrored by the controller's `enabled` field.
- **Effective Auto Potion state**: The runtime result produced by the existing
  controller after evaluating the request and all safety evidence.
- **Session state**: The versioned runtime-state document that owns suspension,
  Fishing request, Auto Potion request, API cache, and window geometry outside
  `config.json`.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Enabled and disabled Auto Potion requests survive a save, process
  model reconstruction, and restore with exact values.
- **SC-002**: All supported legacy session shapes load with Auto Potion disabled
  and no loss of unrelated valid state.
- **SC-003**: Every malformed request test falls back to disabled and emits a
  notice rather than partially applying the state.
- **SC-004**: A restored enabled request produces zero input under each existing
  closed or unknown authorization condition.
- **SC-005**: UI and F3 integration tests persist the same request value, and a
  close-time flush captures a change made before the settle deadline.
- **SC-006**: Formatting, strict linting, complete tests, release build, and all
  documentation gates pass without warnings or skipped safety coverage.

## Clarification Decisions

- Requested enablement belongs in `state.json`, not `config.json`, because it is
  operator session intent rather than Auto Potion configuration.
- The state schema advances from version 3 to version 4. The field remains
  additive with `false` as the deserialization default, so versions 1 through 3
  remain readable without a bespoke migration function.
- Restoration uses `set_enabled` only. It does not tick, weaken startup defaults,
  or introduce a new input path.
- No installed-client verification issue is required for the repository change.
  The existing issue acceptance calls for a packaged two-state restart smoke
  check, which this slice records as a manual post-build check when practical.

## Scope Boundaries

- No configuration schema change.
- No default-on behavior for a missing, legacy, or malformed state file.
- No change to Auto Potion eligibility, triggering, status vocabulary, retry,
  or synthesized input.
- No PixelBeacon, ESO Weave Data, SavedVariables, addon, or protocol change.
- No broader persistence changes for transient controller state.

## Architectural Deviation

S097 intentionally supersedes S039 research decision R7 and S043 requirement
FR-002, which made Auto Potion enablement session-only and always off at launch.
GitHub issue #172 is the newer product authority and requires the request toggle
to behave like the other persisted operator-intent toggles. The safety rationale
is preserved by restoring only the request into the existing fail-closed
controller. No gate, telemetry requirement, or input authorization is restored
or bypassed.

## Assumptions

- Normal application startup continues to restore session state after model
  construction and before ordinary UI interaction.
- Existing Auto Potion controller tests remain the authority for the trigger
  conjunction and fail-closed evidence behavior.
- State loading continues to reject a malformed document as a whole rather than
  applying valid fields selectively.
