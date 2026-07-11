# Feature Specification: Fishing Controller

**Feature Branch**: `007-fishing-controller`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "Fishing Controller per master specification section 8. A BiteDetector trait emits typed events (Heartbeat, FishingStarted, BiteDetected, FishingStopped, SignalLost); version 1 ships one implementation, PixelBusDetector, which adapts the existing Pixel Bus Reader events (dropping Latency). A fishing controller state machine (Disabled, Armed, Waiting, Reeling, Recast) is driven by detector events and an injected clock: a fishing toggle arms and disarms it; in Armed it sends the interact key once to cast and expects FishingStarted within arm_timeout_ms (default 5000) or disarms; on BiteDetected it sends the interact key after reel_delay_ms (default 100), waits recast_delay_ms (default 3000), then sends the interact key again to recast; FishingStopped returns to Armed; loss of the beacon heartbeat (SignalLost) at any point disables fishing rather than blind-firing inputs. All fishing timing parameters are user-configurable and persisted as an additive settings section. The interact key is synthesized through a testable sink seam (the same InputOp synthesis the weave engine uses) so the state machine is pure and unit-tested with an injected clock and a mock sink and a stub detector. Depends on features 002 (input engine) and 005 (pixel bus reader). Excludes the GUI wiring and the actual real-time worker loop that pumps the detector; this slice delivers the controller, the detector abstraction, and the v1 PixelBusDetector adapter."

## Clarifications

### Session 2026-07-11

Resolved under the Build-Phase Autopilot Protocol from the master specification
(section 8) and the constitution (no options were escalated).

- Q: Is the controller event-and-tick driven or does it block on delays? -> A:
  Event-and-tick driven and non-blocking. The controller has one entry point for
  a detector event and one for a clock tick; all delays and timeouts
  (arm_timeout_ms, reel_delay_ms, recast_delay_ms) are deadlines evaluated against
  an injected millisecond clock, never blocking sleeps. This keeps the state
  machine pure and testable and keeps the future worker loop free to poll the
  detector between ticks.
- Q: How is the interact key sent, and what exactly is emitted? -> A: Through a
  sink seam that synthesizes a key press followed by a key release of the
  configured interact key via the input engine's key synthesis (the same
  `InputBackend` synthesis path the weave engine's real sink uses, but the fishing
  sink depends only on the input engine, not on the weave engine). The controller
  never talks to a real device directly; a mock sink records the operations for
  tests and a real sink drives the input backend in production.
- Q: What is the interact key and is it configurable? -> A: The interact key is
  the game's use/interact key, configurable in the fishing settings section
  (default is the interact binding used to cast and reel). It is stored with the
  other fishing parameters.
- Q: On SignalLost, what state results and what is emitted? -> A: From any active
  state (Armed, Waiting, Reeling, Recast) SignalLost transitions to Disabled and
  emits no interact input. Any pending reel or recast deadline is cancelled so no
  queued input fires after the signal is lost.
- Q: What happens to a pending recast if FishingStopped or a toggle arrives first?
  -> A: Leaving Waiting or the recast wait for any reason (FishingStopped, toggle
  off, SignalLost) cancels the pending interact deadline; the controller never
  fires a scheduled interact after it has left the state that scheduled it.
- Q: Does the controller consume the Heartbeat and Latency events? -> A: The
  BiteDetector event set is Heartbeat, FishingStarted, BiteDetected,
  FishingStopped, and SignalLost (no Latency); the PixelBusDetector drops the
  reader's Latency events. Heartbeat is observed (it confirms the signal is live)
  but drives no state change on its own; the absence of the heartbeat surfaces as
  SignalLost from the reader.
- Q: After the recast interact, if no FishingStarted arrives, what happens? -> A:
  In Recast the controller awaits FishingStarted; if it does not arrive within
  arm_timeout_ms of the recast interact, the controller returns to Armed (matching
  the master diagram's `Recast --> Armed: recast timeout`), which re-casts. If that
  fresh cast also receives no FishingStarted within arm_timeout_ms, the Armed arm
  timeout then disarms to Disabled. This makes Recast the auto-continue path and
  Armed the retry-then-give-up path.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Automated cast, reel, and recast loop (Priority: P1)

A player arms fishing and the controller runs the cast-reel-recast loop for them:
it casts, waits for a bite, reels shortly after the bite, waits for the catch to
resolve, and recasts, repeating until the player disarms.

**Why this priority**: This loop is the entire point of the feature; without it
there is no fishing automation. It is the minimum viable slice.

**Independent Test**: Drive a stub detector and a mock sink with an injected
clock: toggle on, feed FishingStarted, BiteDetected, advance the clock past the
reel and recast delays, and confirm the interact key is emitted at the cast, at
reel_delay after the bite, and again at recast, with the expected state
transitions.

**Acceptance Scenarios**:

1. **Given** the controller is Disabled, **When** the fishing toggle is turned on,
   **Then** it enters Armed and emits the interact key once to cast.
2. **Given** Armed after a cast, **When** FishingStarted is received, **Then** it
   enters Waiting and emits no further input.
3. **Given** Waiting, **When** BiteDetected is received, **Then** it enters
   Reeling and, after reel_delay_ms have elapsed on the clock, emits the interact
   key to reel.
4. **Given** the reel was sent, **When** recast_delay_ms have elapsed, **Then** it
   emits the interact key again to recast and enters Recast awaiting the next
   FishingStarted.
5. **Given** Recast, **When** FishingStarted is received, **Then** it returns to
   Waiting for the next bite.
6. **Given** Recast after a recast interact, **When** arm_timeout_ms elapse with no
   FishingStarted, **Then** it returns to Armed and re-casts.

---

### User Story 2 - Signal loss disables fishing safely (Priority: P1)

If the beacon heartbeat is lost at any point, the controller disables fishing
immediately rather than continuing to send interact inputs against an unknown game
state.

**Why this priority**: This is the safety-critical behavior of the feature.
Blind-firing the interact key with no live signal is exactly the failure mode the
design exists to prevent, so it is covered by required tests.

**Independent Test**: From each active state, feed SignalLost and confirm the
controller transitions to Disabled, emits no interact input, and that a previously
scheduled reel or recast does not fire after the clock advances past its deadline.

**Acceptance Scenarios**:

1. **Given** any active state (Armed, Waiting, Reeling, Recast), **When**
   SignalLost is received, **Then** the controller enters Disabled and emits no
   interact input.
2. **Given** Reeling with a pending reel deadline, **When** SignalLost is received
   before the deadline, **Then** advancing the clock past the deadline emits no
   interact input.
3. **Given** Disabled after SignalLost, **When** the clock advances, **Then** no
   input is ever emitted until the player toggles fishing on again.

---

### User Story 3 - Arm and disarm control (Priority: P1)

A player controls fishing with a single toggle, and the controller disarms itself
if a cast never takes (no fish nearby) so it does not sit forever in a stuck cast.

**Why this priority**: The toggle is how the player starts and stops the feature,
and the arm timeout is what keeps a failed cast from wedging the loop. Both are
needed for the loop to be usable.

**Independent Test**: Toggle on into Armed; advance the clock past arm_timeout_ms
with no FishingStarted and confirm it disarms to Armed-retry or Disabled per the
defined rule; toggle off from any state and confirm it returns to Disabled with no
further input.

**Acceptance Scenarios**:

1. **Given** Armed after a cast, **When** arm_timeout_ms elapse with no
   FishingStarted, **Then** the controller disarms (returns to Disabled) and emits
   no further input.
2. **Given** any state, **When** the fishing toggle is turned off, **Then** the
   controller returns to Disabled and cancels any pending interact deadline.
3. **Given** Waiting, **When** FishingStopped is received, **Then** the controller
   returns to Armed and recasts on the next arm cycle.

---

### User Story 4 - Configurable timing persisted in settings (Priority: P2)

A player can adjust the fishing timing (arm timeout, reel delay, recast delay) and
the interact key, and those choices persist across restarts.

**Why this priority**: The defaults work out of the box, so configurability is
valuable but not required for the core loop; it ranks below the P1 behavior.

**Independent Test**: Round-trip a fishing configuration (custom timings and
interact key) through the settings section and confirm the loaded configuration
equals the saved one, and that an absent section yields the documented defaults.

**Acceptance Scenarios**:

1. **Given** a fishing configuration with custom values, **When** it is saved and
   reloaded, **Then** the reloaded values equal the saved values.
2. **Given** no fishing settings section, **When** the configuration is loaded,
   **Then** the documented defaults are used (arm_timeout_ms 5000, reel_delay_ms
   100, recast_delay_ms 3000).
3. **Given** an invalid timing value in the section, **When** it is loaded,
   **Then** the value falls back to its default and a notice is surfaced.

---

### Edge Cases

- What happens if BiteDetected arrives while Armed (before FishingStarted)? The
  controller treats an active bite as the fishing having started and reels; a bite
  implies a live cast. (A defensive transition; the common path is
  FishingStarted then BiteDetected.)
- What happens if FishingStopped arrives during the recast wait? Leaving the wait
  cancels the pending recast interact; the controller returns to Armed without
  firing the queued input.
- What happens if the toggle is turned on when already enabled, or off when
  already Disabled? The operation is idempotent; a redundant toggle-on does not
  emit a second cast, and a redundant toggle-off emits nothing.
- What happens if two detector events arrive between ticks? Each event is handled
  in order; deadlines are evaluated on the next tick against the current clock.
- What happens if the clock does not advance between a schedule and a tick? A
  deadline fires only once the clock reaches or passes it; a tick at exactly the
  deadline fires it.
- What happens on Heartbeat events? They confirm a live signal but cause no state
  change; only SignalLost (heartbeat lost) changes state.

## Requirements *(mandatory)*

### Functional Requirements

Detector abstraction:

- **FR-001**: A BiteDetector abstraction MUST emit exactly the typed events
  Heartbeat, FishingStarted, BiteDetected, FishingStopped, and SignalLost, so the
  controller depends on the abstraction and not on any specific detector.
- **FR-002**: Version 1 MUST provide one detector implementation, PixelBusDetector,
  that adapts the Pixel Bus Reader's events into BiteDetector events and drops the
  reader's Latency events.

State machine:

- **FR-003**: The controller MUST implement the states Disabled, Armed, Waiting,
  Reeling, and Recast and MUST be driven by detector events and a clock tick, with
  no blocking sleeps.
- **FR-004**: A fishing toggle MUST arm the controller from Disabled (entering
  Armed and sending the interact key once to cast) and MUST disarm it from any
  state (returning to Disabled). Redundant toggles MUST be idempotent.
- **FR-005**: In Armed, if FishingStarted is not received within arm_timeout_ms of
  the cast, the controller MUST disarm (return to Disabled) and emit no further
  input.
- **FR-006**: On FishingStarted, the controller MUST enter Waiting; on
  FishingStopped from Waiting, it MUST return to Armed.
- **FR-007**: On BiteDetected, the controller MUST enter Reeling and MUST send the
  interact key once reel_delay_ms have elapsed on the clock.
- **FR-008**: After reeling, the controller MUST wait recast_delay_ms and then send
  the interact key again to recast, entering Recast to await the next
  FishingStarted.
- **FR-008a**: In Recast, if FishingStarted is not received within arm_timeout_ms
  of the recast interact, the controller MUST return to Armed (which re-casts),
  per the master diagram's `Recast --> Armed: recast timeout`.

Safety:

- **FR-009**: On SignalLost from any active state, the controller MUST transition
  to Disabled and MUST NOT emit any interact input.
- **FR-010**: Leaving the state that scheduled a delayed interact (via SignalLost,
  FishingStopped, toggle off, or any other transition) MUST cancel the pending
  interact deadline so no queued input fires afterward.
- **FR-011**: The controller MUST NOT emit any interact input while Disabled.

Input synthesis seam:

- **FR-012**: The interact key MUST be emitted through a sink seam as a key press
  followed by a key release via the input engine's key synthesis; the controller
  MUST NOT drive a real input device directly and MUST NOT depend on the weave
  engine.
- **FR-013**: The controller and its timing MUST be testable with an injected
  clock, a mock sink that records operations, and a stub detector, with no real
  device, game, or wall-clock dependency.

Configuration:

- **FR-014**: The fishing timing parameters (arm_timeout_ms default 5000,
  reel_delay_ms default 100, recast_delay_ms default 3000) and the interact key
  MUST be user-configurable.
- **FR-015**: The fishing configuration MUST persist as an additive settings
  section; an absent section MUST yield the documented defaults and an invalid
  value MUST fall back to its default with a surfaced notice.

### Key Entities *(include if data involved)*

- **BiteDetector Event**: One of Heartbeat, FishingStarted, BiteDetected,
  FishingStopped, or SignalLost.
- **Fishing State**: One of Disabled, Armed, Waiting, Reeling, or Recast, plus any
  pending interact deadline and the time the current cast was armed.
- **Fishing Config**: The three timing parameters and the interact key, with
  documented defaults.
- **Interact Operation**: A key-press-then-key-release of the configured interact
  key, emitted through the sink.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Over a full cast-reel-recast cycle driven by a stub detector and a
  virtual clock, the interact key is emitted exactly at the cast, at reel_delay
  after the bite, and at recast, and at no other time, in 100 percent of runs.
- **SC-002**: From every active state, SignalLost yields Disabled with zero
  interact inputs emitted, and no scheduled reel or recast fires after the signal
  is lost, across all test cases.
- **SC-003**: A cast that receives no FishingStarted within arm_timeout_ms disarms
  in 100 percent of cases, and a toggle off from any state returns to Disabled with
  no further input.
- **SC-004**: A saved fishing configuration reloads equal to what was saved, and an
  absent section yields the documented defaults, in 100 percent of cases.
- **SC-005**: All controller behavior is verifiable without a real device, game, or
  wall clock, using the stub detector, mock sink, and injected clock.

## Assumptions

- Scope is master specification section 8 (the detector abstraction, the
  controller state machine, and the v1 PixelBusDetector adapter). The GUI toggle
  button and status display, and the real-time worker loop that polls the detector
  and drives the real sink, are out of scope for this slice and wired later.
- Feature 002 (input engine) provides the input-operation representation and the
  backend the real sink drives; feature 005 (pixel bus reader) provides the events
  the PixelBusDetector adapts. Both exist.
- The interact key uses the input engine's existing key representation; binding it
  to a specific default and exposing it in the GUI is refined when the GUI slice
  wires the controller.
- The fishing settings section follows the project's additive, user-settings-only
  configuration pattern and requires no configuration schema version change.
- The bound in-game fishing hotkey (default F2) that the master specification
  mentions is delivered when the input engine's binding surface and the GUI wire
  the toggle; this slice exposes the toggle as a controller operation that both the
  hotkey and the GUI button will call.
