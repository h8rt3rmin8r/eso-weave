# Feature Specification: Quickslot Observation Reconstruction

**Feature Branch**: `codex/042-quickslot-reconstruction`

**Created**: 2026-09-03

**Status**: Ready

**Input**: GitHub issue #24, reconstruct selected quickslot detection from ESO first principles

## User Scenarios & Testing *(mandatory)*

### User Story 1 - See the selected quickslot truthfully (Priority: P1)

As an operator, I can distinguish an unavailable observation, an empty wheel slot,
a selected non-potion, and a selected potion so the status view never implies that
an ambiguous cooldown means a potion is present.

**Why this priority**: Auto-potion cannot be made reliable until the selected slot
has a positive, explicit, and fail-closed potion classification.

**Independent Test**: Feed representative protocol samples for every discriminant
through the reader and confirm the normalized view presents a distinct state without
causing any input.

**Acceptance Scenarios**:

1. **Given** a fresh selected slot containing a usable potion, **When** the addon publishes it, **Then** the application reports Potion, its availability, identity when readable, and Ready or Remaining cooldown.
2. **Given** an empty selected slot, **When** it is observed, **Then** the application reports Empty rather than an unknown cooldown.
3. **Given** an item, collectible, quest item, emote, quick chat, or other supported wheel entry that is not a potion, **When** it is selected, **Then** the application reports Non-potion with a bounded kind and never reports a usable potion.
4. **Given** an invalid, stale, unsupported, partial, or corrupt observation, **When** it reaches the reader, **Then** the application reports Unavailable with a reason class and treats it as non-actionable.

---

### User Story 2 - Diagnose the publisher one predicate at a time (Priority: P1)

As a maintainer testing in ESO, I can request a bounded diagnostic snapshot that
shows each non-localized primitive used to classify the selected quickslot and the
final published state.

**Why this priority**: The current opaque function maps every failed predicate to
the same value, which prevents evidence-driven diagnosis of the field failure.

**Independent Test**: Invoke the diagnostic command in a client and verify one
bounded snapshot contains the selected index, category availability, slot type,
bound ID, link presence, item type, count, on-use ability flag, usability, cooldown
tuple, and final discriminant without including item descriptions.

**Acceptance Scenarios**:

1. **Given** diagnostics are off, **When** unchanged periodic updates run, **Then** no diagnostic lines are emitted.
2. **Given** the operator requests a snapshot, **When** the selected slot is sampled, **Then** one bounded, non-localized diagnostic receipt is emitted.
3. **Given** an observation changes while opt-in change logging is enabled, **When** it converges, **Then** one new receipt is emitted and unchanged ticks produce no duplicates.

---

### User Story 3 - Recover after game and signal transitions (Priority: P2)

As an operator, quickslot status updates after selection, contents, inventory,
cooldown, loading, or signal transitions without requiring an application restart.

**Why this priority**: A correct classification that can remain stale is unsafe for
the downstream automation consumer.

**Independent Test**: Exercise event-driven updates, periodic re-baselining, partial
reads, signal loss, and recovery with deterministic fixtures and confirm one state
transition per effective change.

**Acceptance Scenarios**:

1. **Given** a potion remains selected, **When** its cooldown starts and ends, **Then** the state moves from Ready to Remaining and back to Ready without a slot swap.
2. **Given** the selected wheel slot or its contents changes, **When** ESO emits the corresponding event, **Then** the published state converges before the periodic backstop is needed.
3. **Given** beacon samples become stale or unreadable, **When** freshness expires, **Then** the previous positive potion state is cleared.
4. **Given** an older addon publishes only the legacy quickslot blocks, **When** the new reader samples them, **Then** it reports a legacy-protocol unavailable reason and never upgrades the legacy signal into a positive potion state.

### Edge Cases

- A potion item can have a positive stack but still be unusable.
- A depleted potion can keep an identity while remaining non-actionable.
- Identity bytes may be partial or corrupt while the explicit state block is valid.
- A slot can change between calls during one sampling pass; internally inconsistent facts must fail closed instead of being combined into a positive state.
- A global cooldown can be reported separately from a potion-specific cooldown.
- ESO may add a new wheel action type; it must map to a bounded Other kind.
- Gamepad and keyboard modes must use the same underlying selected-slot facts.
- Addon hidden, absent, loading, or old-version samples must not preserve a stale Potion/Ready pair.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The publisher MUST classify the selected quickslot from separate slot type, bound identity, item-link presence, item type, stack count, usability, and cooldown facts.
- **FR-002**: The protocol MUST carry a dedicated quickslot discriminant independent of the cooldown payload.
- **FR-003**: The normalized model MUST represent Unavailable with a bounded reason class, Empty, Non-potion with a bounded kind, and Potion with availability plus Ready or Remaining cooldown.
- **FR-004**: Potion presence MUST NOT be inferred from a successfully decoded cooldown or from a numeric identity.
- **FR-005**: Unknown, unsupported, stale, inconsistent, old-protocol, and corrupt observations MUST remain non-actionable.
- **FR-006**: Empty, non-potion, unavailable, depleted potion, blocked potion, and usable potion MUST remain visibly distinct in diagnostics and the normalized view.
- **FR-007**: A potion identity MUST be exposed only when every identity byte is valid, and a missing identity MUST NOT change a proven potion classification.
- **FR-008**: The publisher MUST provide an opt-in bounded diagnostic snapshot covering every stage named in User Story 2 without localized descriptions.
- **FR-009**: Normal publisher diagnostics MUST be off or change-only so 100 ms and 1 s updates cannot flood chat or logs.
- **FR-010**: Updates MUST react to active quickslot, hotbar slot, hotbar slot state, cooldown, inventory, and player activation signals, with a periodic re-baseline as recovery.
- **FR-011**: Cooldown MUST count down and return to Ready without requiring the selected slot to change.
- **FR-012**: Loss of freshness or block integrity MUST clear any prior positive potion state.
- **FR-013**: Addon and Rust constants, state codes, markers, geometry, and checksums MUST be cross-checked by automated tests.
- **FR-014**: Decoder tests MUST cover every valid discriminant, marker or checksum failure, partial identity, and the old-addon compatibility path.
- **FR-015**: Event and periodic convergence MUST emit at most one application event and one normal diagnostic entry per effective state change.
- **FR-016**: S042 MUST NOT enable or loosen auto-potion behavior. Issue #25 will consume the reconstructed contract in S043.
- **FR-017**: A real-client verification receipt MUST identify the first predicate that failed in the previous pipeline and cover the issue's field matrix before #24 is closed.
- **FR-018**: Existing health, stamina, magicka, combat, movement, menu, cooldown, fishing, and skill-weaving behavior MUST remain unchanged.

### Key Entities

- **Quickslot observation**: The complete selected-slot result, including explicit classification, potion availability, cooldown, and optional identity.
- **Unavailable reason**: A bounded reason class explaining why no fresh valid classification can be trusted.
- **Non-potion kind**: A bounded class for item, collectible, quest item, emote, quick chat, or another wheel entry.
- **Potion availability**: Positive stack and usable, positive stack but blocked, or depleted.
- **Diagnostic snapshot**: One bounded receipt of raw non-localized facts and their final classification.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All explicit protocol discriminants decode to their documented state in automated tests, with no cooldown value capable of changing the classification.
- **SC-002**: Every negative field-matrix case maps to a non-actionable state and never shares a usable Potion state.
- **SC-003**: Twenty unchanged update cycles produce zero duplicate state events and zero normal diagnostic lines.
- **SC-004**: Corrupt, partial, absent, or legacy samples clear or retain only an Unavailable state within the existing freshness bound.
- **SC-005**: A selected potion moves Ready to Remaining to Ready in deterministic tests without a selected-slot change.
- **SC-006**: The full Rust format, lint, unit, integration, protocol-agreement, and policy suites pass.
- **SC-007**: One real-client receipt records the old pipeline's first failed predicate and all required field-matrix outcomes before issue closure.

## Clarifications

### Session 2026-09-03

- S042 contains issue #24 only. Issue #25 is blocked by this contract and is the recommended S043 consumer.
- The protocol will prefer one dedicated discriminant block over reserved cooldown values because classification and cooldown are independent facts.
- Old addons fail closed as Unavailable(legacy protocol); the reader will not preserve the unsafe cooldown-derived potion inference for compatibility.
- A numeric item ID remains optional diagnostic context, not a safety input and not a required main-view field.
- Real-client evidence cannot be fabricated by a headless development session. The implementation must make the receipt trivial to capture, and #24 remains open until that receipt is attached.

## Assumptions

- ESO API 101050 is the current baseline and keeps the documented quickslot calls unprotected.
- The existing pixel marker-plus-complement checksum remains the transport integrity mechanism.
- One additional beacon block fits the current two-row capture contract; geometry redesign remains owned by issue #26.
- No localized item name or description parsing is acceptable for classification.
- The operator performs the final real-client field matrix because the development runner must not launch or focus-steal the game client.
