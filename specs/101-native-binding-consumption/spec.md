# Feature Specification: Native Binding Consumption

**Feature Branch**: `codex/s101-native-binding-consumption`

**Created**: 2026-09-15

**Status**: Implemented

**Input**: Work slice S101 implements GitHub issue #207, the combat-input child of parent issue #188.

## User Scenarios & Testing

### User Story 1 - Weave from current ESO controls (Priority: P1)

As an ESO Weave operator, I can rebind a skill, Ultimate, Synergy, Attack, or Block in ESO and have weaving use the validated native chord without repeating that gameplay binding in desktop settings.

**Why this priority**: Duplicate and hardcoded controls can silently direct automation to the wrong action, which breaks the core weaving promise.

**Independent Test**: Publish coherent valid binding sets containing keyboard, mouse, and modifier chords, then trigger each active combat action and verify the generated operation sequence uses only those chords.

**Acceptance Scenarios**:

1. **Given** current valid skill and Attack chords, **when** the exact active skill chord is pressed, **then** the selected weave sequence uses the detected Attack and skill chords.
2. **Given** a bash or block-casting weave and current valid Block evidence, **when** the exact skill chord is pressed, **then** the sequence uses the detected Block chord instead of assuming the secondary mouse button.
3. **Given** any migrated action is rebound while the application is running, **when** the new coherent evidence arrives, **then** the old chord immediately stops being intercepted and later work uses only the new chord.
4. **Given** a migrated skill is inactive, **when** its exact native chord is pressed, **then** the physical chord passes through without generated input.

---

### User Story 2 - Fail closed on incomplete authority (Priority: P1)

As an operator, I can rely on combat automation to remain dormant whenever required native binding evidence is missing, stale, ambiguous, unsupported, or unsafe with my currently held modifiers.

**Why this priority**: Guessing or falling back at an input boundary can suppress the intended action or synthesize a different action.

**Independent Test**: Exercise every non-valid state, signal loss, protocol incompatibility, binding-set replacement, focus transition, and held-modifier conflict and verify no new generated down event occurs.

**Acceptance Scenarios**:

1. **Given** unavailable, unbound, conflicting, or unsupported evidence for a requested skill or a chord required by its weave type, **when** the user presses the physical skill chord, **then** the event passes through and no weave is queued.
2. **Given** a sequence has been queued or started, **when** binding evidence changes or becomes unavailable, **then** its authorization expires, no further generated down event starts, and every generated held control is released.
3. **Given** the user physically holds a modifier that a generated target chord does not contain, **when** a skill trigger is evaluated, **then** the trigger passes through because execution would require releasing a user-owned modifier.
4. **Given** a binding snapshot predates the current pixel-bus contract or loses freshness, **when** combat input is evaluated, **then** it is treated as unavailable rather than as a default binding.

---

### User Story 3 - Preserve modifier and toggle ownership (Priority: P1)

As an operator, I can use chorded keyboard and mouse bindings without the application stranding controls, releasing a modifier I physically hold, or changing the independent F1, F2, and F3 application toggles.

**Why this priority**: Input ownership errors can leave the game in a logically held state, while the three desktop toggles are the operator's safety controls.

**Independent Test**: Run platform-neutral ownership tests and Windows/Linux mapping tests across normal completion, cancellation after each down event, synthesis failure, focus loss, and user-held modifier combinations. Separately run all toggle binding regressions.

**Acceptance Scenarios**:

1. **Given** a generated chord needs modifiers the user is not holding, **when** it executes, **then** modifiers press in canonical order and only those generated modifiers release in reverse order.
2. **Given** the user physically holds a modifier also required by a generated chord, **when** the generated chord completes or cancels, **then** that user-held modifier is never released by the application.
3. **Given** focus or another existing safety authority closes during a generated chord, **when** cleanup runs, **then** only application-owned held controls are released and no later down event starts.
4. **Given** migrated combat settings are removed, **when** the settings interface and persistence are used, **then** F1 Suspend, F2 Fishing, and F3 Auto Potion remain independently configurable with their existing exemption and collision behavior.

### Edge Cases

- A physical modifier press or release by itself is never treated as a combat action primary.
- Exact matching requires the physical modifier set at primary-down time to equal the detected trigger chord. Extra or missing modifiers prevent interception.
- A mouse wheel direction is a momentary primary with no meaningful release. It can trigger or be generated once, but cleanup never invents an opposite wheel event.
- A repeated key-down for one already-held primary is suppressed only after the initial press was committed to a weave. It never queues a duplicate sequence.
- A release corresponding to a passed-through press also passes through after focus, activity, or binding state changes.
- Multiple migrated actions resolving to the same exact native chord are unsafe at the desktop boundary even if the addon reported each action independently as valid. The affected chord is not intercepted.
- A target chord is incompatible with current physical state when any physically held modifier is absent from that target. The application does not synthesize an up event for that modifier.
- A synthesis failure after any generated down event triggers best-effort reverse-order cleanup and prevents cooldown accounting for a sequence that emitted no primary action.
- Signal loss and binding evidence replacement invalidate both queued and running combat work before controller locks are acquired.
- Interact and Quickslot evidence remain observation-only until S102 and cannot affect S101 weaving or the F2/F3 application toggles.

## Requirements

### Functional Requirements

- **FR-001**: The input engine MUST consume the current coherent native binding set for skill slots 1 through 5, Ultimate, Synergy, Attack, and Block.
- **FR-002**: A migrated combat action MUST be interceptable only from its exact valid native primary and normalized modifier set.
- **FR-003**: The engine MUST support every portable keyboard primary and mouse primary delivered by the S100 evidence contract on both Windows and Linux, including momentary wheel directions.
- **FR-004**: The engine MUST pass physical input through and MUST NOT queue combat work when the requested skill, Attack, or weave-type-required Block chord is unavailable, unbound, conflicting, unsupported, stale, incompatible, or duplicated across migrated actions.
- **FR-005**: Light and heavy attack plans MUST require valid skill and Attack chords. Bash plans MUST require valid skill, Attack, and Block chords. Block-casting plans MUST require valid skill and Block chords.
- **FR-006**: A binding update, binding loss, focus loss, suspension, menu gate, life gate, roll-dodge gate, world gate, travel gate, or game-process loss MUST invalidate already queued and running weave authorization before new generated down events.
- **FR-007**: Generated chords MUST press missing modifiers in one documented canonical order and release application-owned modifiers in reverse order.
- **FR-008**: Generated input MUST NOT release a modifier that is physically held by the user.
- **FR-009**: Before every generated chord begins, the executor MUST reject a physical modifier set containing a modifier absent from the target chord.
- **FR-010**: The executor MUST track every generated held primary and modifier and perform best-effort reverse-order cleanup on completion, cancellation, synthesis failure, or sink teardown.
- **FR-011**: The original physical primary and its corresponding release MUST either both pass through or both remain suppressed for one committed trigger lifecycle.
- **FR-012**: Physical auto-repeat MUST NOT queue duplicate weave work.
- **FR-013**: Self-originated Windows events and structurally separated Linux virtual-device events MUST remain outside physical trigger classification.
- **FR-014**: Windows interception MUST cover supported keyboard and mouse primaries, and Windows synthesis MUST cover every supported portable control and modifier.
- **FR-015**: Linux interception MUST cover supported keyboard and mouse primaries from the selected physical devices, preserve unrelated grabbed events, and synthesize every supported portable control and modifier through the virtual device.
- **FR-016**: Linux startup MUST fail closed if the required keyboard or pointing-device interception surface cannot be acquired without recursion.
- **FR-017**: The persisted desktop binding table and settings interface MUST retain only Toggle Suspend, Toggle Fishing, and Toggle Auto Potion. Legacy migrated combat entries MUST be discarded without becoming runtime fallbacks.
- **FR-018**: F1, F2, and F3 defaults, rebinding, collision rejection, suspend exemption, and independent GUI-intent routing MUST remain unchanged.
- **FR-019**: Skill-slot configuration MUST no longer contain a duplicated physical key. Display labels MUST not present hardcoded combat bindings.
- **FR-020**: S101 MUST NOT consume Interact or Quickslot evidence and MUST NOT change Fishing or Auto Potion gameplay synthesis.
- **FR-021**: Pixel-bus binding events and signal loss MUST reach the lock-free binding safety gate before any controller lock is acquired.
- **FR-022**: Tests MUST cover chord planning, exact matching, ambiguity, pass-through ownership, every cancellation boundary, synthesis failure, stale evidence, all existing gates, and portable control mappings on Windows and Linux.
- **FR-023**: User guidance, input architecture, build-plan chronology, and changelog documentation MUST describe native combat ownership, remediation, and the deferred S102 consumers.
- **FR-024**: No ESO binding mutation API, custom binding action, `Bindings.xml`, SavedVariables binding persistence, or guessed desktop gameplay default may be introduced.

### Key Entities

- **Physical control event**: One real keyboard or mouse primary transition plus origin, with physical modifier state maintained independently.
- **Native combat bindings**: The current coherent subset of S100 evidence used for seven triggers and Attack/Block synthesis.
- **Executable weave plan**: An immutable skill, Attack, and optional Block chord set selected from one evidence generation for one weave type.
- **Binding authorization generation**: A monotonic value that invalidates queued and running work whenever native binding authority changes or closes.
- **Input ownership ledger**: The generated controls currently held by the application, distinct from modifiers physically held by the user.
- **Toggle binding table**: The desktop-owned configuration for F1 Suspend, F2 Fishing, and F3 Auto Potion only.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Every supported keyboard and mouse primary maps bidirectionally on Windows and Linux, with exhaustive registry tests and no unmapped S100 control.
- **SC-002**: Valid single-key, modifier-keyboard, modifier-mouse, and wheel plans produce the exact detected chords for all four weave types.
- **SC-003**: Every non-valid binding state, cross-action duplicate, stale snapshot, incompatible held modifier, and existing safety-gate closure produces zero unauthorized generated down events.
- **SC-004**: Cancellation injected after every generated down event leaves the application ownership ledger empty and never releases a user-owned modifier.
- **SC-005**: Legacy combat binding fields disappear after one settings save while the three application toggles retain their prior values and behavior.
- **SC-006**: Formatting, strict linting, complete locked tests, release build, documentation gates, hosted CI, and both authorized automated review rounds complete without unresolved findings.

## Clarification Decisions

- Physical trigger matching is exact. Additional modifiers are not ignored because that would make one chord activate another.
- The application never synthesizes a release for a physically held modifier. If a target chord cannot execute under the current physical set, the trigger remains a normal pass-through ESO input.
- Modifier ownership is evaluated before each target chord, not only once per sequence, so user input changes cannot silently broaden authority.
- Canonical generated modifier order is Control, Alt, Shift, Command, with reverse-order release. The order is stable across both platforms.
- Cross-action duplicate chords are rejected by the desktop even though S100 validates each action cell independently.
- Wheel directions are momentary controls. One wheel event represents one down-like activation and has no cleanup release.
- Migrated combat entries in existing settings are discarded. They are not retained as hidden fallback configuration.
- S101 closes child issue #207 only. Interact, Quickslot, their UI status, and final parent-issue closure remain in #208.

## Scope Boundaries

- No ESO binding is created, modified, reset, cleared, or persisted by the addon or desktop.
- No Interact or Quickslot controller consumes native evidence.
- No Fishing or Auto Potion gameplay control setting is removed in S101.
- No new user-selectable combat binding editor or fallback key is introduced.
- No gamepad, hold-key, synthesized ESO chord action, or variable-width binding transport is introduced.
- No timing, cooldown, combat-state, resource, or potion-policy behavior changes.

## Assumptions

- S100 protocol 6 evidence remains the only binding authority and reports unavailable on freshness loss.
- Windows low-level keyboard and mouse hooks and Linux evdev/uinput remain the supported interception surfaces.
- An operator who presses an incompatible modifier combination prefers normal ESO pass-through over application intervention.
