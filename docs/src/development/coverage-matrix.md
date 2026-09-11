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
| LOG-008, LOG-009 | [Weaving](../features/weaving.md) | `AuthorizationEpoch`, `InputEngine::authorization_epoch`, `WeaveEngine::handle`, `RealSink::emit`, `RealSink::wait` | [S060](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/060-safety-boundaries/spec.md) epoch invalidation, admitted-sequence cancellation, and held-output release in the [input](https://github.com/h8rt3rmin8r/eso-weave/blob/main/tests/input_engine.rs) and [weave](https://github.com/h8rt3rmin8r/eso-weave/blob/main/tests/weave_engine.rs) tests |
| LOG-010 through LOG-012 | [Fishing](../features/fishing.md) | `FishingController::set_suspended`, `set_enabled`, `on_event`, `tick`, `set_game_environment`, `block_for_safety` | [S060](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/060-safety-boundaries/spec.md) initial-cast refusal, pending-reel cancellation, and recast cancellation in the [Fishing tests](https://github.com/h8rt3rmin8r/eso-weave/blob/main/tests/fishing.rs), plus menu-deferral and signal-loss coverage |
| LOG-013, LOG-014 | [Auto Potion](../features/auto-potion.md) | `potion::evaluate`, `low_resource`, `AutoPotionController::tick` | [S043](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/043-auto-potion-restoration/spec.md) effective-state coverage in the [Auto Potion tests](https://github.com/h8rt3rmin8r/eso-weave/blob/main/tests/potion.rs), plus threshold and retry coverage |
| LOG-016 through LOG-019 | [Game Observation and Safety State](../concepts/game-observation.md) | `ProcessObservation::runtime`, `GameObservations::context`, addon state handlers | game-state reduction and context tests, embedded addon contract tests |

The [State Machines](state-machines.md) page provides the compact transition
tables and decision sequences for reviewers who need to follow those areas
together.

## Protocol, settings, and delivery

| IDs | Canonical page | Evidence boundary |
| --- | --- | --- |
| LOG-020 through LOG-023 | [Pixel Bus Protocol](../reference/pixel-bus-protocol.md) | Layout decoding, B0 through B28 payload decoding, signal loss, invalidation, and recovery republication |
| LOG-024, LOG-025 | [PixelBeacon](../features/pixelbeacon.md) | `BeaconStatus::Unmanaged`, ownership-gated lifecycle writers, managed in-place update, and bounded API-version upkeep; [S060](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/060-safety-boundaries/spec.md) unproven-target, unmanaged-manifest, and link-boundary coverage in the [PixelBeacon tests](https://github.com/h8rt3rmin8r/eso-weave/blob/main/tests/beacon.rs), plus App Model action tests |
| CFG-001 through CFG-005 | [Configuration](../reference/configuration.md) | Settings and state ownership, migration, write scheduling, and geometry restoration |
| DEF-005 | [Settings Reference](../reference/settings.md) | `FishingController::apply_config`, `PixelBusReader::apply_live_config`, `wait_for_live_config`, App Model runtime application, Fishing phase tests, reader update tests, and headless Interact Key coverage |
| CFG-006, CFG-007 | [Logging](../reference/logging.md) | Global capture level, Live Log projection, ring eviction, files, and sink failure |
| PLT-001, PLT-002 | [Scope and Platform Support](../concepts/scope-and-platform.md) | Windows, X11, XWayland, and pure Wayland capability boundaries |
| REL-001 | [Installation](../getting-started/installation.md) | Platform packages, permissions, update, removal, and checksums |
| REL-002, REL-003 | [Release and Packaging](release-and-packaging.md) | Stable release state sequence and separation from the maintainer command ritual |
| REL-004, REL-006 | [Troubleshooting](../getting-started/troubleshooting.md) | Startup failure handling and ordered user diagnosis |
| REL-005 | [Test Strategy](test-strategy.md) | Pure engines, deterministic clocks, platform traits, addon contracts, headless interface checks, shell contracts, and CI |

## How to use the matrix

Start with the canonical page for the contract. Exact symbols shown here are
direct implementation evidence on this dedicated crosswalk. Compact behavioral
links replace work-slice-prefixed test names; the source tree and the
[evidence manifest](https://github.com/h8rt3rmin8r/eso-weave/blob/main/docs/project/content-coverage.json)
retain all exact identifiers and stable representative relationships,
respectively. An Unknown or unavailable observation never becomes positive
authorization merely because prose is incomplete.
