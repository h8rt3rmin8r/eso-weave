# Implementation Plan: Pixel-Bus Block Size Single Source of Truth

**Branch**: `main` (trunk-based) | **Date**: 2026-07-24 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/028-pixelbus-block-size/spec.md`

## Summary

Make `block_px` (the physical-pixel size of each PixelBeacon square) the single
authoritative value shared between the deployed addon and the companion reader.
Every other geometry number is derived from it by a documented formula on both
sides: the reader's four block-center read points, the Windows screen-capture
region, and the addon's drawn strip width and per-block placement. Expose
`block_px` as an advanced setting in the existing Pixel Beacon settings cluster;
when it changes, update the stored value and drive a managed re-deploy of the
addon (rewriting the deployed Lua's `BLOCK_PX` line, mirroring the existing
manifest `APIVersion` templating), while preserving the managed marker and all
other addon content. The default stays 16 so existing and fresh installs are
byte-for-byte unchanged.

## Technical Context

**Language/Version**: Rust (edition per repo `rust-toolchain.toml`); embedded
Lua addon (ESO client Lua, executed in-game only).

**Primary Dependencies**: `serde` / `serde_json` (opaque settings sections),
`windows-sys` (GDI capture), `x11rb` (Linux sampling), `egui`/`eframe`
(settings UI). No new dependencies.

**Storage**: JSON config file (`config.json`), UTF-8 no BOM, LF, pretty; the
`pixelbus` section is an opaque `serde_json::Value` with additive fields.

**Testing**: `cargo test --all --locked`; pure decoders/geometry unit-tested with
crafted samples; addon templating tested against the embedded Lua string;
re-deploy safety tested against an injected AddOns root.

**Target Platform**: Windows 10/11 x64 and Linux x64.

**Project Type**: Single Rust crate (desktop companion app) plus a bundled Lua
addon under `addon/PixelBeacon/`.

**Performance Goals**: No change to sampling cadence; the capture region shrinks
with smaller block sizes (never grows beyond the current 64x16 at the default).

**Constraints**: No behavior change at the default size; safety-critical
managed-marker guarantees preserved; text hygiene (no em/en dashes, UTF-8 no
BOM, LF).

**Scale/Scope**: Four fixed blocks; supported block sizes are even integers 2 to
32 inclusive.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development**: This slice runs the full spec-kit sequence;
  artifacts land under `specs/028-pixelbus-block-size/`. PASS.
- **II. Safety-Critical Surfaces Are Sacrosanct**: The re-deploy reuses the
  existing `install`/`uninstall` path. Uninstall's managed-marker verification is
  untouched. Re-deploy is refused for unmanaged installs; no unmanaged folder is
  written or deleted; all writes stay confined to the `PixelBeacon` subfolder.
  New tests assert the marker survives `render_lua` templating and that a
  block-size-driven re-deploy never touches an unmanaged folder. PASS.
- **III. Test-First With Explicit Seams**: Geometry, validation, and templating
  are pure functions tested before wiring; the `SurfaceSampler` seam is reused
  for capture-dimension tests. PASS.
- **IV. CI Parity Before Every Commit**: fmt, clippy (-D warnings), and
  `cargo test --all --locked` run in the foreground before commit. PASS.
- **V. Bounded Scope: Outside The Game**: No new in-game capability; the addon
  keeps to the existing screen-signal contract (same four blocks, only their
  size becomes configurable). PASS.

No violations; Complexity Tracking not required.

## Project Structure

### Documentation (this feature)

```text
specs/028-pixelbus-block-size/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/
│   └── geometry.md      # Phase 1 output: the block-size derivation contract
├── checklists/
│   ├── requirements.md  # Spec quality checklist
│   └── geometry.md      # Requirements-quality checklist
└── tasks.md             # Phase 2 output (/speckit-tasks)
```

### Source Code (repository root)

```text
src/
├── pixelbus/
│   ├── mod.rs           # ReaderConfig: add block_px SSOT + derivation helpers
│   │                    #   (NUM_BLOCKS, block_center, capture_dims,
│   │                    #    sanitize_block_px); derive the four read points;
│   │                    #    RawPixelBus/load/store gain block_px
│   ├── windows.rs       # GdiSampler: capture dims from block_px, not consts
│   └── linux.rs         # unchanged (reads whatever points it is given)
├── beacon/
│   └── mod.rs           # render_lua(block_px); install() takes block_px;
│                        #   managed-only re-deploy entry point
├── app/
│   ├── settings_form.rs # ReaderConfig already carried; block_px rides along
│   ├── mod.rs           # apply_settings: drive re-deploy on block_px change;
│   │                    #   install_beacon passes block_px
│   ├── ui.rs            # advanced block-size control in the beacon cluster
│   └── strings.rs       # label/help strings for the block-size setting
├── main.rs              # resolve_sampler passes reader_config.block_px
addon/PixelBeacon/
└── PixelBeacon.lua      # remains the template; BLOCK_PX line is rewritten;
                         #   header comment de-hardcoded (cosmetic)
tests/
├── pixelbus.rs          # formula-derived points for sizes 2/4/8/16; capture
│                        #   dims; sanitize_block_px cases
└── beacon.rs            # render_lua rewrites only BLOCK_PX + preserves marker;
                         #   re-deploy refuses Unmanaged
```

**Structure Decision**: Single-crate layout is unchanged; edits are localized to
the existing `pixelbus`, `beacon`, and `app` modules plus the bundled addon.

## Key design decisions

1. **`block_px` replaces the four stored read-point fields on `ReaderConfig`.**
   Storing both a size and the points invites drift; instead `ReaderConfig` holds
   `block_px` and exposes the four points as methods computed by `block_center`.
   This makes the single source of truth structural, not conventional.
2. **Reader geometry applies at startup, like tolerance and intervals.** The
   pixel-bus worker owns `reader_config` by move (`main.rs`), and
   `reload_from_settings` does not update it; runtime tolerance/interval changes
   already take effect only on the next app start. `block_px` follows the same
   rule. A settings-apply notice states that a size change takes effect after
   `/reloadui` (addon redraw) and an app restart (reader geometry).
3. **Re-deploy is automatic on apply, managed-only.** When the applied `block_px`
   differs from what is deployed and the install is managed, `apply_settings`
   re-runs `install` with the new size (writing the addon's `BLOCK_PX` to match
   the stored value). Unmanaged or not-installed: no write, a notice explains.
   This keeps the on-disk addon and the stored setting in lockstep so both sides
   agree after reload/restart.
4. **Template the Lua, do not codegen.** `render_lua(block_px)` rewrites only the
   `local BLOCK_PX = N` line, mirroring `rewrite_api_version`. The addon already
   derives its strip width (`BLOCK_PX * 4`) and block placement (`BLOCK_PX * N`)
   from `BLOCK_PX`, so no other addon edits are needed for correctness.
5. **Validation clamps to a supported even value with a notice.** Even integers
   2..=32; odd rounds down to the next even, out-of-range clamps to the nearest
   bound, wrong type falls back to the default. Never a panic (config discipline).
6. **Default 16, zero behavior change.** `DEFAULT_BLOCK_PX = 16`; with no user
   change the derived points are `(8,8),(24,8),(40,8),(56,8)` and the capture
   region is 64x16, identical to today.

See [research.md](research.md) for rationale and alternatives, and
[contracts/geometry.md](contracts/geometry.md) for the exact derivation contract.

## Complexity Tracking

No constitution violations; no entries.
