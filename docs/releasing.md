# Releasing ESO Weave

This is the one authoritative procedure for cutting a release. It exists so the steps are never
re-derived or improvised per session. Cutting a release is a single action; the pipeline does the
rest and fails loudly if anything required is missing.

## Cut a release

Prerequisite: every change since the last release is logged under `## [Unreleased]` in
`CHANGELOG.md`. That section becomes the release notes, so keep it current as features land.

Run one command:

```bash
cargo release X.Y.Z --execute
```

That command (configured in `release.toml`):

1. Bumps the crate version in `Cargo.toml`. This is the single source of the application
   version, the MSI version, and the PixelBeacon addon version embedded in the binary.
2. Rewrites `CHANGELOG.md`, renaming `## [Unreleased]` to `## [X.Y.Z] - DATE` and opening a fresh
   empty `## [Unreleased]`.
3. Commits the change as `release: vX.Y.Z`.
4. Tags `vX.Y.Z` and pushes the commit and tag.

Pushing the tag triggers `.github/workflows/release.yml`.

Note: the README version badge is a static shields.io badge (`version-X.Y.Z-2ea44f`). The
`cargo release` rollover bumps it in lockstep with the version via a `[[pre-release-replacements]]`
entry in `release.toml`, so it never drifts from the released version. Do not hand-edit the badge
version; let the release command set it.

## What the pipeline guarantees

The release workflow performs these as gated steps and fails the release if any does not hold:

1. The tag version matches the version in `Cargo.toml`. A tag without a version bump fails here.
2. The `CHANGELOG.md` section for the version exists and is non-empty. A release with nothing
   logged fails here.
3. A Windows x64 MSI installer is built with `cargo-wix` and checksummed.
4. Linux x86_64 assets are built and checksummed: a `.deb` package (`cargo-deb`), an AppImage
   (assembled from `packaging/appimage/`), and a plain tarball.
5. A GitHub Release is created, with notes taken from the changelog section and every asset plus
   the combined `SHA256SUMS` attached.

You are responsible for the version number and that the changelog is current. Everything else is
the machine's job, and steps 1 and 2 catch the common omissions before any asset is built.

## Asset shape

Current decision (2026-07-10): Windows x64 MSI; Linux x86_64 `.deb`, AppImage, and tarball; a
combined `SHA256SUMS` file. No container images (this is a desktop application). macOS is out of
scope per the specification. Linux aarch64 is deferred until an end user needs it. PixelBeacon is
not a separate release asset: the addon ships embedded inside the application binary and is
installed from the application UI. Change this shape only with a dated decision recorded in
`CHANGELOG.md`.

## Supporting scripts

Two pinned scripts back the pipeline and are shared with local development:

- `scripts/changelog-section.sh <version>`: prints the changelog body for a version; used by the
  verify gate and to assemble release notes.
- `scripts/linux-build-deps.sh`: installs the system libraries required to build the GUI and
  input backends on Linux (X11/XCB, xkbcommon, Wayland, GL, evdev/udev headers). The dependency
  list lives only in this script so CI and developer machines cannot drift apart.

## Governance

`.github/workflows/**`, `rust-toolchain.toml`, `release.toml`, `scripts/**`, `packaging/**`, and
this file are pinned. An agent must not modify them without an explicit dated decision recorded in
`CHANGELOG.md`. If branch protection ever blocks the release commit, the rollover stays local
(it happens before the tag push), so the workflow itself never needs write access to `main`.
