# Research: Deterministic Screenshot Sandbox

## Existing seam

`EsoWeaveApp::frame_ui` already renders the complete application from `AppModel` through an `egui::Ui` without an `eframe::Frame`, native window, or platform startup. `tests/app_ui_sizing.rs` proves this seam with `egui_kittest::Harness` and installs the same bundled fonts as production.

Reusing that seam avoids a parallel documentation UI and keeps screenshots sensitive to real interface changes.

## Target isolation decision

Three placements were evaluated:

1. A production command-line mode is easy to invoke but creates an accidental production entry path and must defend every platform side effect at runtime.
2. A Cargo feature still compiles capture authority into a production-capable binary and can be enabled by `--all-features`.
3. A named `harness = false` integration-test target links application library code and development dependencies but is not part of the shipped binary.

Option 3 is selected. Execution without the dedicated `--capture-to` marker performs catalog and isolation checks only, including when Cargo supplies an ordinary test filter. A caller must pass that marker and a destination after Cargo's `--` separator to arm rendering.

## Renderer decision

The pinned `egui_kittest` 0.36.1 source documents a WGPU test renderer, but enabling that crate feature fails dependency resolution because the published package requests unavailable `pollster ^1.0`. Replacing or locally impersonating that dependency would make the test graph misleading.

The harness therefore retains dependency-free egui frame driving and supplies a test-local renderer built directly on the matching `egui-wgpu` 0.36.1 API plus available `pollster` 0.4. It follows the upstream headless pattern: no display handle, predictable renderer options, CPU-adapter preference, an offscreen texture, and readback into the existing `image` crate. This is selected over a custom CPU painter because reproducing egui clipping, fonts, textures, blending, and tessellation would create a second rendering engine. Image comparison is not used as a cross-platform CI gate because adapter and driver differences can legitimately change pixels.

## Fixture decision

The harness builds the real `AppModel` over:

- `InputEngine` with an observed but never consumed action receiver;
- real `WeaveEngine`, `GameState`, and `AutoPotionController` instances seeded with fixed observations;
- `MockFishingSink`, with no platform input backend;
- an in-memory log and no configuration directory;
- a beacon path override below the explicit capture output root;
- the checked-in read-only catalog when a healthy catalog line is needed.

The fixture writes only minimal synthetic PixelBeacon manifest shapes needed by the existing read-only status classifier. It does not call install, update, uninstall, discovery, or any game-running probe.

## Scene and variant decision

The ordered catalog contains:

1. `first-launch`
2. `healthy-system-state`
3. `pixelbeacon-lost`
4. `pixelbeacon-unmanaged`
5. `weaving-configuration`
6. `auto-potion-ready`
7. `auto-potion-blocked`

Every scene supports dark and light themes and 760 by 1000 narrow and 1280 by 900 wide canvases. Generated names are `{scene}--{theme}--{viewport}.png`.

## Output safety decision

The caller supplies the destination. The harness rejects repository escape, the repository root itself, existing non-directory targets, any symlink in the destination chain, and symlinks or special files at exact generated names. PNGs and the manifest are staged before publication. It creates only the exact variant PNGs, `capture-manifest.json`, and an automatically removed `.fixture-data-*` subtree. Validation-only filesystem probes use a writable temporary mock repository and do not require writes to the source checkout. Existing unrelated files remain untouched.

## Test seams

The target validates:

- the exact scene catalog and variant cardinality;
- scene-specific `AppView` expectations before painting;
- absence of forbidden platform, capture, lifecycle, configuration, and production-entry symbols in the test source and production manifest surfaces;
- output-root containment and symlink rejection;
- empty input-engine output after every frame;
- canonical receipt order and dimensions;
- a second same-process render producing equal image bytes for the representative first scene.
