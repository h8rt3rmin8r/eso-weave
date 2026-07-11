# Feature Specification: PixelBeacon Companion Addon

**Feature Branch**: `004-pixelbeacon-addon`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "PixelBeacon companion addon per master specification sections 9.1 to 9.3 (the addon rendering and bite-detection side). A minimal in-game Lua shim that renders three color-coded pixel-bus blocks and detects a fishing bite via bait consumption. Deliverable is the embedded addon files: manifest PixelBeacon.txt with the managed marker line and a version field, plus PixelBeacon.lua. Excludes the Beacon Manager (install, verify, uninstall, AddOns discovery), the Pixel Bus Reader sampling side, and the fishing controller."

## Clarifications

### Session 2026-07-11

Resolved under the Build-Phase Autopilot Protocol from the master specification
and the constitution (no options were escalated).

- Q: What is the deliverable and where does it live? -> A: Two files under
  `addon/PixelBeacon/`: the manifest `PixelBeacon.txt` and `PixelBeacon.lua`.
  They are the source of the embedded addon; installing them into the game is the
  later Beacon Manager slice.
- Q: How is the addon version expressed and kept in sync? -> A: The manifest
  carries an addon version and the game API version; the same addon version is
  referenced by the Beacon Manager for verification. This slice sets the initial
  addon version.
- Q: How is the pixel-bus block geometry kept constant in physical pixels given
  the user's UI scale? -> A: The addon divides the fixed 16 physical-pixel block
  size by the current UI global scale to compute UI-space dimensions, and
  re-derives them when the scale changes, so blocks stay 16 by 16 physical
  pixels.
- Q: How are false positives from non-fishing consumable use avoided? -> A: Bite
  detection is gated on an active fishing interaction and is suppressed while any
  menu is open, per the bait-consumption contract.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Heartbeat and status block (Priority: P1)

While the addon is loaded and rendering in the game, a solid status block is
shown at the top-left of the client area so an external reader can confirm the
addon is alive. The block disappears during loading screens and reappears
afterward.

**Why this priority**: The status block is the heartbeat the whole pixel bus
depends on; without it the reader cannot tell the addon is present, and loss of
it is what later triggers a safe shutdown of fishing.

**Independent Test**: Load the addon in-game (or inspect the rendering logic) and
confirm the status block is drawn at the defined position with the defined color
whenever the addon is active, and is hidden during loading screens.

**Acceptance Scenarios**:

1. **Given** the addon is loaded and the world is shown, **When** rendering
   occurs, **Then** the status block is drawn at the top-left at the defined
   position and color.
2. **Given** a loading screen is active, **When** the UI lifecycle hides addon
   UI, **Then** the status block is not shown.

---

### User Story 2 - Fishing state block (Priority: P1)

The fishing block reflects the fishing state: one color while a cast is active
and waiting, a different color when a bite is detected, and absent otherwise, so
an external reader can drive fishing automation from the screen.

**Why this priority**: The fishing block is the signal the fishing feature
consumes; its correctness is the reason the addon exists.

**Independent Test**: Drive the addon through cast, bite, and idle transitions
(in-game or via the state logic) and confirm the fishing block shows the correct
color for each state and is absent when idle.

**Acceptance Scenarios**:

1. **Given** a fishing cast is active and waiting, **When** the fishing block is
   rendered, **Then** it shows the waiting color.
2. **Given** a bite is detected, **When** the fishing block is rendered, **Then**
   it shows the bite color.
3. **Given** no fishing is in progress, **When** the fishing block is rendered,
   **Then** it is absent.

---

### User Story 3 - Bite detection via bait consumption (Priority: P1)

The addon detects a bite by observing the equipped bait stack decrease by one
while a fishing interaction is active, and clears the bite state when the catch
resolves, the interaction ends, or a safety timeout elapses. Detection is
suppressed while menus are open.

**Why this priority**: Bite detection is the core in-game logic that sets the
fishing block; a false or missed bite makes the whole fishing feature unreliable.

**Independent Test**: Simulate the bait-slot decrement while a fishing
interaction is active (and while a menu is open) and confirm a bite is detected
only in the valid case and cleared on the defined conditions.

**Acceptance Scenarios**:

1. **Given** a fishing interaction is active and no menu is open, **When** the
   equipped bait stack decreases by one, **Then** a bite is detected.
2. **Given** a menu is open, **When** a stack decreases by one, **Then** no bite
   is detected.
3. **Given** a bite was detected, **When** a new item is gained, the interaction
   ends, or the safety timeout elapses, **Then** the bite state is cleared.

---

### User Story 4 - Latency block and managed manifest (Priority: P2)

The latency block encodes the server latency with a marker and a checksum so a
reader can validate it, updated about once per second and only while the status
block is shown. The manifest identifies the addon as managed by this application
and carries a version.

**Why this priority**: The latency block feeds later latency-adaptive timing, and
the managed marker is what makes safe uninstall possible in a later slice.

**Independent Test**: Inspect the latency encoding for representative latency
values (marker and checksum correct), confirm it is only rendered with the status
block, and confirm the manifest contains the managed marker line and a version.

**Acceptance Scenarios**:

1. **Given** a latency value, **When** the latency block is encoded, **Then** the
   red channel is the clamped scaled latency, the green channel is the fixed
   marker, and the blue channel is the checksum complement.
2. **Given** the status block is not rendered, **When** the latency block would
   update, **Then** it is not rendered.
3. **Given** the manifest, **When** it is inspected, **Then** it contains the
   managed marker line and an addon version.

---

### Edge Cases

- What happens when the UI scale changes at runtime? Block dimensions are
  re-derived so blocks remain 16 by 16 physical pixels.
- What happens when latency is above the encodable range? It is clamped before
  encoding.
- What happens when the player never reels in after a bite? The safety timeout
  clears the bite state.
- What happens when a consumable is used outside fishing? Detection is gated on an
  active fishing interaction and suppressed while menus are open, so it is not a
  bite.
- What happens during a loading screen? The blocks are hidden by the game UI
  lifecycle and reappear afterward.

## Requirements *(mandatory)*

### Functional Requirements

Pixel-bus rendering:

- **FR-001**: The addon MUST render blocks anchored to the top-left of the game
  window client area, each 16 by 16 physical pixels, compensating for the user's
  UI scale so geometry is constant in physical pixels.
- **FR-002**: The status block MUST be solid magenta (`#FF00FF`) at position (0,
  0) whenever the addon is loaded and rendering (the heartbeat).
- **FR-003**: The fishing block MUST be at position (16, 0) and show one color
  while a cast is active and waiting (`#0080FF`), another when a bite is detected
  (`#00FF00`), and be absent otherwise.
- **FR-004**: The latency block MUST be at position (32, 0) and encode
  `GetLatency()` as red = clamp(latency, 0, 1020) / 4, green = `0xA5` (marker),
  blue = 255 minus red (checksum), updated about once per second, and rendered
  only while the status block is rendered.
- **FR-005**: All blocks MUST be hidden during loading screens by the game UI
  lifecycle and reappear afterward.

Bite detection:

- **FR-006**: The addon MUST treat a stack-count change of minus one on the
  equipped bait item, while a fishing interaction is active, as a bite.
- **FR-007**: The "fishing interaction active" condition MUST be gated via the
  client interaction events and the camera interaction state.
- **FR-008**: The bite state MUST be cleared when a new item is gained (catch
  resolved), when the interaction ends, or after a safety timeout.
- **FR-009**: Bite detection MUST be suppressed while any menu is open.

Addon nature and manifest:

- **FR-010**: The addon MUST be a minimal shim with no settings, no user
  interface beyond the blocks, no external libraries, and no saved variables.
- **FR-011**: The manifest MUST contain the managed marker line
  `## X-ESO-Weave-Managed: true` and an addon version field, and MUST declare the
  game API version.
- **FR-012**: The deliverable MUST be the two embedded files `PixelBeacon.txt`
  and `PixelBeacon.lua` under `addon/PixelBeacon/`; the addon MUST NOT be
  published to any addon index.

### Key Entities *(include if feature involves data)*

- **Beacon Block**: A solid-color rectangle at a fixed physical-pixel position
  and size, whose color encodes a piece of state (status, fishing, latency).
- **Fishing State**: Whether a cast is active and waiting, a bite is detected, or
  neither, which selects the fishing block color.
- **Bite Signal**: The detected bite condition derived from bait consumption
  during an active fishing interaction.
- **Manifest**: The addon descriptor carrying the managed marker, the addon
  version, and the declared game API version.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The status block is rendered at the defined position and color
  whenever the addon is active, in 100 percent of active frames outside loading
  screens.
- **SC-002**: The fishing block shows the correct color for each of the three
  states and is absent when idle, for 100 percent of states.
- **SC-003**: The latency encoding is correct (marker and checksum) for
  representative latency values across the range, including clamping above the
  range.
- **SC-004**: A bite is detected only when the bait stack decreases by one during
  an active fishing interaction with no menu open, and never from an out-of-
  fishing consumable use.
- **SC-005**: The manifest contains the managed marker line verbatim and an addon
  version, verifiable by inspection.

## Assumptions

- Scope is master specification sections 9.1 to 9.3 (the addon side). The Beacon
  Manager install, verify, uninstall, and AddOns discovery (9.4, 9.5), the Pixel
  Bus Reader sampling (the reader half of 9.3), and the fishing controller
  (section 8) are out of scope and handled in later slices.
- The addon uses only the game's Lua API available to addons loaded from the
  AddOns directory; it has no IPC, no network, and no saved variables, consistent
  with the bounded-scope constitution principle.
- Bite detection follows the bait-consumption mechanism established by prior ESO
  addon art, with the false-positive suppressions the specification lists.
- The addon version set here is the initial version; the game API version
  declared in the manifest is subject to the later APIVersion upkeep item (R4).
- Verification of this slice is structural (manifest and Lua content) and manual
  in-game; there is no automated game harness, so the Lua logic is written to be
  inspectable and is exercised in the running client.
