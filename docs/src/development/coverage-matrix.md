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
| LOG-008, LOG-009 | [Weaving](../features/weaving.md) | `AuthorizationEpoch`, `InputEngine::authorization_epoch`, `WeaveEngine::handle`, `RealSink::emit`, `RealSink::wait` | `s060_queued_weave_epoch_is_invalid_after_each_runtime_gate_closes`, `s060_transient_suspend_closure_cancels_an_admitted_sequence`, `s060_focus_closure_stops_new_presses_but_releases_held_output`, shared safety-gate tests |
| LOG-010 through LOG-012 | [Fishing](../features/fishing.md) | `FishingController::set_suspended`, `set_enabled`, `on_event`, `tick`, `set_game_environment`, `block_for_safety` | `s060_suspension_refuses_initial_cast_and_preserves_request`, `s060_suspension_cancels_pending_reel_without_replay`, `s060_suspension_cancels_recast_and_timeout_paths`, menu-deferral and signal-loss tests |
| LOG-013, LOG-014 | [Auto Potion](../features/auto-potion.md) | `potion::evaluate`, `low_resource`, `AutoPotionController::tick` | `s043_effective_state_distinguishes_ready_triggered_and_every_runtime_family`, threshold and retry tests |
| LOG-016 through LOG-019 | [Game Observation and Safety State](../concepts/game-observation.md) | `ProcessObservation::runtime`, `GameObservations::context`, addon state handlers | game-state reduction and context tests, embedded addon contract tests |

The [State Machines](state-machines.md) page provides the compact transition
tables and decision sequences for reviewers who need to follow those areas
together.

## Protocol, settings, and delivery

| IDs | Canonical page | Evidence boundary |
| --- | --- | --- |
| LOG-020 through LOG-023 | [Pixel Bus Protocol](../reference/pixel-bus-protocol.md) | Layout decoding, B0 through B28 payload decoding, signal loss, invalidation, and recovery republication |
| LOG-024, LOG-025 | [PixelBeacon](../features/pixelbeacon.md) | `BeaconStatus::Unmanaged`, ownership-gated lifecycle writers, managed in-place update, and bounded API-version upkeep; `s060_install_refuses_unproven_targets_without_mutation`, `s060_api_refresh_does_not_write_an_unmanaged_manifest`, `s060_lifecycle_operations_do_not_follow_an_unproven_link`, and App Model action tests |
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
| [#93](https://github.com/h8rt3rmin8r/eso-weave/issues/93) | Linux uinput capabilities omit shipped `E` and `F3` mappings. | Linux parity is qualified in Scope and Platform Support and Test Strategy. |
| [#95](https://github.com/h8rt3rmin8r/eso-weave/issues/95) | Some saved settings do not apply live, and Fishing Interact Key is not exposed in Settings. | Runtime application and configuration limits are identified on their canonical pages. |
| [#96](https://github.com/h8rt3rmin8r/eso-weave/issues/96) | Shipped latency help, Live Log prose, and menu-evidence comments contain stale descriptions. | Published behavior follows current implementation and labels the stale shipped text as a defect. |

## How to use the matrix

Start with the canonical page for the contract. Use the named symbol or test to
verify implementation details. When implementation and documentation differ,
the discrepancy remains explicit and linked until its issue is resolved; an
Unknown or unavailable observation never becomes positive authorization merely
because prose is incomplete.
