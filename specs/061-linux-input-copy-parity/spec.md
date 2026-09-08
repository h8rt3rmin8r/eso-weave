# Feature Specification: Linux Input and Copy Parity

**Feature Branch**: `codex/s061-linux-input-copy-parity`

**Created**: 2026-09-07

**Status**: Implemented

**Input**: Issues #93 and #96: advertise and forward supported Linux input safely, and align latency, Live Log, and menu-gate language with actual runtime behavior.

## User Scenarios & Testing

### User Story 1 - Linux Input Is Never Silently Lost (Priority: P1)

As a Linux operator, I need the virtual input device to advertise every application key and every key supported by the grabbed keyboard, so ordinary physical input and shipped E/F3 bindings continue to work after interception begins.

**Why this priority**: Grabbing a keyboard while the replacement device lacks its capabilities can silently discard user input.

**Independent Test**: Build capability sets from the canonical application key universe and a representative physical keyboard set, then prove all application keys and mouse controls are synthesizable, unknown physical keys remain forwardable, and failed key forwarding is returned as an error.

**Acceptance Scenarios**:

1. **Given** the closed 13-key application domain, **When** Linux capabilities are built, **Then** every mapped key plus primary and secondary mouse buttons is advertised.
2. **Given** a physical keyboard with keys outside the application domain, **When** it is selected for grabbing, **Then** all of its supported key codes are also advertised by the virtual device.
3. **Given** the default Fishing E and Auto Potion F3 bindings, **When** native events arrive, **Then** both map into the application domain and round-trip through Linux mappings.
4. **Given** a forwarded physical key event, **When** virtual emission fails, **Then** the backend returns an explicit input error rather than silently discarding the key.
5. **Given** non-key metadata from evdev, **When** events are forwarded, **Then** it is intentionally excluded because the virtual device contract is key-only.

---

### User Story 2 - Missing Menu Evidence Starts Closed (Priority: P1)

As an operator, I need generated input to remain blocked until valid gameplay menu evidence arrives, so startup, corrupt, and lost PixelBeacon evidence cannot authorize input or Fishing work.

**Why this priority**: Unknown safety evidence cannot safely authorize synthesized input.

**Independent Test**: Construct input, Fishing, and Auto Potion controllers with no menu observation, attempt generated actions, and verify zero synthesis until `MenuSurface::None` arrives; then verify missing or corrupt evidence closes them again.

**Acceptance Scenarios**:

1. **Given** no menu sample has been decoded, **When** controllers initialize, **Then** input, Fishing, and Auto Potion are gated.
2. **Given** a valid gameplay sample, **When** `MenuGate(Some(MenuSurface::None))` is routed, **Then** the menu gate opens and a pending initial Fishing request begins its cast.
3. **Given** unavailable, corrupt, or lost menu evidence, **When** `MenuGate(None)` is routed, **Then** every generated-input controller closes while physical pass-through remains available.

---

### User Story 3 - Interface Copy Describes Runtime Behavior (Priority: P2)

As an operator, I need latency and logging help text to describe what the controls actually do, so configuration choices are predictable and diagnostics are trustworthy.

**Why this priority**: Incorrect help text can cause operators to tune timing and diagnostics in the wrong direction.

**Independent Test**: Assert semantic anchors in shipped strings and canonical documentation: latency adds a bounded allowance to Light Attack and Bash, the Live Log selector controls the saved global capture level including file logging, and unavailable menu evidence is described and reported as gated.

**Acceptance Scenarios**:

1. **Given** Adapt to Latency help, **When** it is rendered, **Then** it says higher scaling adds bounded delay to Light Attack and Bash rather than shortening delays.
2. **Given** the Live Log level help, **When** it is rendered, **Then** it says the saved choice controls captured events, the Live Log, and optional file logging.
3. **Given** menu evidence is unavailable, **When** diagnostics and source guidance describe the state, **Then** they consistently identify it as gated.

### Edge Cases

- The selected keyboard exposes key codes unknown to the application `Key` enum.
- Synthesis occurs before the interception loop has discovered a physical keyboard.
- A device emits repeat values or non-key metadata alongside key transitions.
- The first PixelBeacon sample is unavailable, then later becomes valid gameplay.
- Valid menu evidence is followed by corrupt or lost evidence.
- File logging is disabled while the global captured level still changes and persists.

## Requirements

### Functional Requirements

- **FR-001**: `Key` MUST expose one canonical iterable universe containing all 13 supported application keys.
- **FR-002**: Linux application capabilities MUST derive from that canonical universe and include both supported mouse buttons.
- **FR-003**: Before grabbing a physical keyboard, the Linux virtual device MUST advertise the union of application capabilities and every key reported by that keyboard.
- **FR-004**: A virtual device created early for synthesis MUST be upgraded before the physical keyboard is grabbed when its capabilities do not cover the physical keyboard.
- **FR-005**: Linux native-to-domain and domain-to-native mappings MUST round-trip all 13 application keys, including E and F3.
- **FR-006**: Unknown native keys MUST remain outside application classification while remaining eligible for key-only pass-through.
- **FR-007**: Failure to emit a forwarded key MUST surface as `InputError`; the backend MUST NOT silently discard it.
- **FR-008**: The Linux forwarding contract MUST deliberately ignore non-key evdev metadata rather than attempting unsupported virtual emission.
- **FR-009**: Linux capability and mapping behavior MUST be testable without `/dev/input` or `/dev/uinput` access.
- **FR-010**: Input and Fishing menu gates MUST initialize closed; the already closed Auto Potion default MUST remain unchanged.
- **FR-011**: Only valid explicit gameplay evidence MAY open the menu gate; missing, corrupt, and lost evidence MUST close it.
- **FR-012**: The first valid gameplay observation MUST be emitted even when the prior decoded menu state was unavailable.
- **FR-013**: Initial or later unavailable menu evidence MUST produce zero generated input while preserving physical pass-through and documented toggle exemptions; a pending initial Fishing request MUST start when valid gameplay evidence opens the startup gate.
- **FR-014**: Latency help MUST state that measured latency adds a bounded allowance to Light Attack and Bash delays, capped at 300 ms after scaling.
- **FR-015**: Live Log help MUST state that the saved global level controls captured events shown in the ring and written to optional file logging.
- **FR-016**: Source comments, diagnostics, tests, and canonical documentation MUST consistently describe unavailable menu evidence as gated.
- **FR-017**: Semantic copy tests MUST reject the obsolete claims that latency shortens delays and that the Live Log choice does not change captured events.
- **FR-018**: S061 MUST close issues #93 and #96, archive completed plan 030 with PR #98 evidence, and establish plan 031 as the sole active build plan.

### Key Entities

- **Canonical key universe**: The complete ordered set of application-recognized keys used by mappings, UI choices, and tests.
- **Linux capability set**: The union of application synthesis controls and the selected physical keyboard's supported key codes.
- **Menu evidence state**: Unavailable, gameplay, or a gating menu surface, projected fail-closed into generated-input controllers.
- **Behavior copy contract**: Stable semantic claims for latency adaptation, logging capture, persistence, and menu evidence.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Automated Linux-only tests prove 13 of 13 application keys round-trip and all 13 plus two mouse controls are advertised.
- **SC-002**: A representative physical-only key remains in the virtual capability union and its emission failure is observable.
- **SC-003**: Input, Fishing, and Auto Potion emit zero generated actions before valid gameplay menu evidence and after evidence loss.
- **SC-004**: Zero shipped strings or canonical pages retain either obsolete latency-shortening or panel-local/non-capturing logging claims.
- **SC-005**: Full format, strict Clippy, locked tests, documentation policy, spelling, text hygiene, and mdBook validation pass.

## Assumptions

- The application key domain remains intentionally closed; arbitrary physical keys pass through without becoming configurable application bindings.
- The virtual device remains key-only. Scan metadata is unnecessary for correct key transitions and is not part of the advertised capability contract.
- Rebuilding an early app-only virtual device before grabbing is acceptable because no physical event interception has begun.
- The existing 300 ms latency cap and global logging-level behavior are authoritative.

## Explicit Deviation

Issue #96 assumes existing menu-evidence behavior is already fail-closed. Discovery showed that input and Fishing initialize open and the first unavailable sample emits no transition. S061 therefore includes a proportional runtime correction before aligning copy, because preserving the existing startup hole would make the requested documentation false and violate the safety constitution.

## Out of Scope

- Settings application timing and Fishing binding issue #95.
- Expanding the application key domain or settings schema.
- Windows key mapping changes.
- Forwarding non-key Linux event classes.
- Embedded documentation site expansion, release work, and live-game verification.
