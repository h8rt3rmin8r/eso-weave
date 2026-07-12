# Research: Hotkey and Weapon-Bar Detection Fixes

**Feature**: 015-hotkey-detection-fixes | **Date**: 2026-07-11

This slice is a wiring and observability fix; the research is a root-cause trace
of three defects against the existing code, not new technology selection.

## R-1: Why F1/F2 hotkeys have no effect

- Default bindings map F1 to `Action::ToggleSuspend` and F2 to
  `Action::ToggleFishing` (`src/input/action.rs`), both `suspend_exempt`.
- `InputEngine::classify` correctly intercepts the focused key, marks it
  newly-pressed (once per physical press), and hands the action off on the action
  channel (`src/input/mod.rs`).
- The action channel is drained only by the weave worker
  (`src/main.rs`: `while let Ok(action) = actions.recv()`), which calls
  `WeaveEngine::handle`. In the engine, `index_for_action` returns `None` for both
  toggle actions (`src/weave/mod.rs`), so they are silently ignored.
- The real suspend state is `InputEngine.suspended` (an `AtomicBool`), toggled
  only by `AppModel::apply_intent(UiIntent::ToggleSuspend)`. The real fishing
  state is `FishingController`, toggled only by
  `apply_intent(UiIntent::SetFishing(_))`. Both are GUI-button paths.

Conclusion: the hotkey action never reaches the state the GUI mutates. Decision
D1/D2/D3 route the toggle actions to that same intent path.

## R-2: How the GUI reflects and persists state (so hotkeys get parity for free)

- `EsoWeaveApp::update` derives the view from live state every frame and calls
  `ctx.request_repaint_after(250 ms)`, so the window repaints on a fixed cadence
  even when the game holds focus and the GUI is in the background.
- `apply_intent(ToggleSuspend)` flips `InputEngine.suspended` and calls
  `scheduler.mark_session`. `apply_intent(SetFishing)` sets the controller and
  marks the session too.
- `maybe_flush` writes `state.json` from live state (`current_session_state`)
  once the session store has settled, and `current_session_state` reads live
  `input.is_suspended()` and the controller state. So any change reached through
  `apply_intent` is persisted identically, whatever its trigger.

Conclusion: draining the toggle channel in `update` and applying via
`apply_intent` yields button parity for display and persistence with no new
persistence logic. Worst-case reflection latency is the 250 ms idle cadence,
already how worker-driven state (fishing sub-state, weapon bar) surfaces.

## R-3: Why the weapon bar reads "Not detected", and what is fixable headless

- The runtime path samples B3 at `weapon_point` (56, 8) and calls `observe`,
  which decodes B3 only inside the `if heartbeat` branch (B0 status present) and
  emits `WeaponBar` only on a change (`src/pixelbus/mod.rs`).
- `decode_weapon_bar` validates the green marker `0x5A` within tolerance and
  unpacks red (front/back class nibbles) and blue (active bar). The addon
  `renderWeapon` writes exactly that encoding (`addon/PixelBeacon/PixelBeacon.lua`:
  red = front*16 + back, green = `WEAPON_MARKER`, blue = bar), positioned at
  `BLOCK_PX * 3` (x = 48, center 56). The two sides match byte for byte.
- The GUI already renders `weapon_bar_view(active_bar, front, back)` and shows the
  detected values or "Not detected".

Conclusion: there is no headless-detectable decode or wiring bug. The permanent
"Not detected" is a live signal condition, most plausibly:
1. an installed PixelBeacon predating slice 014 (no B3 block rendered), resolved
   by reinstalling the addon through the app; or
2. the B0 heartbeat not matching in-game (gates the whole strip); or
3. a capture/color or tolerance mismatch on the live surface.
All three are invisible today because the reader logs nothing. Decision D4 adds
layered `tracing` so the operator can tell these apart in-game without a debugger.

## R-4: Diagnostic level choice

- Default runtime level is INFO. To satisfy FR-009/SC-005 (diagnosable, but no
  per-idle-sample spam at the default level): weapon-bar detected and lost
  transitions log at DEBUG; raw per-sample block bytes (including the diagnostic
  case of a present heartbeat with a non-decoding B3) log at TRACE. The operator
  raises the level temporarily to validate in-game.
- Constitution constraint honored: input contents are never logged; these are
  pixel-bus bytes, not keystrokes, and nothing is logged while suspended changes
  this (the reader is not keystroke data).

## Alternatives considered and rejected

- Splitting the action channel inside `InputEngine` (two senders): rejected;
  perturbs the safety-critical core and its `new` signature and every test that
  builds an engine, for no benefit over a consumer-side split.
- Handling toggles directly in the weave worker by mutating shared state:
  rejected; duplicates the toggle semantics outside `apply_intent`, risks
  double-toggle, and would need a parallel persistence-mark path.
- A diff-based persistence model (GUI detects any live-state divergence and marks
  dirty): rejected as a larger change to the persistence model than this slice
  warrants; the intent path already marks correctly.
