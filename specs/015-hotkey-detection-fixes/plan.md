# Implementation Plan: Hotkey and Weapon-Bar Detection Fixes

**Branch**: `015-hotkey-detection-fixes` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/015-hotkey-detection-fixes/spec.md`

## Summary

Three runtime-wiring defects surfaced in live in-game testing. The F1 (suspend)
and F2 (fishing) hotkeys have no effect because their actions are handed off to
the weave worker, which maps both to `None`; the actual suspend and fishing state
is owned by `AppModel::apply_intent`, reached only by the GUI buttons. The
weapon-bar readout is permanently "Not detected" because the decode path (correct
in code) has no observability, so the live cause cannot be diagnosed.

This slice routes the two toggle actions from the weave worker to the GUI intent
path over a dedicated channel, so a hotkey and its button reach one shared state
with identical persistence and display. It adds reader diagnostics so the
operator can confirm in-game whether the weapon-bar signal is present and
decoding. It changes no weave timing and synthesizes no input from detection.
The correctness logic (the action split and the intent mapping) is pure and unit
tested; the egui and thread plumbing stay thin. The live pixel signal is
validated in-game by the operator using the new diagnostics, an explicit
follow-up rather than a design blocker.

## Technical Context

**Language/Version**: Rust (2021); no addon logic changes (the B3 encoding is
already correct and unchanged)

**Primary Dependencies**: `eframe`/`egui` 0.35, `tracing`, `serde`/`serde_json`;
`std::sync::mpsc` for the new toggle channel

**Storage**: No schema change. `state.json` (SessionState: suspended, fishing)
is written exactly as today; hotkey toggles reuse the existing session-mark path.

**Testing**: `cargo test --all --locked` for the action split (weave vs app
toggle), the toggle-to-intent mapping, and the unchanged decode/observe tests.
The live pixel signal is validated in-game (no headless harness).

**Target Platform**: Windows 10/11 x64 and Linux x64

**Project Type**: single-crate desktop application plus a companion addon

**Performance Goals**: toggles are rare; the channel is unbounded but sees a
handful of messages per session. Diagnostics add no per-sample work at the
default log level.

**Constraints**: no blocking work on the input hook thread (unchanged);
suppression stays scoped to the focused game window (unchanged); thin egui layer;
UTF-8 no BOM, LF, no em-dashes or en-dashes.

**Scale/Scope**: one new internal channel, one action classifier, one GUI drain
loop, and reader diagnostics. No new persisted state, no new pixel block, no
contract changes.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development (NON-NEGOTIABLE)**: PASS. Built through the full
  spec-kit sequence as slice 015; traces to master spec sections 7.1 (input
  activation and suspend), 8 (fishing control), and 9.3/16 R1 (weapon-bar
  signal). The `/speckit.analyze` gate runs before implementation.
- **II. Safety-Critical Surfaces Are Sacrosanct (NON-NEGOTIABLE)**: PASS. The
  `InputEngine::classify` path (recursion breaking, focus-scoped suppression,
  non-blocking hand-off) is not touched. Fishing still degrades to disabled on
  `SignalLost`. Toggling suspend or fishing via a hotkey performs no input while
  the game is unfocused, exactly like the restored-session path. Existing
  safety-critical tests are unchanged and still required.
- **III. Test-First With Explicit Seams**: PASS. The new action classifier
  (toggle vs weave) and the toggle-to-intent mapping are pure functions with
  unit tests written first. The decode/observe seam already has tests; the
  diagnostics are `tracing` calls that are inert without a subscriber and do not
  change decode behavior.
- **IV. CI Parity Before Every Commit (NON-NEGOTIABLE)**: PASS. fmt, clippy
  `-D warnings`, and `cargo test --all --locked` run in the foreground to
  completion before the commit.
- **V. Bounded Scope: Outside The Game**: PASS. No game memory or network. The
  addon is unchanged. Diagnostics read only the already-sampled pixels.

No pinned artifacts change except `CHANGELOG.md` (an Added line plus dated
Decisions entries). No pixel-bus or reader contract changes: the B3 encoding and
the sampled geometry are unchanged, and diagnostics are not part of any contract.

## Key Decisions (autopilot decision log)

**D1: Route the two toggle actions to the GUI intent path, not the weave worker.**
Alternatives: (a) handle toggles in the weave worker by mutating the shared
`InputEngine`/`FishingController` directly; (b) split the action channel at the
`InputEngine` source into two senders; (c) forward toggle actions from the weave
worker to the GUI over a second channel and apply them through the existing
`apply_intent`. Chosen: (c). It reuses `AppModel::apply_intent`, so a hotkey and
its button provably share one state, one persistence-mark, and one display path
(satisfies FR-001, FR-002, FR-005, FR-012, SC-001, SC-002 by construction). It
leaves the safety-critical `InputEngine` untouched (rejecting (b), which would
perturb the most-tested core and its `new` signature). It avoids splitting the
toggle logic across two owners and the double-toggle risk of (a), and keeps
persistence marking where it already lives. Cost: the GUI reflects and persists
the change on its next frame; the idle repaint cadence is 250 ms, so worst-case
reflection latency is a quarter second, consistent with how the app already
surfaces worker-driven state (fishing sub-state, weapon bar). Recorded in
CHANGELOG Decisions.

**D2: Split the action stream at the consumer (the weave worker), not the source.**
The weave worker already drains the single action receiver. It classifies each
action with a pure helper: skill/ultimate/synergy actions go to
`WeaveEngine::handle` as today; `ToggleSuspend`/`ToggleFishing` are forwarded to
the app-toggle channel. This confines the change to the worker plus one tested
pure function and does not alter `InputEngine`.

**D3: `ToggleFishing` maps to `SetFishing(!current)`.** There is no dedicated
"toggle fishing" intent; the GUI uses `SetFishing(bool)`. The GUI reads the live
fishing on/off state and applies the negation, matching the button's behavior and
keeping one intent vocabulary.

**D4: Diagnostics via `tracing` inside `observe`, layered by level.** Detected
and lost weapon-bar transitions log at DEBUG with the decoded bar and classes;
raw per-sample block values (including a present heartbeat with a non-decoding
B3, the exact in-game failure signature) log at TRACE. Nothing weapon-related
logs at INFO or above on an idle sample, satisfying FR-009 and SC-005 without
default-level spam. `tracing` is inert without a subscriber, so decode tests are
unaffected.

**D5: No addon or decode change.** The addon B3 encoding and the reader decode
match byte for byte and the runtime path already samples B3. The permanent
"Not detected" is therefore an in-game signal condition (most likely a
PixelBeacon install predating slice 014, or a capture/tolerance mismatch), found
with D4 diagnostics and resolved by the operator (reinstall the addon via the app
and validate in-game). This slice makes that diagnosis possible; it does not
guess at a decode change that the code does not need.

## Project Structure

### Documentation (this feature)

```text
specs/015-hotkey-detection-fixes/
├── plan.md, research.md, data-model.md, quickstart.md
├── checklists/{requirements.md, wiring.md}
└── tasks.md   (from /speckit-tasks)
```

No new `contracts/` directory: this slice changes no contract.

### Source Code (repository root)

```text
src/input/action.rs   # add a pure classifier: Action::is_app_toggle() (true for
                      #   ToggleSuspend/ToggleFishing), unit tested
src/app/routing.rs    # add pure map: app_toggle_intent(action, fishing_on) ->
                      #   Option<UiIntent>; unit tested
src/main.rs           # create the app-toggle channel; the weave worker forwards
                      #   toggle actions to it instead of handing them to
                      #   WeaveEngine::handle; pass the receiver into the GUI
src/app/ui.rs         # EsoWeaveApp holds the toggle receiver; update() drains it
                      #   (try_recv) and applies each via model.apply_intent
src/app/mod.rs        # (if needed) a small accessor for current fishing-on state
                      #   used to negate ToggleFishing; no new intent variants
src/pixelbus/mod.rs   # observe(): tracing diagnostics (DEBUG transitions, TRACE
                      #   raw samples); no decode behavior change
CHANGELOG.md          # Added lines + dated Decisions entries (D1, D4)
```

**Structure Decision**: Extend the existing single crate. The toggle-routing
correctness lives in two pure functions (`is_app_toggle`, `app_toggle_intent`)
behind unit tests; `main.rs` and `ui.rs` carry only the thread and frame
plumbing. Diagnostics live with the decode they describe.

## Complexity Tracking

No constitution violations. The one new channel and drain loop are the minimum
needed to reach the existing intent path from a background thread; the rejected
alternatives (source-side split, worker-side direct mutation) were heavier or
touched the safety-critical core. No new persisted state or contract surface is
added.
