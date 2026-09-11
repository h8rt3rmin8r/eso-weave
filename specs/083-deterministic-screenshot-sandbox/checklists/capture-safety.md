# Capture Safety Checklist: Deterministic Screenshot Sandbox

**Purpose**: Define the isolation evidence required before S083 implementation.

**Created**: 2026-09-11

**Feature**: [spec.md](../spec.md)

## Production Isolation

- [x] Capture authority exists only in an explicit Cargo test target.
- [x] No application argument, feature, environment switch, or startup branch enables capture.
- [x] Ordinary tests validate without rendering or generated capture writes; synthetic fixture files remain in an automatically removed temporary directory.
- [x] No native application window is created.

## Input and Screen Safety

- [x] No platform input backend or hook constructor is imported.
- [x] No synthesis backend is imported or called.
- [x] No Pixel Bus screen sampler is imported or called.
- [x] The input-engine action receiver is asserted empty after every render.

## Filesystem Safety

- [x] An explicit output root is required before any persistent generated write.
- [x] The output root must resolve inside the repository and must not be a symlink.
- [x] Fixture files remain under a clearly synthetic subtree.
- [x] No addon lifecycle function or user configuration resolver is called.
- [x] The output root itself is never recursively deleted.

## Determinism and Privacy

- [x] Scene order, identifiers, labels, themes, and viewports are fixed.
- [x] Clocks, paths, providers, resources, cooldowns, and status values are fixed.
- [x] No account, character, machine, current-date, random, or profile value is rendered.
- [x] The receipt is canonical pretty JSON with LF and a trailing newline.
