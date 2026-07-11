# Feature Specification: Input Engine

**Feature Branch**: `002-input-engine`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "Input Engine: platform-abstracted key interception and input synthesis per master specification section 6. One Input abstraction, two real backends (Windows, Linux), and a mock backend for tests. Intercept configured keys only while the ESO window is focused, suppress bound keys, pass others through, hand off on a non-blocking path, break self-input recursion, honor suspend with suspend-exempt toggles, and a conflict-rejecting keybinding model. Excludes weave execution, fishing, PixelBeacon, and GUI."

## Clarifications

### Session 2026-07-11

Resolved under the Build-Phase Autopilot Protocol from the master specification
and the constitution (no options were escalated).

- Q: How are key-down, key-up, and auto-repeat handled for a bound key? -> A: A
  bound key while focused suppresses both its key-down and key-up so no partial
  keystroke reaches the game, hands off exactly one action on the initial
  key-down, and ignores auto-repeat until the key is released.
- Q: How do bindings persist without breaking S001 compatibility? -> A: An
  additive `bindings` section is added to the settings; files without it load the
  defaults, so no schema version bump is required.
- Q: How are physical keys identified across platforms? -> A: The core and the
  binding table use a platform-neutral key identifier; each backend maps between
  that identifier and its native scan or key code.
- Q: What are the hand-off semantics from the interception path to the worker?
  -> A: The interception path pushes the action onto a non-blocking bounded
  channel and returns immediately; if the channel is full the action is dropped
  with a warning rather than blocking the interception path.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Focused-window interception with non-blocking hand-off (Priority: P1)

While the ESO window holds keyboard focus, pressing a bound physical key is
intercepted: the original keystroke is suppressed (it does not reach the game or
anything else) and an action event is handed to a worker for later execution.
Unbound keys pass through untouched. When the ESO window does not hold focus,
nothing is intercepted at all. The interception path itself never blocks.

**Why this priority**: This is the engine's reason to exist and the surface where
getting it wrong is most damaging (a blocking interception path is silently
disabled by the OS; global interception would capture keystrokes outside the
game). It is safety-critical.

**Independent Test**: Drive synthetic key events through the test backend with the
game modeled as focused and as unfocused; confirm bound keys are suppressed and
produce a handed-off action only while focused, unbound keys always pass through,
and the interception path performs no blocking work.

**Acceptance Scenarios**:

1. **Given** the game is focused and a key is bound, **When** that key is pressed,
   **Then** the keystroke is suppressed and exactly one action event is handed to
   the worker.
2. **Given** the game is focused and a key is not bound, **When** that key is
   pressed, **Then** it passes through untouched and no action is handed off.
3. **Given** the game is not focused, **When** any key is pressed, **Then**
   nothing is intercepted or suppressed and no action is handed off.
4. **Given** a bound key press, **When** it is classified on the interception
   path, **Then** that path only classifies, suppresses, and hands off, and never
   sleeps or performs timed or blocking work.

---

### User Story 2 - Self-input never re-intercepted (Priority: P1)

When the engine synthesizes key output (on request), that synthetic input is
distinguishable from real input and is never intercepted by the engine itself, so
synthesizing a key can never trigger a runaway loop.

**Why this priority**: Recursion breaking is safety-critical; without it, a
synthesized keypress could be re-intercepted and re-synthesized without end.

**Independent Test**: Ask the engine to synthesize a key that is also bound, with
the game focused, and confirm the synthesized event is not intercepted or handed
off as a new action.

**Acceptance Scenarios**:

1. **Given** a key is bound and the game is focused, **When** the engine
   synthesizes that same key, **Then** the synthesized event is recognized as
   self-originated and is not intercepted or handed off.
2. **Given** a real press of the same key arrives after synthesis, **When** it is
   classified, **Then** it is still intercepted normally (the distinction is per
   event, not a lasting mute).

---

### User Story 3 - Suspend with suspend-exempt toggles (Priority: P2)

When the application is suspended, all interception stops except the bindings
marked suspend-exempt (toggle suspend and toggle fishing), which continue to work
so the operator can resume. Toggling suspend off restores full interception.

**Why this priority**: Suspend is the operator's safety switch; it must reliably
stop automation while leaving a way back.

**Independent Test**: With the game focused, suspend the engine, press a normal
bound key (confirm it is not intercepted and passes through), press the
suspend-exempt toggle (confirm it is still intercepted and handled), then resume
and confirm normal interception returns.

**Acceptance Scenarios**:

1. **Given** the engine is suspended and the game is focused, **When** a
   non-exempt bound key is pressed, **Then** it is not intercepted and passes
   through.
2. **Given** the engine is suspended and the game is focused, **When** a
   suspend-exempt bound key is pressed, **Then** it is intercepted and handled.
3. **Given** the engine is suspended, **When** the suspend toggle is used to
   resume, **Then** full interception is restored.

---

### User Story 4 - Configurable, conflict-free keybindings (Priority: P3)

The binding table maps each action to a physical key and can be reconfigured. Two
actions may not map to the same key; an attempt to create such a conflict is
rejected and the prior binding is unchanged. The table has documented defaults and
persists across restarts.

**Why this priority**: Bindings are user-facing configuration; correctness and
conflict rejection prevent ambiguous interception.

**Independent Test**: Start from defaults, rebind an action to a free key
(confirm accepted and persisted), attempt to rebind another action to an
already-used key (confirm rejected and unchanged), and reload to confirm
persistence.

**Acceptance Scenarios**:

1. **Given** the default bindings, **When** they are inspected, **Then** they
   match the documented defaults (skills 1 to 5 on keys 1 to 5, ultimate on R,
   synergy on X, toggle suspend on F1 and suspend-exempt, toggle fishing on F2 and
   suspend-exempt).
2. **Given** an action rebound to an unused key, **When** settings are reloaded,
   **Then** the new binding persists.
3. **Given** a request to bind an action to a key already used by another action,
   **When** it is applied, **Then** it is rejected and both existing bindings are
   unchanged.

---

### Edge Cases

- What happens when a synthesized key matches a bound key while focused? It is
  recognized as self-originated and not intercepted (US2).
- What happens when focus changes between key-down and key-up? Interception is
  decided per event against the current focus state; the engine never leaves a key
  suppressed globally.
- What happens when the persisted bindings contain a conflict (two actions on one
  key)? The conflict is reported as a notice and the affected actions fall back to
  their defaults rather than loading an ambiguous table.
- What happens when the persisted bindings name an unknown action or key? The
  unknown entry is reported as a notice and ignored, leaving that action at its
  default.
- What happens on Linux without the required input permission? The engine reports
  that it cannot start interception rather than failing silently, and the
  permission requirement is documented for packaging.

## Requirements *(mandatory)*

### Functional Requirements

Interception and focus:

- **FR-001**: The engine MUST intercept only configured physical keys, and only
  while the ESO window holds keyboard focus.
- **FR-002**: On a bound key while focused, the engine MUST suppress the original
  keystroke so it does not reach the game or other applications.
- **FR-003**: Keys that are not bound MUST pass through untouched.
- **FR-004**: On a bound key while focused, the engine MUST hand off exactly one
  action event to the worker for that press.
- **FR-005**: The engine MUST NOT intercept input when the ESO window does not
  hold focus, and MUST never intercept globally.

Threading contract:

- **FR-006**: The interception path MUST NOT sleep or perform blocking or timed
  work; it may only classify the key, suppress it when bound, and hand off an
  event.
- **FR-007**: All timed work (delays, sequences, synthesis timing) MUST run on a
  dedicated worker separate from the interception path.

Synthesis and recursion breaking:

- **FR-008**: The engine MUST be able to synthesize key output on request.
- **FR-009**: Synthesized input MUST be distinguishable from real input so the
  engine never intercepts its own output; a synthesized event MUST NOT be handed
  off as a new action.
- **FR-010**: The self-origin distinction MUST be per event, so a later real press
  of the same key is still intercepted normally.

Suspend:

- **FR-011**: When suspended, the engine MUST disable interception for all
  bindings except those marked suspend-exempt.
- **FR-012**: The suspend-exempt bindings (toggle suspend, toggle fishing) MUST
  remain intercepted while suspended and while focused.
- **FR-013**: Resuming from suspend MUST restore full interception.

Keybinding model:

- **FR-014**: The engine MUST maintain a binding table mapping each action to one
  physical key, with the documented default bindings from master specification
  section 6.4.
- **FR-015**: The engine MUST reject any assignment that would map two actions to
  the same key, leaving the prior bindings unchanged.
- **FR-016**: Bindings MUST persist across restarts through the Config Store, and
  MUST default to the documented defaults when absent.
- **FR-017**: A persisted binding table that is conflicting, or that names an
  unknown action or key, MUST be reported as a notice and MUST fall back to
  defaults for the affected entries rather than loading an ambiguous table.

Testability seam:

- **FR-018**: The engine MUST expose a single input abstraction with interchangeable
  backends, including a test backend that drives synthetic key events and observes
  suppression and hand-off without real operating-system hooks, so the
  safety-critical behaviors are testable without the game or hardware.

Platform startup:

- **FR-019**: When a platform backend cannot begin interception (for example a
  Linux permission error), the engine MUST surface that it could not start rather
  than failing silently.

Key transitions, identity, and hand-off:

- **FR-020**: For a bound key while focused, the engine MUST suppress both the
  key-down and the key-up, hand off exactly one action on the initial key-down,
  and ignore auto-repeat until the key is released.
- **FR-021**: The binding table MUST persist as an additive settings section that
  is backward compatible: settings written before this slice (without the section)
  MUST load the default bindings without requiring a schema version bump.
- **FR-022**: Keys MUST be identified in the core and the binding table by a
  platform-neutral key identifier; each backend maps between that identifier and
  its native scan or key code.
- **FR-023**: The hand-off from the interception path to the worker MUST use a
  non-blocking bounded channel; when the channel is full the action MUST be
  dropped with a warning rather than blocking the interception path.

### Key Entities *(include if feature involves data)*

- **Action**: A named operation the engine can be triggered to perform (skills 1
  to 5, ultimate, synergy, toggle suspend, toggle fishing). This slice classifies
  and hands off actions; their execution belongs to later slices.
- **Binding**: A mapping from an action to one physical key, with a suspend-exempt
  flag. The set of bindings is the binding table.
- **Key Event**: A single physical or synthetic key transition, carrying the key
  identity (a platform-neutral key identifier), the transition (down or up), and
  whether it originated from the engine (self-originated) or from a real device.
- **Engine State**: Whether the game is focused and whether the engine is
  suspended. Interception decisions are made against this state per event.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: While focused, 100 percent of bound-key presses are suppressed and
  produce exactly one handed-off action; 0 percent of unbound-key presses are
  suppressed.
- **SC-002**: While unfocused, 0 percent of key presses are intercepted or
  suppressed.
- **SC-003**: A synthesized key that is also bound produces 0 handed-off actions
  (no self-interception), verifiable while focused.
- **SC-004**: The interception path performs no blocking or timed work, verifiable
  by inspecting that its handling contains only classification, suppression, and
  hand-off.
- **SC-005**: While suspended, 0 percent of non-exempt bound keys are intercepted
  and 100 percent of suspend-exempt keys remain intercepted.
- **SC-006**: A rebinding that collides with an existing key is rejected in 100
  percent of cases with both bindings left unchanged.

## Assumptions

- Scope is master specification section 6 only: interception, suppression,
  hand-off, synthesis with recursion breaking, suspend semantics, and the
  keybinding model. Executing the handed-off actions (the Weave Engine), fishing,
  PixelBeacon, and the GUI are out of scope and handled in later slices. The GUI
  capture-style rebinding control is later; this slice provides the underlying
  conflict-rejecting binding operations.
- The engine builds on the S001 foundations: bindings persist through the Config
  Store, and engine events are recorded through the Logging subsystem under its
  privacy rule (key identities may be logged at debug and below, subject to the
  input suppression control).
- The realizations are platform-specific and behind the input abstraction: the
  Windows backend uses a low-level keyboard hook with injected-input flagging and
  synthesized input, and raises timer resolution for the worker lifetime; the
  Linux backend grabs the physical keyboard device and synthesizes through a
  virtual device below the display server, requiring input-group membership or an
  equivalent permission rule that packaging documents.
- The operating-system hook wiring is a thin adapter that cannot be exercised in an
  automated, headless test; the safety-critical decision logic (classification,
  suppression, recursion breaking, focus and suspend gating, hand-off) lives behind
  the input abstraction and is tested with the test backend. The thin OS adapters
  are validated manually against the running game.
- Focus determination uses the operating system's notion of the focused window;
  identifying the ESO window is done by the platform backend.
