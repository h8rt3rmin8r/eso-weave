# Feature Specification: Hotkey and Weapon-Bar Detection Fixes

**Feature Branch**: `015-hotkey-detection-fixes`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "Fix runtime input/detection wiring so the default
hotkeys and the weapon-bar readout actually work in-game. Scope is bug-fix and
wiring only; no new weaving behavior."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - F1 suspends and resumes the engine (Priority: P1)

While the game window is focused, the operator presses the default suspend
hotkey (F1) and the application toggles its suspend state, exactly as the GUI
Status button already does. Pressing F1 again resumes it. The GUI Status
indicator reflects the change without the operator touching the mouse.

**Why this priority**: A hotkey the operator cannot use from inside the game is
the single most visible failure; suspend is the primary safety control and must
be reachable without alt-tabbing away from combat.

**Independent Test**: Feed a focused F1 key-down through the input engine and
assert the suspend state flips and the derived Status view label changes; feed a
second F1 and assert it flips back.

**Acceptance Scenarios**:

1. **Given** the engine is running (not suspended) and the game window is
   focused, **When** the operator presses F1, **Then** the engine becomes
   suspended and the GUI Status line reflects the suspended state.
2. **Given** the engine is suspended and the game window is focused, **When** the
   operator presses F1, **Then** the engine resumes and the GUI Status line
   reflects the running state.
3. **Given** the game window is NOT focused, **When** F1 is pressed, **Then** the
   suspend state does not change (input is scoped to the focused game window).

---

### User Story 2 - F2 enables and disables fishing (Priority: P1)

While the game window is focused, the operator presses the default fishing
hotkey (F2) and fishing toggles between enabled and disabled, exactly as the GUI
Fishing button already does. The GUI Fishing line reflects the change.

**Why this priority**: Fishing is a headline feature; toggling it from the game
is the expected interaction and today it silently does nothing.

**Independent Test**: Feed a focused F2 key-down through the input engine and
assert the fishing controller's enabled state flips and the derived Fishing view
label changes; feed a second F2 and assert it flips back.

**Acceptance Scenarios**:

1. **Given** fishing is disabled and the game window is focused, **When** the
   operator presses F2, **Then** fishing becomes enabled and the GUI Fishing line
   reflects it.
2. **Given** fishing is enabled and the game window is focused, **When** the
   operator presses F2, **Then** fishing becomes disabled and the GUI Fishing
   line reflects it.
3. **Given** the game window is NOT focused, **When** F2 is pressed, **Then**
   fishing state does not change.

---

### User Story 3 - Detected weapon bar is visible and diagnosable (Priority: P2)

When the PixelBeacon addon renders a valid weapon-bar (B3) block, the GUI shows
the detected active bar and the front/back weapon classes instead of the
permanent "Not detected". The operator can confirm from the log whether the
signal is being decoded, so an absent readout can be told apart from a decode
mismatch without guessing.

**Why this priority**: Detection has never surfaced in-game. Making the readout
correct and the signal diagnosable unblocks the operator's own in-game
validation, which is the only place the live pixel signal can be confirmed.

**Independent Test**: Feed a synthetic surface carrying a valid B3 block through
the decode-to-view path and assert the derived weapon-bar view is `detected`
with the expected active bar and classes; feed a surface with no B3 block and
assert the view stays `Not detected`.

**Acceptance Scenarios**:

1. **Given** the addon renders a valid B3 block for a known bar and classes,
   **When** the reader samples it, **Then** the GUI weapon-bar line shows the
   active bar and front/back classes.
2. **Given** no valid B3 block is present, **When** the reader samples,
   **Then** the GUI weapon-bar line reads "Not detected".
3. **Given** detection diagnostics are available, **When** the operator inspects
   the log at the diagnostic level, **Then** decoded weapon-bar samples (and the
   transition into or out of a detected state) are visible, without logging on
   every idle sample at the default level.

---

### Edge Cases

- What happens when F1 or F2 is held down (auto-repeat)? A held key must toggle
  once per physical press, not once per repeat, matching the existing
  newly-pressed guard for skill actions.
- What happens when the operator has rebound suspend or fishing to a different
  key? The toggle must follow the action, not the literal F1/F2 key, so a
  rebound key toggles and F1/F2 no longer does.
- What happens when the pixel signal is lost while a bar was detected? The
  readout returns to "Not detected"; no weave timing effect is introduced either
  way in this slice.
- What happens when a B3 block is present but its marker byte is outside
  tolerance? It is treated as no signal (not a misdecode), and the readout stays
  "Not detected".

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The suspend action delivered by a hotkey MUST toggle the same
  input-engine suspend state that the GUI Status control toggles, so hotkey and
  button reach one shared state.
- **FR-002**: The fishing action delivered by a hotkey MUST toggle the same
  fishing enabled/disabled state that the GUI Fishing control toggles.
- **FR-003**: A hotkey toggle MUST fire once per physical key press and MUST NOT
  repeat while the key is held.
- **FR-004**: Hotkey toggles MUST only take effect while the game window holds
  focus, preserving the existing focus-scoped suppression guarantee.
- **FR-005**: Toggling suspend or fishing via a hotkey MUST be reflected in the
  GUI derived view on the next frame, identically to toggling via the button.
- **FR-006**: The suspend and fishing hotkey toggles MUST follow their bound
  action, so a rebound key works and the previously bound key no longer toggles.
- **FR-007**: When a valid weapon-bar (B3) block is decoded, the GUI weapon-bar
  line MUST display the detected active bar and the front and back weapon
  classes.
- **FR-008**: When no valid weapon-bar block is decoded, the GUI weapon-bar line
  MUST display "Not detected".
- **FR-009**: The system MUST provide a log-based diagnostic that lets the
  operator confirm in-game whether weapon-bar samples are being decoded,
  including the transition into and out of a detected state, without emitting a
  log line on every idle sample at the default log level.
- **FR-010**: This slice MUST NOT change weave timing, skill weaving, or any
  input synthesis as a result of weapon-bar detection; detection is display and
  diagnostics only.
- **FR-011**: Existing safety-critical behavior MUST remain unchanged and covered
  by tests: injected-input recursion breaking, suppression scoped to the focused
  game window, no blocking work on the input hook thread, and fishing degrading
  to disabled on signal loss.
- **FR-012**: The suspend and fishing toggle behavior delivered by a hotkey MUST
  be persisted the same way the GUI toggles are (live session state), so the
  restored-on-launch behavior is unchanged.

### Key Entities *(include if feature involves data)*

- **Suspend state**: The single boolean the input engine reads to decide whether
  non-exempt bound keys pass through; owned outside the weave worker and shared
  with the GUI.
- **Fishing enabled state**: The fishing controller's enabled/disabled state,
  shared with the GUI and the pixel-bus worker.
- **Weapon-bar signal**: The decoded active bar plus front and back weapon
  classes produced by the pixel-bus reader; consumed by the GUI for display only
  in this slice.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With the game focused, a single F1 press changes the visible Status
  between running and suspended 100% of the time, matching the button.
- **SC-002**: With the game focused, a single F2 press changes the visible
  Fishing state between enabled and disabled 100% of the time, matching the
  button.
- **SC-003**: A held F1 or F2 produces exactly one toggle per physical press,
  never a rapid oscillation from auto-repeat.
- **SC-004**: When a valid weapon-bar block is present, the readout shows the
  correct active bar and classes; when absent, it reads "Not detected"; there are
  no other states.
- **SC-005**: The operator can determine from the log alone whether the addon's
  weapon-bar signal is reaching the reader, without adding print statements or a
  debugger.
- **SC-006**: No change to weave timing or synthesized input occurs as a result
  of this slice (verified by the unchanged weave sequence tests).

## Assumptions

- The existing default bindings (F1 = suspend, F2 = fishing) are retained; this
  slice fixes their effect, not their defaults.
- The GUI already renders the Status, Fishing, and weapon-bar lines from a
  derived view each frame; surfacing hotkey-driven changes requires no new GUI
  layout, only that the underlying shared state changes.
- The slice 014 decode path (addon B3 block, `decode_weapon_bar`,
  `set_weapon_bar`, `weapon_bar_view`) is structurally correct; the live pixel
  signal itself is validated in-game by the operator and remains an explicit
  follow-up, not a blocker for this slice.
- Diagnostics use the existing structured logging with a runtime-selectable
  level; input contents are still never logged above DEBUG and never while
  suspended.
- The slice culminates in a v0.4.2 patch release; cutting the tag and running the
  release require separate explicit authorization and are out of this slice's
  automated scope.
