# Feature Specification: Native ESO Binding Evidence

**Feature Branch**: `codex/s100-native-binding-evidence`

**Created**: 2026-09-15

**Status**: Ready for Planning

**Input**: Work slice S100 implements GitHub issue #206, the first child of parent issue #188.

## User Scenarios & Testing

### User Story 1 - Observe authoritative native bindings (Priority: P1)

As an ESO Weave operator, I can rely on PixelBeacon to observe the current native ESO keyboard and mouse assignments for every gameplay action that later automation slices will consume.

**Why this priority**: Native evidence must exist before duplicate desktop configuration or hardcoded controls can be removed safely.

**Independent Test**: Feed the addon fixtures for default, rebound, chorded, mouse, unbound, conflicting, unsupported, and unavailable actions, then verify the fixed-size published facts for all eleven required actions.

**Acceptance Scenarios**:

1. **Given** exactly one supported native binding, **when** PixelBeacon reads the action, **then** it publishes the primary control and the normalized modifier set as valid evidence.
2. **Given** no assigned binding, **when** PixelBeacon reads the action, **then** it publishes unbound rather than inventing a default.
3. **Given** more than one distinct assigned binding, **when** PixelBeacon reads the action, **then** it publishes conflicting rather than choosing one.
4. **Given** a gamepad, chord-key, hold-key, unknown primary, or unknown modifier, **when** PixelBeacon reads the action, **then** it publishes unsupported.
5. **Given** the discovery API or action indices are unavailable, **when** PixelBeacon refreshes, **then** it publishes unavailable.

---

### User Story 2 - Decode a safe desktop snapshot (Priority: P1)

As a later automation controller, I can consume one typed snapshot containing a state for every required ESO action without parsing strings or guessing at malformed evidence.

**Why this priority**: Publication alone is not useful until the desktop can validate and represent it at the existing pixel-bus trust boundary.

**Independent Test**: Decode exhaustive synthetic frames and verify every control, modifier combination, state, action position, protocol version, checksum failure, and stale-frame condition.

**Acceptance Scenarios**:

1. **Given** a valid current-protocol frame, **when** the desktop decodes it, **then** all eleven action facts appear in stable action order.
2. **Given** a corrupt or transposed binding cell, **when** its action-specific checksum is evaluated, **then** only that fact is unavailable and no chord is guessed.
3. **Given** a legacy layout without binding cells, **when** the desktop decodes it, **then** every binding fact is unavailable while existing telemetry remains compatible.
4. **Given** a stale or missing beacon, **when** a consumer asks for the snapshot, **then** no binding fact is presented as current.

---

### User Story 3 - Preserve read-only authority (Priority: P1)

As an operator, I can install this update knowing that PixelBeacon observes native bindings but cannot create, replace, clear, reset, or persist ESO bindings.

**Why this priority**: ESO bindings are server-persisted player state, so accidental mutation would exceed the feature's authority and could disrupt the user's controls.

**Independent Test**: Scan addon source and manifests with prohibited API and file fixtures, and verify that each mutation surface fails the policy test.

**Acceptance Scenarios**:

1. **Given** the shipped addon, **when** its source and manifest are audited, **then** no binding mutation API, custom action declaration, binding manifest, or SavedVariables entry exists.
2. **Given** a prohibited binding API or `Bindings.xml` fixture, **when** the static guard runs, **then** it fails with the offending surface identified.
3. **Given** the S100 desktop release, **when** operators use existing automation, **then** behavior and binding settings remain unchanged because migration belongs to #207 and #208.

### Edge Cases

- Duplicate binding slots that contain the same primary and modifier set count as one distinct chord, not a conflict.
- Modifier order from ESO is normalized, duplicate modifiers are rejected, and a modifier used as the primary is unsupported.
- A binding that mixes a supported primary with any unsupported modifier is unsupported as a whole.
- Gamepad, synthesized chord, and hold key codes are unsupported in this keyboard-and-mouse contract.
- A binding-change event can arrive before bindings are loaded. The publisher reports unavailable until the subsequent loaded or periodic refresh succeeds.
- A cell from one action is moved to another action position. The action-specific checksum rejects it.
- One malformed binding cell does not invalidate unrelated existing telemetry or other independently valid binding facts.
- Older negotiated layout versions remain decodable and expose unavailable binding facts rather than failing the entire beacon.

## Requirements

### Functional Requirements

- **FR-001**: PixelBeacon MUST discover skill slots 1 through 5, Ultimate, Synergy, Attack, Block, Interact, and Quickslot in that stable order.
- **FR-002**: Discovery MUST use only read-only ESO binding APIs and MUST NOT call binding creation, bind, unbind, reset, or equivalent mutation functions.
- **FR-003**: Each action MUST resolve to exactly one of unavailable, unbound, conflicting, unsupported, or valid.
- **FR-004**: A valid binding MUST contain exactly one supported keyboard or mouse primary and a normalized set drawn from Shift, Control, Alt, and Command.
- **FR-005**: Discovery MUST inspect every binding slot reported by ESO, ignore empty slots, deduplicate exact chords, and report conflicting when more than one distinct non-empty chord remains.
- **FR-006**: PixelBeacon MUST refresh binding evidence after bindings load, after a binding is set or cleared, and through its existing periodic resynchronization path.
- **FR-007**: The wire contract MUST allocate exactly one fixed-position RGB cell per required action and MUST NOT publish strings or variable-width data.
- **FR-008**: Each binding cell MUST encode its state or portable primary, modifiers when valid, and an action-specific integrity check.
- **FR-009**: The pixel-bus layout protocol MUST advertise the new fixed payload count while retaining explicit decoding support for existing negotiated and legacy layouts.
- **FR-010**: The desktop MUST define stable typed representations for native actions, portable keyboard and mouse controls, modifier sets, chords, and binding states.
- **FR-011**: The decoder MUST validate state codes, primary codes, modifier bits, action position, checksum, protocol version, and freshness before exposing a valid chord.
- **FR-012**: Malformed evidence for one action MUST degrade that action to unavailable without manufacturing a state or invalidating independent telemetry.
- **FR-013**: Layouts predating binding evidence MUST expose all eleven actions as unavailable.
- **FR-014**: Static tests MUST reject binding mutation API names, custom binding declarations, `Bindings.xml`, and binding SavedVariables in PixelBeacon.
- **FR-015**: S100 MUST NOT change controller behavior, input synthesis, application binding settings, persisted desktop configuration, or the F1, F2, and F3 desktop controls.
- **FR-016**: Protocol, addon, troubleshooting, build-plan, and changelog documentation MUST describe the read-only evidence foundation and its deferred consumers.
- **FR-017**: The S099 trust-policy bootstrap notice MUST accurately distinguish a pull-request bootstrap from execution on protected `main`.

### Key Entities

- **Native action**: One of the eleven ESO gameplay actions in the fixed transport order, with a stable action identifier and ESO action name.
- **Portable control**: A stable transport identifier for a supported physical keyboard key or mouse control, independent of ESO's runtime numeric key code.
- **Modifier set**: A normalized combination of Shift, Control, Alt, and Command with no ordering or duplication ambiguity.
- **Native chord**: One portable primary control plus a modifier set.
- **Binding state**: Unavailable, unbound, conflicting, unsupported, or valid with a native chord.
- **Binding evidence frame**: Eleven fixed-position RGB cells whose action-specific integrity checks bind their contents to their expected action positions.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All eleven required actions produce the correct state across default, rebound, chorded, mouse, unbound, duplicate, conflicting, unsupported, and unavailable addon fixtures.
- **SC-002**: Every supported portable control and all 16 modifier combinations round-trip through the desktop encoder fixture and decoder.
- **SC-003**: Every reserved state, invalid state, invalid primary, invalid modifier, checksum corruption, and action transposition fixture fails closed for the affected action.
- **SC-004**: Existing layout-version fixtures continue to decode their prior telemetry with all eleven binding states unavailable.
- **SC-005**: Static policy tests detect every prohibited mutation API and binding artifact fixture with zero prohibited surfaces in shipped PixelBeacon files.
- **SC-006**: Formatting, strict linting, complete locked tests, release build, documentation gates, hosted CI, and both authorized automated review rounds complete without unresolved findings.

## Clarification Decisions

- S100 closes child issue #206, not parent #188. Controller consumption and duplicate-setting removal remain in #207 and #208.
- A binding is conflicting when more than one distinct non-empty native chord is assigned. Exact duplicate slots collapse to one chord.
- Unsupported takes precedence only after the distinct native assignment count is known. Two assignments are conflicting even when one or both cannot be represented.
- Portable controls cover ordinary keyboard keys, mouse buttons 1 through 5, and mouse-wheel directions. Gamepad, hold, combined, and opaque key codes are unsupported.
- Binding evidence uses one RGB cell per action. Its action-specific checksum prevents a neighboring action's otherwise valid cell from being accepted in the wrong position.
- Older addons and layouts remain readable. Their absent binding evidence is represented as unavailable, never as unbound.

## Scope Boundaries

- No automation controller consumes the new chords in S100.
- No desktop binding setting or persisted field is removed or migrated.
- No input interception or synthesis behavior changes.
- No ESO binding is created, modified, reset, cleared, or persisted by PixelBeacon.
- No custom addon action, `Bindings.xml`, SavedVariables entry, string payload, or variable-size pixel region is introduced.
- No UI or screenshot changes are required because the visible desktop interface is unchanged.

## Assumptions

- The current ESO API continues to provide action lookup, maximum binding count, binding inspection, binding-loaded, binding-set, and binding-cleared surfaces.
- Keyboard and mouse binding discovery is sufficient for the desktop automation supported by #188; gamepad automation remains outside scope.
- The existing negotiated pixel-grid geometry can carry eleven additional fixed payload cells within its supported block-size and client-width bounds.
