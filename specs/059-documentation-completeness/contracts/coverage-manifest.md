# Contract: Coverage Manifest

## Purpose

The S059 coverage manifest is the auditable crosswalk between issue #81, current
implementation evidence, tests, and the one canonical published destination for
each obligation.

## Required Fields

Every manifest row must provide:

```text
id
area
audience
statement
destination
source_evidence[]
test_evidence[]
coverage
labels[]
```

Allowed coverage values are `Missing`, `Partial`, `Covered`, and `Deferred`.
Completion requires every in-scope row to be `Covered` or `Deferred`, and every
Deferred row to name a follow-up issue or a clearly recorded implementation
discrepancy. Deferral cannot be used for missing prose whose behavior is already
known.

## Baseline Logic Rows

The final manifest must contain at least these stable obligations. Destinations
may be refined during planning, but each row must still have exactly one
canonical page.

| ID | Obligation | Baseline destination | Evidence anchors |
| --- | --- | --- | --- |
| LOG-001 | Physical input is intercepted only for an active, focused ESO window and otherwise passes through | `docs/src/concepts/input-safety.md` | `InputEngine::classify`; `unfocused_never_intercepts`; `inactive_game_never_intercepts_even_when_focused` |
| LOG-002 | Self-originated input is never intercepted | `docs/src/concepts/input-safety.md` | `Origin::SelfOriginated`; `self_originated_event_is_never_intercepted` |
| LOG-003 | Key releases retire held state, passed-through presses keep their releases passed, and repeat Down events enqueue once | `docs/src/concepts/input-safety.md` | `InputEngine::classify`; `pass_through_release_while_unfocused_retires_held_key`; `auto_repeat_down_hands_off_only_once` |
| LOG-004 | The interception callback performs only non-blocking classification and handoff | `docs/src/concepts/input-safety.md` | `InputEngine::hand_off`; `full_channel_drops_without_blocking` |
| LOG-005 | Queue overload or disconnect drops the action after suppression and emits a warning | `docs/src/concepts/input-safety.md` | `TrySendError::Full`; `TrySendError::Disconnected`; `full_channel_drops_without_blocking` |
| LOG-006 | Application toggles remain reachable through suspend and safety gates but do not themselves authorize generated work | `docs/src/concepts/action-authorization.md` | `Action::suspend_exempt`; `the_gate_exempts_the_toggle_hotkeys`; `life_gate_defaults_closed_passes_weaves_and_exempts_toggles` |
| LOG-007 | Missing, invalid, or lost menu evidence never becomes Gameplay | `docs/src/concepts/action-authorization.md` | `route_reader_event` MenuGate branch; `menu_event_only_on_change_and_clears_on_loss`; `a_menu_gate_event_gates_the_potion_controller_directly` |
| LOG-008 | Weave handoff rechecks active slot, life, roll dodge, world, travel, and cooldown before starting | `docs/src/features/weaving.md` | `WeaveEngine::handle`; `queued_weave_requires_alive_and_is_not_replayed_after_recovery`; `queued_weave_requires_safe_world_and_inactive_travel_without_replay` |
| LOG-009 | Mid-sequence synthesis stops on shared gate closure, releases held output, does not replay, and cancelled work does not consume cooldown | `docs/src/features/weaving.md` | `RealSink::emit`; `RealSink::wait`; `real_sink_observes_roll_gate_closure_during_a_wait`; `a_gate_cancelled_sequence_does_not_consume_global_cooldown` |
| LOG-010 | Fishing follows Disabled, Armed, Waiting, Reeling, and Recast states with event and deadline transitions | `docs/src/features/fishing.md` | `FishingController::on_event`; `FishingController::tick`; `cast_reel_recast_cycle` |
| LOG-011 | Fishing menu gating defers reel and recast without advancing past unsent input | `docs/src/features/fishing.md` | `FishingController::tick`; `a_gated_controller_defers_the_reel_instead_of_sending_it`; `a_gated_controller_defers_the_recast_too` |
| LOG-012 | Fishing safety cancellation, focus/runtime pause, SignalLost reset, and their distinct recovery policies are explicit | `docs/src/features/fishing.md` | `FishingController::block_for_safety`; `set_game_environment`; `non_alive_cancels_pending_fishing_without_replay_and_keeps_request`; `signal_loss_while_focus_paused_applies_the_existing_reset_policy` |
| LOG-013 | Auto Potion states name the first current blocker in implementation order and Unknown never authorizes required evidence | `docs/src/features/auto-potion.md` | `potion::evaluate`; `s043_effective_state_distinguishes_ready_triggered_and_every_runtime_family`; `missing_and_corrupt_state_blocks_fail_closed` |
| LOG-014 | Auto Potion uses OR across enabled resource watches, inclusive thresholds, explicit usable potion classification, cooldown readiness, and retry floor | `docs/src/features/auto-potion.md` | `low_resource`; `it_fires_when_every_condition_is_satisfied`; `the_comparison_is_at_or_below_not_strictly_below`; `the_retry_interval_bounds_the_rate_independently_of_the_cooldown` |
| LOG-015 | The cross-feature authorization table distinguishes physical pass-through from permission to synthesize | `docs/src/concepts/action-authorization.md` | Input, weave, fishing, potion, and routing tests named above |
| LOG-016 | Runtime, focus, freshness, and surface observations remain independent and Gameplay requires every positive axis | `docs/src/concepts/game-observation.md` | `GameObservations::context`; `gameplay_requires_every_authoritative_axis` |
| LOG-017 | Runtime reduction and installation reconciliation preserve Unknown and Ambiguous rather than guessing | `docs/src/concepts/game-observation.md` | `ProcessObservation::runtime`; `reconcile`; `runtime_reduction_honors_game_precedence_over_launcher`; `distinct_roots_and_conflicting_strong_providers_are_ambiguous` |
| LOG-018 | Life, world, travel, roll-dodge, and sprint state machines identify entry, invalidation, watchdog, and recovery rules | `docs/src/concepts/game-observation.md` | PixelBeacon addon handlers; beacon contract tests; pixel-bus decoder tests |
| LOG-019 | Process probing remains responsive even when the reader uses a slow interval | `docs/src/concepts/game-observation.md` | `runtime_probe_delay_ms`; `runtime_probe_caps_long_reader_intervals_and_is_immediately_due` |

## Protocol and Addon Rows

| ID | Obligation | Baseline destination | Evidence anchors |
| --- | --- | --- | --- |
| LOG-020 | Layout authority, current and legacy versions, complete-fit validation, and same-frame capture are exact | `docs/src/reference/pixel-bus-protocol.md` | `decode_layout_header`; `recognized_header_corruption_never_falls_back_to_legacy`; `reader_prepares_twice_on_acquisition_then_once_when_steady` |
| LOG-021 | Every payload block B0 through B28 has exact position, marker, checksum, value, and unavailable semantics | `docs/src/reference/pixel-bus-protocol.md` | PixelBus constants and decoders; `block_center_and_capture_dims_match_contract_table`; addon contract tests |
| LOG-022 | Corrupt recognized layout clears payload immediately while missing heartbeat raises SignalLost after the bounded timeout | `docs/src/reference/pixel-bus-protocol.md` | `PixelBusReader::lose_signal`; `invalid_recognized_header_suppresses_payload_sampling`; `heartbeat_then_signal_loss_then_recovery` |
| LOG-023 | Recovery republishes unchanged safety evidence and closes gates before controller locks while opening only after synchronization | `docs/src/reference/pixel-bus-protocol.md` | `invalidate_safety_observations`; `route_reader_safety_gate`; `safety_invalidation_fails_closed_and_republishes_unchanged_values`; routing safety tests |
| LOG-024 | PixelBeacon install, update, redeploy, manifest edit, and removal remain confined and managed-marker gated | `docs/src/features/pixelbeacon.md` | Beacon lifecycle functions; `install_writes_only_under_pixelbeacon`; `uninstall_refuses_unmanaged_folder`; `redeploy_refuses_unmanaged_folder` |
| LOG-025 | Addon API upkeep uses stored/default numeric versions and a bounded non-blocking game-version check without guessing | `docs/src/features/pixelbeacon.md` | `api_check::run_check`; `never_downgrades_or_churns`; `fetch_error_is_swallowed_and_default_used` |

## Configuration, Logging, Platform, and Delivery Rows

| ID | Obligation | Baseline destination | Evidence anchors |
| --- | --- | --- | --- |
| CFG-001 | `config.json` contains settings only and each module owns validation of its section | `docs/src/reference/configuration.md` | `config::Settings`; module load/store tests |
| CFG-002 | Missing, corrupt, older, newer, unknown-key, and invalid-field settings paths are distinct | `docs/src/reference/configuration.md` | `config::load`; `missing_file_yields_defaults_without_notice`; `corrupt_file_is_preserved_and_defaults_returned`; migration tests |
| CFG-003 | `state.json` ownership and invalid-state fallback differ from `config.json` corruption preservation | `docs/src/reference/configuration.md` | `SessionState`; `state::load`; app session-state tests |
| CFG-004 | Settings and state writes settle after the configured interval and force a final close flush | `docs/src/reference/configuration.md` | session scheduler and application model; `scheduler_flushes_only_after_settle`; `flush_session_now_writes_immediately_without_settle` |
| CFG-005 | Window geometry is point-based, clamped, and kept reachable on supported platforms | `docs/src/reference/configuration.md` | `sanitize_geometry`; geometry tests in config state and UI sizing |
| CFG-006 | The persisted logging level filters both file and in-memory capture, the Live Log reflects that level, and the ring evicts oldest records at capacity | `docs/src/reference/logging.md` | `SinkLayer::on_event`; `runtime_level_change_takes_effect`; `ring_buffer_is_independent_and_evicts_oldest`; app log tests |
| CFG-007 | Input content and resource verbosity policies, platform log paths, monthly rotation, and file-sink failure behavior are explicit | `docs/src/reference/logging.md` | `log_input`; `FileSink::write_line`; platform log paths; logging tests |
| PLT-001 | Windows and Linux discovery, focus, interception, synthesis, capture, display, permission, and geometry differences are explicit | `docs/src/concepts/scope-and-platform.md` | `src/game`; `src/input`; `src/pixelbus`; `src/platform`; platform-scoped tests where available |
| PLT-002 | X11, XWayland, and pure Wayland behavior distinguishes input support from window focus and capture support | `docs/src/concepts/scope-and-platform.md` | `src/input/linux.rs`; `src/pixelbus/linux.rs`; `src/platform/linux.rs` |
| REL-001 | Installation, update, uninstall, package choice, permission, and checksum instructions exist for each supported platform | `docs/src/getting-started/installation.md` | packaging metadata; release assets; CI release workflow |
| REL-002 | The stable release state model documents verification, platform builds, checksums, publication, and failure boundaries | `docs/src/development/release-and-packaging.md` | `.github/workflows/release.yml`; release scripts and tests |
| REL-003 | The command-by-command maintainer ritual remains canonical only in `docs/project/releasing.md` | `docs/src/development/release-and-packaging.md` | constitution pinned-artifact rule; project release record |
| REL-004 | Startup panic behavior distinguishes pre-GUI user notification from post-GUI log-only handling and platform output | `docs/src/getting-started/troubleshooting.md` | `src/startup/mod.rs`; startup unit tests |
| REL-005 | Test strategy explains pure engines, platform traits, addon byte-contract checks, headless UI checks, shell checks, and cross-platform CI | `docs/src/development/test-strategy.md` | tests tree; CI workflow |
| REL-006 | A task-oriented diagnosis path covers runtime, focus, addon status, geometry, heartbeat, per-feature blockers, input backend, config, and logging | `docs/src/getting-started/troubleshooting.md` | user-visible state projections and failure tests across modules |

## Deferred Discrepancy Rows

These rows prevent documentation from falsely asserting repaired behavior:

| ID | Discrepancy | Evidence | S059 treatment |
| --- | --- | --- | --- |
| DEF-001 | Linux uinput capabilities omit `E` and `F3` although supported mappings use them | `src/input/linux.rs` constants and mappings | Record follow-up; do not claim verified Linux parity for these keys |
| DEF-002 | Running weave sink gates omit focus, suspension, and menu state | `WeaveGates`; `RealSink`; absent mid-sequence tests | Record follow-up; distinguish interception-time checks from mid-sequence checks |
| DEF-003 | Menu-gate source and test comments describe obsolete fail-open behavior | `InputEngine::set_menu_gated` comment; pixel-bus menu-loss test comment; current routing | Correct published explanation and record comment cleanup outside docs-only scope |

## Validation Rules

The documentation policy must fail when:

- an identifier is duplicated or removed without an explicit manifest update;
- an in-scope row remains Missing or Partial at completion;
- a destination does not exist or is absent from `SUMMARY.md`;
- a safety row lacks test evidence and lacks an explicit test-gap note;
- a Deferred row has neither evidence nor a follow-up disposition;
- two pages claim canonical ownership of the same obligation;
- evidence paths do not exist or stable anchors are absent; or
- a destination contains none of the substantive anchors named by its row.
