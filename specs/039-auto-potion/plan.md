# Implementation Plan: Auto-Potion

**Branch**: `main` (trunk-based) | **Date**: 2026-07-27 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/039-auto-potion/spec.md`

## Summary

Add `src/potion/mod.rs`: an `AutoPotionController` that presses a configured
quickslot key when an enabled resource is at or below its own threshold, the
active quickslot holds a usable potion, its cooldown is zero, and a minimum retry
interval has elapsed, while the application is neither suspended nor menu-gated.
Add `Key::F3` and `Action::ToggleAutoPotion` to drive it, per-resource settings to
configure it, and a status readout to show it.

**This is the first feature in the project that synthesizes input from a
beacon-derived value**, and therefore the first on a constitution NON-NEGOTIABLE
surface. The design response is to add no new input path: synthesis goes through
the existing input engine, the controller is modelled on the fishing controller,
and the trigger rule is a pure function so every blocking condition can be tested
in isolation.

## Technical Context

**Language/Version**: Rust (pinned by `rust-toolchain.toml`).

**Primary Dependencies**: none new.

**Storage**: settings only (thresholds, enables, key, interval). The enable state
itself is deliberately **not** persisted; see research.md R7.

**Testing**: `cargo test --all --locked`, with a new `tests/potion.rs` carrying
the truth-table coverage SC-002 requires, plus additions to
`tests/input_engine.rs` (the new key and action), `tests/app_settings.rs`
(persistence), and `tests/app_view_model.rs` (the intent and the readout).

**Target Platform**: Windows 10/11 x64 and Linux x64. No platform-specific code.

**Project Type**: desktop application, single Rust crate.

**Performance Goals**: unchanged. FR-021 forbids a cadence change; the rule is a
handful of comparisons on the existing worker tick.

**Constraints**: no new input path, no new thread or timer, no blocking work on
the hook thread; text stays UTF-8 without BOM, LF, free of em-dashes and
en-dashes.

**Scale/Scope**: one new module, one new key, one new action, one new intent, one
new settings group, one new status row.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Status | Evidence |
| --- | --- | --- |
| I. Spec-Driven Development | PASS | Traces to `docs/plans/plan-012.md` slice 039 and issue #20. Full sequence run; `analyze` runs before `implement`. |
| II. Safety-Critical Surfaces | **PASS, and this is the feature's central concern.** | Adds no input path: synthesis goes through `InputBackend::synthesize` via a sink seam identical to fishing's, so focus scoping and recursion flagging are inherited, not re-implemented (FR-008). The menu gate is applied to the controller directly, carrying the slice 032 lesson (FR-009). Suspend is a checked condition, not an emergent property (FR-010). Signal loss disables (FR-011). The controller ticks on the existing worker loop; no thread, no timer, nothing on the hook thread (FR-012). No existing safety test is touched (FR-014). |
| III. Test-First With Explicit Seams | PASS | The rule is a pure function and the sink is a trait with a mock, so the entire feature is exercised with a virtual clock and no game, window, or input hardware. Every behavior lands as a failing test first. |
| IV. CI Parity Before Every Commit | PASS | `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`, foreground, watched to completion. |
| V. Bounded Scope: Outside The Game | PASS | Reads no process memory and no network traffic. Consumes signals that already exist on the screen-signal contract; changes neither the contract nor the addon (FR-020). |

**Post-design re-check**: PASS, unchanged. One new module, no new dependency, no
new thread, no new input path.

**Complexity Tracking**: not applicable.

## Design Decisions

Recorded under the autopilot decision policy.

### D1: Unknown is never low

A reading the companion cannot make never satisfies a threshold, for resources,
for the quickslot, and for the cooldown. The failure directions are asymmetric:
treating unknown as low fires potions on every beacon hiccup, addon reload, and
loading screen, while treating it as not-low means the feature quietly does
nothing during an outage. See research.md R1. Stated three times in the spec on
purpose, because it is the decision most likely to be reversed as a tidy-up.

### D2: The retry interval and the quickslot cooldown are both required

They cover different windows. The cooldown is read from the screen and does not
update until at least one sampling interval after the key is pressed; the retry
interval is the floor that covers exactly that lag. See research.md R2.

### D3: The rule is a pure function returning a typed reason

`evaluate(...) -> Result<(), Block>` rather than a boolean on a stateful object.
SC-002 requires eight blocking conditions tested in isolation with every other
condition satisfied, and a typed reason is what lets a test assert *which*
condition blocked. A bare boolean would let a test pass because a different
condition happened to be false, which is the failure the safety checklist's
CHK012 names.

### D4: No state machine

Unlike `FishingController` there is no sequence to be partway through. What is
taken from fishing is the seam, the gate, and the enable, not the five-state
machine. Adding states would add a concept carrying no information.

### D5: The enable does not survive a restart

Deliberately inconsistent with suspend and fishing, which are both restored. A
restored fishing session does nothing until the operator stands at a fishing hole;
a restored auto-potion waits silently to press a key days later in a fight the
operator does not associate with this application. See research.md R7.

### D6: Suspend is pushed into the controller, not pulled from the engine

Keeps the rule a pure function of explicit inputs and makes the suspended case
testable without constructing an input engine.

## Phase 0: Research

Complete. See [research.md](research.md): R1 unknown-is-not-low, R2 the retry
interval, R3 the controller shape, R4 the pure rule, R5 the key and the two
predicates, R6 where it ticks, R7 not restoring the enable.

## Phase 1: Design

Complete. See [data-model.md](data-model.md) and
[contracts/trigger-rule.md](contracts/trigger-rule.md).

## Implementation Outline

Test-first throughout.

1. **The key and the action.** `Key::F3` across all five representations;
   `Action::ToggleAutoPotion` with `ALL`, `as_str`, `default_key`, and **both**
   predicates. The existing enumerating unit tests fail first.
2. **The rule.** `evaluate` plus `Block`, with the truth-table tests: each
   blocking condition in isolation with all others satisfied, the OR across the
   three resources, unknown-never-low across the full threshold range, and the
   at-or-below boundary.
3. **The controller.** Enable, gate, suspend, last-attempt, tick, sink seam, mock
   and real sinks.
4. **Configuration.** `AutoPotionConfig` load and store with degradation notices,
   into `SettingsForm`.
5. **Wiring.** `UiIntent::SetAutoPotion`, the routing branch (menu gate and
   signal loss), the worker-loop tick in `src/main.rs`, and the hotkey path.
6. **Interface.** The status readout, the toggle control, and the settings group.
7. **Documentation.** Master specification, README, changelog.

## Risks

- **A blocking condition passes its test for the wrong reason.** Mitigated by D3:
  the typed `Block` reason is asserted, not just the absence of a keypress.
- **The two `Action` predicates are missed.** Mitigated by their existing tests,
  which enumerate the expected sets and fail loudly. Called out in the tasks.
- **The feature fires during the cooldown-reporting lag.** This is exactly what
  D2 exists for, and SC-005 pins it against a virtual clock.
- **Scope creep into potion selection or restore-type awareness.** Explicitly out
  of scope; the previous slice established the restore types are not
  machine-readable, and the operator's per-stat enables are the substitute.
