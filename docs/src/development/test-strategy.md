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
| Encounter capture addon | `tests/encounter_addon.rs` with vendored Lua 5.1 | Exact production Lua state transitions, explicit one-shot consent, all event families, anonymous persisted data, sequence and time, bounded loss, interruption recovery, teardown, and source confinement |
| Encounter import and raw store | `tests/encounter_import.rs` plus shared parser unit tests | Non-executing bounded parsing, terminal and privacy invariants, complete and truthful partial handoffs, canonical identity, immutable transactional storage, collision preservation, explicit deletion, consistent backup, corruption handling, and the 100,000-event ceiling |
| Encounter metrics | `tests/encounter_metrics.rs`, `tests/encounter_cli.rs` | Deterministic DPS, effective HPS, damage share, clipped effect uptime, cast order, explicit loss quality, player attribution, zero-duration behavior, catalog compatibility, later ID resolution, atomic no-clobber output, and CLI publication |
| Local icon cache | `tests/icon_cache.rs` | Synthetic PNG/DDS decode, hostile paths and links, source preservation, deterministic objects, explicit fallback mappings, immutable publication, tamper rejection, and manifest privacy |
| Catalog candidate pipeline | `tests/catalog_pipeline.rs`, `tests/catalog_pipeline_workflow.rs` | Exact channel and version identity, dual network gates, source-cache integrity, deterministic allowlisted reports, failed-publication preservation, and read-only automation authority |
| User catalog updates | `tests/catalog_update.rs`, `tests/app_ui_sizing.rs` | Redacted inspection, Live/PTS availability, trusted-origin acknowledgement, immutable install, cancellation, restart resolution, file locking, collector save boundary, receipts, recovery, rollback, and accessible modal controls |
| Documentation capture sandbox | `tests/documentation_capture.rs` | Exact scene catalog, deterministic model truth, dark and light themes, narrow and wide viewports, production isolation, repository-contained output, and zero emitted input without live ESO or a native window |

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
| Runtime authorization epoch | `InputEngine::authorization_epoch`, `WeaveGates::admits`, focus, suspension, and menu setters | [S060](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/060-safety-boundaries/spec.md) tests prove epoch invalidation, admitted-sequence cancellation, and held-output release across the [input](https://github.com/h8rt3rmin8r/eso-weave/blob/main/tests/input_engine.rs) and [weave](https://github.com/h8rt3rmin8r/eso-weave/blob/main/tests/weave_engine.rs) boundaries |
| Recovery ordering | `route_reader_safety_gate`, `route_reader_event` | `safety_preroute_defers_recovery_until_worker_state_is_synchronized` |
| Death episode ordering | `PixelBusReader::observe`, `InputEngine::death_epoch` | Recovered Alive is published after an actionable baseline, and [S067](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/067-death-recovery-safety/spec.md) proves one death-epoch advance per open-to-closed transition in the [input tests](https://github.com/h8rt3rmin8r/eso-weave/blob/main/tests/input_engine.rs) |
| Fishing signal loss | `FishingController::on_event` | `signal_lost_from_every_active_state_disables_without_emitting` |
| Auto Potion first blocker | `potion::evaluate` | [S043](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/043-auto-potion-restoration/spec.md) proves Ready, Triggered, and every runtime blocker family in the [Auto Potion tests](https://github.com/h8rt3rmin8r/eso-weave/blob/main/tests/potion.rs) |
| Auto Potion death retry | `AutoPotionController::tick` | [S067](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/067-death-recovery-safety/spec.md) proves recovery starts a complete new retry episode in the [Auto Potion tests](https://github.com/h8rt3rmin8r/eso-weave/blob/main/tests/potion.rs) |
| Managed removal | `beacon::uninstall` | `uninstall_refuses_unmanaged_folder` |
| Managed lifecycle writes | `beacon::status`, `install_with_options`, `redeploy_for_block_size` | [S060](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/060-safety-boundaries/spec.md) proves unproven targets, unmanaged manifests, and unproven links cannot be mutated in the [PixelBeacon tests](https://github.com/h8rt3rmin8r/eso-weave/blob/main/tests/beacon.rs) |
| Fishing suspension | `FishingController::set_suspended` | [S060](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/060-safety-boundaries/spec.md) proves initial-cast refusal and cancellation of pending reel and recast work in the [Fishing tests](https://github.com/h8rt3rmin8r/eso-weave/blob/main/tests/fishing.rs) |
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

## Documentation capture sandbox

The S083 capture target is development-only and cannot be entered through the
shipped application. Its ordinary test invocation validates the exact seven-scene
catalog, scene models, and isolation rules without initializing a graphics adapter
or retaining generated files:

```bash
cargo test --locked --test documentation_capture
```

An explicit repository-local destination arms headless PNG generation. The command
below writes 28 variants and `capture-manifest.json` below `target/` without opening
a native window, resolving a personal configuration directory, installing an addon,
capturing the desktop, or constructing an operating-system input backend:

```bash
cargo test --locked --test documentation_capture -- --capture-to target/documentation-captures
```

Generated captures are review evidence for issue #125 and are not source assets in
S083. The target rejects repository escape, the repository root, non-directory
collisions, and symlinked destinations. Every rendered scene also proves its input
action receiver stayed empty.

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

```bash
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
