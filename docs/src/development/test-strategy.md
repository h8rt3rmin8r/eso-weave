# Test Strategy

ESO Weave keeps correctness-bearing logic behind deterministic seams, then adds
smaller platform and workflow contracts around it. This page explains what each
layer proves and what it does not prove.

## Test layers

| Layer | Primary evidence | What it proves |
| --- | --- | --- |
| Pure engines and controllers | `tests/input_engine.rs`, `tests/weave_engine.rs`, `tests/fishing.rs`, `tests/potion.rs` | Decisions, state transitions, ordering, deadlines, cancellation, and no replay without live ESO or device input |
| Protocol codec and reader | `tests/pixelbus.rs`, `tests/pixelbus_display.rs` | Exact colors, markers, checksums, versions, geometry, freshness, aggregate events, corruption, and display reconciliation |
| Embedded addon contract | `tests/beacon.rs` parses `addon/PixelBeacon/PixelBeacon.lua` and its manifest | Rust and Lua constants agree, required ESO APIs and events remain present, lifecycle invalidation is represented, embedded files stay managed and versioned |
| Application routing and view | `tests/app_view_model.rs`, `tests/app_settings.rs`, `tests/app_session_state.rs` | One event reaches the correct consumers, intents converge, settings and state persist, and visible states remain truthful |
| Headless interface geometry | `tests/app_ui_sizing.rs` through `egui_kittest` | Responsive cards, controls, meter geometry, disclosure, text allocation, scaling, and layout boundaries without a GPU |
| Logging and startup | `tests/logging.rs`, `tests/app_log_view.rs`, unit tests in `src/startup/mod.rs` | Runtime filtering, ring eviction, file format, input suppression, log presentation, and pre-GUI notification gating |
| Packaging and release scripts | `scripts/release-notes.test.sh`, `scripts/validate-debian-package.test.sh`, and release workflow verification | Release-note grammar, bounds, extraction, tag-version agreement, changelog presence, required Debian control fields, and asset gating |
| Documentation policy | `.github/scripts/docs-policy.test.mjs` | Navigation, links, offline assets, lifecycle boundaries, preservation, coverage, aliases, and prose constraints |
| Bundled documentation | `tests/documentation.rs`, `build.rs`, and release-profile CI builds | Immutable lookup, content types, strict routing, HTTP bounds, reuse, concurrent reads, shutdown, browser seams, and production embedding |
| Catalog compiler and runtime | `tests/catalog_compiler.rs`, `tests/catalog_runtime.rs` | Strict data-only input, provenance, constraints, deterministic hashes and bytes, atomic rollback, typed read-only queries, and graceful degradation |
| Catalog packaging | `tests/catalog_packaging.rs` | MSI, Debian, AppImage, and tarball catalog paths plus baseline rights boundaries |
| Discovery collector | `tests/collector_addon.rs`, `tests/collector_lifecycle.rs`, `tests/collector_import.rs` | Dedicated addon identity, explicit bounded collection, hostile non-executing parsing, truthful coverage, atomic staging, and marker-gated lifecycle isolation |

## Deterministic seams

Platform operations implement traits such as input backends and surface samplers.
Tests substitute recording sinks, mock clocks, mock pixel samples, temporary
directories, and injected observations. A controller test advances `now_ms`
explicitly instead of sleeping. The headless UI harness renders frames at
specified sizes and scale factors.

These seams make negative properties reviewable:

- no physical suppression outside the focused active game;
- no self-interception of synthesized input;
- no blocking handoff from the input callback;
- no generated Down operation after a tested shared gate closes;
- no replay when life, world, travel, or roll state recovers;
- no Fishing output after SignalLost;
- no Auto Potion output from Unknown resources, quickslot, cooldown, life,
  world, or travel;
- no PixelBeacon removal without the managed marker; and
- no partial or corrupt Pixel Bus layout accepted as current.

## Safety evidence map

| Contract | Source symbol | Representative test |
| --- | --- | --- |
| Focus-scoped suppression | `InputEngine::classify` | `focus_scoping_is_unconditional_regardless_of_the_gate` |
| Recursion rejection | `Origin::SelfOriginated` | `self_originated_event_is_never_intercepted` |
| Non-blocking overload | `InputEngine::hand_off` | `full_channel_drops_without_blocking` |
| Mid-sequence cancellation | `RealSink::emit`, `RealSink::wait` | `real_sink_observes_roll_gate_closure_during_a_wait` |
| Runtime authorization epoch | `InputEngine::authorization_epoch`, `WeaveGates::admits`, focus, suspension, and menu setters | `s060_queued_weave_epoch_is_invalid_after_each_runtime_gate_closes`, `s060_transient_suspend_closure_cancels_an_admitted_sequence`, `s060_focus_closure_stops_new_presses_but_releases_held_output` |
| Recovery ordering | `route_reader_safety_gate`, `route_reader_event` | `safety_preroute_defers_recovery_until_worker_state_is_synchronized` |
| Death episode ordering | `PixelBusReader::observe`, `InputEngine::death_epoch` | `recovered_alive_is_last_after_a_forced_actionable_baseline`, `s067_death_epoch_advances_once_per_open_to_closed_life_transition` |
| Fishing signal loss | `FishingController::on_event` | `signal_lost_from_every_active_state_disables_without_emitting` |
| Auto Potion first blocker | `potion::evaluate` | `s043_effective_state_distinguishes_ready_triggered_and_every_runtime_family` |
| Auto Potion death retry | `AutoPotionController::tick` | `s067_recovery_starts_a_complete_new_retry_episode` |
| Managed removal | `beacon::uninstall` | `uninstall_refuses_unmanaged_folder` |
| Managed lifecycle writes | `beacon::status`, `install_with_options`, `redeploy_for_block_size` | `s060_install_refuses_unproven_targets_without_mutation`, `s060_api_refresh_does_not_write_an_unmanaged_manifest`, `s060_lifecycle_operations_do_not_follow_an_unproven_link` |
| Fishing suspension | `FishingController::set_suspended` | `s060_suspension_refuses_initial_cast_and_preserves_request`, `s060_suspension_cancels_pending_reel_without_replay`, `s060_suspension_cancels_recast_and_timeout_paths` |
| Protocol compatibility | `decode_layout_header` | `recognized_header_corruption_never_falls_back_to_legacy` |

## Platform coverage

Continuous integration runs strict Clippy and the locked test suite on Windows
and Ubuntu. Formatting runs on Ubuntu. Pure logic tests exercise both sides of
platform-independent contracts, but they do not prove a live Windows hook,
Linux evdev grab, uinput capability set, X11 compositor, XWayland focus, GPU
capture, or live ESO client.

Platform modules therefore require narrow source review and, where possible,
contract tests over exported tables and translation helpers. A passing mock test
must never be described as live-game verification.

Bundled-documentation tests use only loopback sockets and compiled fixture bytes.
They prove hostile paths cannot become filesystem reads and service lifetime is
bounded. Windows and Linux CI also run the production Cargo build after installing
exact mdBook tools, while documentation policy proves generated local references
resolve before embedding.

## Settings runtime coverage

S062 tests Fishing configuration changes in every active phase, including
zero-output cancellation, unchanged no-op behavior, and explicit re-enable with
the new key. Reader tests cover live-subset preservation, tolerance invalidation,
worker wake, latest-value coalescing, timeout, and disconnect. App Model and
headless UI tests connect those seams to persistence and the Interact Key control.
S061 separately provides exhaustive Linux mapping plus string and menu-evidence
behavior anchors.

## Local verification sequence

Run the documentation gates first, then the full source parity suite:

```text
node --test .github/scripts/docs-policy.test.mjs
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
bash scripts/validate-debian-package.test.sh
git diff --check
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all --locked
cargo build --release --locked --bin eso-weave
```

The release-note and Debian package shell contracts also run in Linux CI. The
Debian suite requires `dpkg-deb`. A missing local tool is a recorded limitation,
not a substitute for its hosted check.
