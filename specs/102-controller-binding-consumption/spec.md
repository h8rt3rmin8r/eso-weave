# Feature Specification: Controller Binding Consumption

**Feature Branch**: `codex/s102-controller-binding-consumption`

**Created**: 2026-09-15

**Status**: Implemented

**Input**: Work slice S102 implements GitHub issue #208 and completes parent issue #188.

## User Scenarios & Testing

### User Story 1 - Fish with ESO's current Interact chord (Priority: P1)

As an operator, Fishing casts, reels, and recasts with the validated Interact chord currently configured in ESO, including supported keyboard, mouse, and modifier chords.

**Why this priority**: A guessed Interact key can act on the wrong game control and violates the native-authority boundary.

**Independent Test**: Publish valid Interact chords, run every Fishing emission path, and verify each complete attempt uses only the published immutable chord.

**Acceptance Scenarios**:

1. **Given** a valid Interact chord, **when** Fishing casts, reels, or recasts, **then** it synthesizes that chord once with balanced ownership.
2. **Given** Interact is rebound after work is scheduled, **when** the deadline arrives, **then** the expired work emits no primary down event.
3. **Given** Interact evidence is non-valid, **when** Fishing is requested, **then** Fishing stays disabled and presents binding remediation.

### User Story 2 - Drink with ESO's current Quickslot chord (Priority: P1)

As an operator, Auto Potion activates the validated Quickslot chord currently configured in ESO and records a retry attempt only when the chord primary was actually emitted.

**Why this priority**: Auto Potion acts without a physical trigger, so authoritative targeting and truthful retry accounting are safety-critical.

**Independent Test**: Drive a low-resource trigger with each valid chord form and every non-valid binding state, then verify output, state, and retry timestamps.

**Acceptance Scenarios**:

1. **Given** all potion gates and a valid Quickslot chord, **when** a watched resource crosses its threshold, **then** exactly one complete Quickslot chord is attempted.
2. **Given** Quickslot is unavailable, unbound, conflicting, or unsupported, **when** the rule would fire, **then** no input is emitted, no retry time is consumed, and the state explains the binding block.
3. **Given** binding or safety authority changes during admission, **when** synthesis is reached, **then** no later primary down starts.

### User Story 3 - Remove duplicate gameplay controls (Priority: P1)

As an operator, settings show detected Interact and Quickslot authority without offering duplicate desktop gameplay-key controls, while old configuration files migrate safely.

**Why this priority**: Leaving the controls editable preserves two competing sources of truth and makes the runtime behavior misleading.

**Independent Test**: Load legacy configuration containing `interact_key` and `quickslot_key`, save it, inspect the settings interface, and verify the obsolete fields disappear while all timing and threshold values remain.

**Acceptance Scenarios**:

1. **Given** a legacy configuration, **when** it loads and saves, **then** obsolete gameplay-key fields are ignored and omitted without preventing startup.
2. **Given** current binding evidence, **when** settings are displayed, **then** read-only Interact and Quickslot states identify valid chords or actionable non-valid states.
3. **Given** S102 completes, **when** documentation and examples are inspected, **then** none instructs the operator to configure a duplicate Fishing or Auto Potion gameplay key.

### Edge Cases

- Signal loss publishes unavailable evidence before controller locks and invalidates scheduled work.
- A valid binding may use a keyboard primary, mouse button, or wheel direction with zero or more portable modifiers.
- Physically held modifiers must be a subset of the target chord; an extra physical modifier rejects the attempt rather than releasing user input.
- Generated modifiers press in canonical order and release in reverse order, including after primary synthesis failure.
- Momentary wheel primaries have no invented up event.
- A binding can change between initial availability display, controller admission, and backend synthesis; the authorization epoch closes each race.
- A failed primary down does not consume Auto Potion retry time.
- F1, F2, and F3 remain desktop-owned toggles and are outside native gameplay binding authority.

## Requirements

### Functional Requirements

- **FR-001**: Fishing MUST resolve every generated action from the current valid native Interact binding.
- **FR-002**: Auto Potion MUST resolve every generated action from the current valid native Quickslot binding.
- **FR-003**: Neither controller may use a default, legacy, or guessed gameplay key when native evidence is unavailable, unbound, conflicting, unsupported, stale, or replaced.
- **FR-004**: Binding replacement and signal loss MUST invalidate admitted autonomous work before any later primary down event.
- **FR-005**: Controller chord execution MUST support the complete portable keyboard and mouse primary vocabulary published by S100 on Windows and Linux.
- **FR-006**: Controller chord execution MUST preserve physical modifier ownership, balance every generated down with cleanup, and avoid an up event for momentary primaries.
- **FR-007**: Auto Potion MUST record an attempt and retry interval only after a Quickslot primary down succeeds.
- **FR-008**: Fishing and Auto Potion MUST expose truthful blocked or stopped presentation when required native authority cannot admit synthesis.
- **FR-009**: Persistence MUST ignore legacy `fishing.interact_key` and `potion.quickslot_key` fields and MUST omit them on the next save while preserving all unrelated values.
- **FR-010**: Settings MUST replace the two editable gameplay-key controls with read-only native binding status and remediation.
- **FR-011**: Canonical documentation, troubleshooting, captures, build-plan chronology, and release notes MUST describe ESO as the only gameplay binding authority.
- **FR-012**: Existing gameplay, focus, suspension, menu, life, world, and travel gates MUST remain authoritative for both controllers; Fishing MUST continue to disable on signal loss.
- **FR-013**: Application toggles F1, F2, and F3 MUST remain independently configurable and behaviorally unchanged.

### Key Entities

- **Autonomous binding authority**: Shared snapshot access, modifier state, safety gates, and monotonic generation used by Fishing and Auto Potion.
- **Native controller chord**: Immutable primary and normalized modifier set copied from one valid action state.
- **Legacy controller setting**: An obsolete persisted gameplay key accepted only for migration and never used as authority.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All three Fishing emission paths and all Auto Potion trigger causes pass tests using valid keyboard, mouse, and modified native chords.
- **SC-002**: Every non-valid Interact and Quickslot state produces zero primary down events in tests.
- **SC-003**: Binding replacement and signal-loss race tests produce zero post-invalidation primary down events.
- **SC-004**: Legacy configuration loads successfully and its next serialized form contains neither obsolete gameplay-key field.
- **SC-005**: The settings surface contains zero editable gameplay binding controls and shows both detected binding states.
- **SC-006**: Formatting, strict linting, complete locked tests, release build, documentation, trust, spelling, links, encoding, and hosted CI all pass.

## Assumptions

- S100 remains the sole producer of coherent binding evidence and S101's native vocabulary remains the platform contract.
- The shared controller authorization excludes roll-dodge because existing Fishing and Auto Potion policy does not use roll state as an authority.
- Legacy fields are silently ignored during migration because their values are no longer actionable and startup must remain safe.
- This slice closes issue #208 and parent #188 after delivery; no release is part of S102.
