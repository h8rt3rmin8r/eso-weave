# Feature Specification: Fishing Signal Diagnosis and Capture Hardening

**Feature Branch**: `024-fishing-capture-hardening`

**Created**: 2026-07-13

**Status**: Draft

**Input**: User description: "Fishing signal diagnosis and capture hardening (build plan 006, slice 024): the fourth attempt at a fishing failure where the status shows Casting and reverts to Idle (no cast detected). Harden the Windows screen capture so DirectX-rendered beacon pixels are actually read, add log-only diagnostics that pinpoint the culprit, reconcile the spec, and provide an in-game validation protocol."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Fishing actually starts (Priority: P1)

A player selects bait, casts at a fishing hole (or presses the fishing hotkey),
and the app detects the cast: the status advances from Casting to Fishing and,
on a bite, reels in, instead of reverting to Idle (no cast detected) after the arm
timeout.

**Why this priority**: This is the whole point. Fishing has been unusable across
three prior fixes because the app never sees the beacon signal, so it never leaves
Casting. Reading the on-screen signal correctly is what makes fishing work.

**Independent Test**: With bait selected, the current addon loaded, the beacon
strip visible, and the window focused, cast at a fishing hole and confirm the
status advances Casting to Fishing to Reeling rather than reverting to Idle (no
cast detected). (In-game validation, per the quickstart.)

**Acceptance Scenarios**:

1. **Given** the addon is loaded and rendering its signal on screen, **When** the
   app samples the screen, **Then** the heartbeat reads present (rather than the
   app seeing black or stale pixels).
2. **Given** the heartbeat is present and the player casts, **When** the addon
   shows the waiting signal, **Then** the status advances from Casting to Fishing.
3. **Given** a bite is shown, **When** the app samples it, **Then** the status
   advances to Reeling and the line is reeled in.

---

### User Story 2 - Diagnose the culprit from the log (Priority: P1)

If fishing still does not work, the operator can read the live log and tell, in a
single session, whether the problem is that the app is not reading the beacon at
all (no heartbeat) or that the app reads the beacon but the addon never shows the
waiting signal (heartbeat present, fishing block never blue).

**Why this priority**: Three prior fixes failed by guessing. The true root cause
can only be confirmed in-game, so the app must make the evidence unambiguous so
the next step is chosen from data, not another guess.

**Independent Test**: Enable the most verbose logging, fish, and confirm the log
shows each block's raw color bytes, the decoded fishing signal, and whether the
heartbeat is present, plus a clear message when the heartbeat is first read and
when it is lost.

**Acceptance Scenarios**:

1. **Given** verbose logging, **When** the app samples the screen, **Then** the log
   shows the raw color bytes for each beacon block, the decoded fishing signal, the
   heartbeat state, and the heartbeat age.
2. **Given** the heartbeat becomes readable, **When** it is first seen, **Then** a
   clear log message records that the beacon signal was acquired; when it is later
   lost, a clear message records that.
3. **Given** the heartbeat is present but the fishing block never turns blue with
   all preconditions met, **Then** the log evidence distinguishes this case (an
   addon interaction-detection problem) from a never-present heartbeat (a capture
   problem).

---

### Edge Cases

- The beacon strip is covered by another window, or the game window is minimized or
  unfocused: the capture cannot read the strip, no heartbeat is seen, and after the
  timeout fishing degrades to disabled (signal lost) rather than firing input
  blindly. The log makes the missing heartbeat visible.
- No bait selected: ESO does not start a fishing interaction, so the addon never
  shows the waiting signal and the app times out as no cast detected. This is a
  documented precondition, surfaced in the validation protocol.
- The addon is out of date or not loaded: no heartbeat is rendered, so no signal is
  read; the validation protocol checks this first.
- The capture momentarily fails (window moving, transient): a sample returns no
  pixels, which is treated the same as an absent block, and recovers on the next
  sample.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The Windows screen capture MUST read the beacon pixels as they are
  actually displayed on screen, including when the game renders through a
  hardware-accelerated (DirectX) surface, rather than reading a stale or blank
  window buffer.
- **FR-002**: When the addon is loaded and rendering, the app MUST read the
  heartbeat as present and MUST advance fishing from Casting to Fishing when the
  waiting signal appears and to Reeling on a bite.
- **FR-003**: The capture change MUST preserve the existing decoding behavior: the
  block positions, colors, tolerance, and event meanings are unchanged, and the
  existing decoder and reader tests continue to pass.
- **FR-004**: The app MUST log, at the most verbose level, each sample's raw block
  colors, the decoded fishing signal, the heartbeat state, and the heartbeat age.
- **FR-005**: The app MUST log a clear message when the heartbeat is first acquired
  and when it is lost, so the operator can tell at a glance whether the beacon is
  being read.
- **FR-006**: The diagnostics MUST let the operator distinguish a never-present
  heartbeat (a capture problem) from a present heartbeat with a never-appearing
  waiting signal (an addon interaction problem) in one session, with no new
  interface surface (log-only).
- **FR-007**: Fishing MUST still degrade to disabled on signal loss rather than
  firing input blind; this behavior is unchanged.
- **FR-008**: The capture MUST remain outside the game: it only reads on-screen
  pixels (the existing screen-signal contract) and MUST NOT read game memory or
  intercept network traffic.
- **FR-009**: The master specification's capture-mechanism description MUST be
  updated to match the screen-composited capture, recorded as a dated decision.
- **FR-010**: The safety-critical input-engine surfaces and the beacon
  managed-marker uninstall guarantee MUST NOT be changed by this slice; the addon
  interaction-detection contract and the keypress synthesis are also left unchanged
  so the capture fix stays isolable.
- **FR-011**: The pure pixel-extraction logic added for the capture (bounds
  checking and color-channel decoding) MUST be covered by unit tests.

### Key Entities *(include if data involved)*

- **Beacon strip capture**: a small region of the screen, at the game window's
  top-left client area, captured as displayed (composited) so the beacon blocks are
  read correctly; the four block points are read from it.
- **Heartbeat state**: whether the status block is currently read as present; its
  acquire and lose transitions are logged.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With bait selected, the current addon loaded and rendering, the strip
  visible, and the window focused, fishing advances past Casting to Fishing and
  reels in catches across a run of casts, instead of reverting to Idle (no cast
  detected).
- **SC-002**: When the addon is rendering, the app reads the heartbeat as present
  (the log shows the status block as magenta, not black), whereas before the fix it
  read black or stale pixels.
- **SC-003**: From one logged fishing session, the operator can state whether the
  heartbeat was ever present, and therefore whether any remaining failure is a
  capture problem or an addon problem.
- **SC-004**: All existing decoder and reader unit tests pass unchanged, and the
  new pixel-extraction helper is unit-tested.
- **SC-005**: Fishing still degrades to disabled on signal loss (no input is fired
  when the signal is absent).

## Assumptions

- The proven-working capture technique on this project reads the composited desktop
  framebuffer (the same mechanism as the PowerShell CopyFromScreen workaround that
  captures accelerated content where a window-buffer read or a print-window call
  returns black); this is what the Windows backend adopts.
- The beacon strip is small and at the game window's top-left, so only a tiny
  region is captured per sample; the capture runs at the existing sampling cadence.
- The common case is the game on a single monitor with the beacon strip visible and
  the window focused; occluded, minimized, or multi-monitor-edge cases degrade to no
  heartbeat (handled by the existing signal-loss path) rather than crashing.
- The true root cause can only be confirmed in-game; if the capture fix alone does
  not resolve it, the enhanced logs identify whether the remaining problem is the
  addon interaction detection, which a follow-up slice would address from evidence.
- The Linux sampling path is unchanged in behavior by this slice.
