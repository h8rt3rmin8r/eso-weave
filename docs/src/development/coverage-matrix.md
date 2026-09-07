# Documentation Coverage Matrix

This developer crosswalk identifies the canonical published page for each
correctness-bearing area. It is an audit aid, not a second specification. The
linked page owns the explanation, while the source symbols and tests identify
the evidence reviewed for S059.

## Runtime and authorization logic

| IDs | Canonical page | Source evidence | Test evidence |
| --- | --- | --- | --- |
| LOG-001 through LOG-005 | [Input Safety](../concepts/input-safety.md) | `InputEngine::classify`, `InputEngine::hand_off`, `TrySendError` | `unfocused_never_intercepts`, `self_originated_event_is_never_intercepted`, `auto_repeat_down_hands_off_only_once`, `full_channel_drops_without_blocking` |
| LOG-006, LOG-007, LOG-015 | [Action Authorization](../concepts/action-authorization.md) | `Action::suspend_exempt`, `route_reader_event`, controller routing | `the_gate_exempts_the_toggle_hotkeys`, `menu_event_only_on_change_and_clears_on_loss`, feature authorization tests |
| LOG-008, LOG-009 | [Weaving](../features/weaving.md) | `WeaveEngine::handle`, `RealSink::emit`, `RealSink::wait` | `queued_weave_requires_safe_world_and_inactive_travel_without_replay`, `real_sink_observes_roll_gate_closure_during_a_wait`, `a_gate_cancelled_sequence_does_not_consume_global_cooldown` |
| LOG-010 through LOG-012 | [Fishing](../features/fishing.md) | `FishingController::on_event`, `tick`, `set_game_environment`, `block_for_safety` | `cast_reel_recast_cycle`, menu-deferral tests, `signal_loss_while_focus_paused_applies_the_existing_reset_policy` |
| LOG-013, LOG-014 | [Auto Potion](../features/auto-potion.md) | `potion::evaluate`, `low_resource`, `AutoPotionController::tick` | `s043_effective_state_distinguishes_ready_triggered_and_every_runtime_family`, threshold and retry tests |
| LOG-016 through LOG-019 | [Game Observation and Safety State](../concepts/game-observation.md) | `ProcessObservation::runtime`, `GameObservations::context`, addon state handlers | game-state reduction and context tests, embedded addon contract tests |

The [State Machines](state-machines.md) page provides the compact transition
tables and decision sequences for reviewers who need to follow those areas
together.

## Protocol, settings, and delivery

| IDs | Canonical page | Evidence boundary |
| --- | --- | --- |
| LOG-020 through LOG-023 | [Pixel Bus Protocol](../reference/pixel-bus-protocol.md) | Layout decoding, B0 through B28 payload decoding, signal loss, invalidation, and recovery republication |
| LOG-024, LOG-025 | [PixelBeacon](../features/pixelbeacon.md) | Managed addon lifecycle and bounded API-version upkeep |
| CFG-001 through CFG-005 | [Configuration](../reference/configuration.md) | Settings and state ownership, migration, write scheduling, and geometry restoration |
| CFG-006, CFG-007 | [Logging](../reference/logging.md) | Global capture level, Live Log projection, ring eviction, files, and sink failure |
| PLT-001, PLT-002 | [Scope and Platform Support](../concepts/scope-and-platform.md) | Windows, X11, XWayland, and pure Wayland capability boundaries |
| REL-001 | [Installation](../getting-started/installation.md) | Platform packages, permissions, update, removal, and checksums |
| REL-002, REL-003 | [Release and Packaging](release-and-packaging.md) | Stable release state sequence and separation from the maintainer command ritual |
| REL-004, REL-006 | [Troubleshooting](../getting-started/troubleshooting.md) | Startup failure handling and ordered user diagnosis |
| REL-005 | [Test Strategy](test-strategy.md) | Pure engines, deterministic clocks, platform traits, addon contracts, headless interface checks, shell contracts, and CI |

## Recorded implementation discrepancies

These rows describe current limitations. Their presence in documentation does
not claim that implementation work is complete.

| Issue | Current discrepancy | Documentation treatment |
| --- | --- | --- |
| [#92](https://github.com/h8rt3rmin8r/eso-weave/issues/92) | Queued automation is not stopped at every authorization boundary. Running weave omits focus, suspension, and menu gates; Fishing does not route suspension to its initial cast or pending timers. | Marked as a known defect in Action Authorization, Input Safety, State Machines, Architecture, and Test Strategy. |
| [#93](https://github.com/h8rt3rmin8r/eso-weave/issues/93) | Linux uinput capabilities omit shipped `E` and `F3` mappings. | Linux parity is qualified in Scope and Platform Support and Test Strategy. |
| [#94](https://github.com/h8rt3rmin8r/eso-weave/issues/94) | PixelBeacon Update can overwrite unmanaged addon content after removal refuses it. | The managed-marker boundary and current defect are identified without promising a fix. |
| [#95](https://github.com/h8rt3rmin8r/eso-weave/issues/95) | Some saved settings do not apply live, and Fishing Interact Key is not exposed in Settings. | Runtime application and configuration limits are identified on their canonical pages. |
| [#96](https://github.com/h8rt3rmin8r/eso-weave/issues/96) | Shipped latency help, Live Log prose, and menu-evidence comments contain stale descriptions. | Published behavior follows current implementation and labels the stale shipped text as a defect. |

## How to use the matrix

Start with the canonical page for the contract. Use the named symbol or test to
verify implementation details. When implementation and documentation differ,
the discrepancy remains explicit and linked until its issue is resolved; an
Unknown or unavailable observation never becomes positive authorization merely
because prose is incomplete.
