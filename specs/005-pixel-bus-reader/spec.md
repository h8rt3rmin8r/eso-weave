# Feature Specification: Pixel Bus Reader

**Feature Branch**: `005-pixel-bus-reader`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "Pixel Bus Reader per master specification section 9.3 (the sampling and decoding side). Sample the three beacon points from the game window surface, decode the status heartbeat, fishing state, and latency with per-channel tolerance and a checksum, and emit typed events including SignalLost when the heartbeat is lost. Excludes the addon rendering (done), the Beacon Manager, and the fishing controller."

## Clarifications

### Session 2026-07-11

Resolved under the Build-Phase Autopilot Protocol from the master specification
and the constitution (no options were escalated).

- Q: What is the reader's testable core versus the untestable part? -> A: The
  decoders (color matching with tolerance, latency checksum) and the state
  machine that turns a timed sequence of samples into events are pure and tested
  with crafted samples and a virtual clock. The operating-system surface sampling
  is a thin backend behind a seam, validated on real hardware.
- Q: How is an absent block represented? -> A: Sampling always yields the pixel
  currently on screen (or nothing if the window cannot be sampled). A block is
  absent when its sample does not match any of that block's colors within
  tolerance; the status block absent (or no sample) is what drives the heartbeat
  timeout.
- Q: What events does the reader emit? -> A: Heartbeat, SignalLost,
  FishingStarted, BiteDetected, FishingStopped, and Latency, matching the fishing
  detector event set (section 8.1) plus latency, so the later fishing controller
  consumes them directly.
- Q: How is the latency sample validated? -> A: The green channel must equal the
  marker within tolerance and the red plus blue channels must sum to 255 within
  tolerance (the checksum); only then is latency decoded as red times four.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Heartbeat and signal loss (Priority: P1)

The reader treats a present status block as a heartbeat and, when the status
block is absent (or the window cannot be sampled) for longer than the heartbeat
timeout, raises a single signal-loss event so downstream automation stops rather
than acting on stale state.

**Why this priority**: The heartbeat is the safety anchor of the whole fishing
vertical; losing it must reliably and promptly disable fishing rather than
blind-firing.

**Independent Test**: Feed the reader a sequence of samples with a virtual clock:
a present status block yields heartbeats; removing it and advancing the clock past
the timeout yields exactly one signal-loss event; restoring it yields a heartbeat
again.

**Acceptance Scenarios**:

1. **Given** the status block matches its color, **When** the reader observes a
   sample, **Then** it emits a heartbeat.
2. **Given** a prior heartbeat, **When** the status block is absent for longer
   than the heartbeat timeout, **Then** the reader emits exactly one signal-loss
   event and reports the signal as lost.
3. **Given** the signal was lost, **When** the status block reappears, **Then**
   the reader emits a heartbeat and clears the lost state.

---

### User Story 2 - Fishing state transitions (Priority: P1)

The reader decodes the fishing block into fishing-started, bite-detected, and
fishing-stopped events as the block color changes, so the fishing controller can
drive casts and reels.

**Why this priority**: These transitions are the fishing signal the controller
acts on; wrong transitions cause missed or spurious reels.

**Independent Test**: Feed samples transitioning through the waiting color, the
bite color, and absent, and confirm the correct ordered events.

**Acceptance Scenarios**:

1. **Given** the fishing block changes to the waiting color from absent or bite,
   **When** observed, **Then** the reader emits fishing-started.
2. **Given** the fishing block changes to the bite color, **When** observed,
   **Then** the reader emits bite-detected.
3. **Given** the fishing block becomes absent from waiting or bite, **When**
   observed, **Then** the reader emits fishing-stopped.

---

### User Story 3 - Latency decoding with checksum (Priority: P2)

The reader decodes the latency block only when its marker and checksum validate,
converting it to a latency value, and ignores a block that fails validation, so a
misread never produces a bogus latency.

**Why this priority**: Latency feeds later adaptive timing; a misread that slipped
through would corrupt timing, so validation is essential.

**Independent Test**: Decode representative latency encodings (including the
clamped maximum) and confirm the value; corrupt the marker or checksum and confirm
no latency is produced.

**Acceptance Scenarios**:

1. **Given** a latency block whose marker and checksum validate, **When**
   decoded, **Then** the reader emits the decoded latency value (red times four).
2. **Given** a latency block whose marker or checksum is wrong beyond tolerance,
   **When** decoded, **Then** no latency value is produced.

---

### Edge Cases

- What happens when compositor rounding shifts a channel by a small amount? A
  configurable per-channel tolerance (default plus or minus 2) absorbs it.
- What happens when the window cannot be sampled at all? That is treated the same
  as an absent status block and drives the heartbeat timeout.
- What happens when the status block is absent only briefly (less than the
  timeout)? No signal-loss event is emitted; a returning status block resumes
  heartbeats.
- What happens when fishing colors and latency are present but the status block is
  absent? With no heartbeat, fishing and latency are not decoded; the heartbeat
  timeout governs.

## Requirements *(mandatory)*

### Functional Requirements

Sampling:

- **FR-001**: The reader MUST sample the three beacon points (the centers of the
  status, fishing, and latency blocks) from the game window surface through a
  sampling seam, at a configurable interval (default 100 ms while fishing is
  enabled, 1000 ms otherwise).
- **FR-002**: A per-channel color match tolerance MUST be configurable (default
  plus or minus 2) and applied to all color comparisons.

Heartbeat and signal loss:

- **FR-003**: A status sample matching magenta within tolerance MUST be treated as
  a heartbeat and emit a heartbeat event.
- **FR-004**: When the status block is absent (no match, or the surface cannot be
  sampled) for longer than a configurable heartbeat timeout (default 2000 ms)
  after the last heartbeat, the reader MUST emit exactly one signal-loss event and
  mark the signal lost until the status block returns.
- **FR-005**: Fishing and latency MUST be decoded only when the status heartbeat
  is present.

Fishing state:

- **FR-006**: A fishing sample matching the waiting color (transitioning into
  waiting from absent or bite) MUST emit fishing-started.
- **FR-007**: A fishing sample matching the bite color MUST emit bite-detected.
- **FR-008**: A fishing sample that becomes absent from waiting or bite MUST emit
  fishing-stopped.

Latency:

- **FR-009**: The latency block MUST be decoded only when the green channel equals
  the marker (`0xA5`) within tolerance and the red plus blue channels sum to 255
  within tolerance; the decoded latency is red times four.
- **FR-010**: A latency block failing marker or checksum validation MUST NOT
  produce a latency value.

Events and testability:

- **FR-011**: The reader MUST emit typed events (heartbeat, signal-loss,
  fishing-started, bite-detected, fishing-stopped, latency) matching the fishing
  detector event set plus latency.
- **FR-012**: The decoding and the event state machine MUST be pure and testable
  with crafted samples and an injected clock, independent of any real surface
  sampling.

Platform sampling:

- **FR-013**: Surface sampling MUST be provided by platform backends behind the
  seam (GDI window-surface capture on Windows; X11 or XWayland surface capture on
  Linux). Pure-Wayland capture without an XWayland surface is out of scope.

### Key Entities *(include if feature involves data)*

- **Color Sample**: A red-green-blue triple sampled from one beacon point, or
  nothing when the surface cannot be sampled.
- **Fishing Signal**: Waiting, bite, or none, decoded from the fishing block.
- **Pixel Bus Event**: One of heartbeat, signal-loss, fishing-started,
  bite-detected, fishing-stopped, or latency.
- **Reader State**: The last heartbeat time, whether the signal is lost, and the
  last fishing signal, used to compute transitions.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A present status block yields a heartbeat on every observation; an
  absent status block past the timeout yields exactly one signal-loss event, in
  100 percent of cases.
- **SC-002**: Each fishing color transition yields exactly its defined event, for
  100 percent of transitions.
- **SC-003**: Latency decodes correctly for representative values including the
  clamped maximum, and a corrupted marker or checksum yields no latency, in 100
  percent of cases.
- **SC-004**: A channel shift within tolerance still matches; a shift beyond
  tolerance does not, verifiable at the tolerance boundary.
- **SC-005**: All event and decoding behavior is verifiable without real surface
  sampling, using the sampler seam and an injected clock.

## Assumptions

- Scope is the reader half of master specification section 9.3. The addon
  rendering (done in the prior slice), the Beacon Manager, and the fishing
  controller are out of scope.
- The reader consumes the beacon rendered by the PixelBeacon addon; block
  positions, colors, and the latency encoding are the fixed contract from the
  prior slice.
- The operating-system surface sampling backends are thin and validated on real
  hardware; the decoding and state machine are pure and fully unit-tested. The
  GDI single-pixel read of a hardware-accelerated game window is the mechanism the
  specification chose and is validated in-game.
- The sampling interval and enabling of fishing sampling are driven by the fishing
  controller in a later slice; this slice exposes the interval as configuration.
