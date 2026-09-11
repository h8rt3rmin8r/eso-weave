# Releasing ESO Weave

This is the one authoritative procedure for cutting a release. It exists so the steps are never
re-derived or improvised per session. Cutting a release is a single action; the pipeline does the
rest and fails loudly if anything required is missing.

## Cut a release

Prerequisite: every change since the last release is logged under `## [Unreleased]` in
`CHANGELOG.md`. Keep the complete Added, Changed, Fixed, and Decisions history there.

The same section must begin with `### Highlights`: one through six top-level bullets totaling no
more than 120 words. Summarize the outcomes users need to know and leave implementation detail in
the full changelog subsections. GitHub release pages contain only this excerpt followed by a link
to the complete changelog at the release tag, keeping downloads immediately discoverable.

Preview the candidate notes before releasing:

```bash
CHANGELOG_HEADING=Unreleased scripts/release-notes.sh X.Y.Z h8rt3rmin8r/eso-weave
```

The command fails when Highlights is missing, empty, malformed, or over budget. Replace `X.Y.Z`
with the version being prepared.

Run one command:

```bash
cargo release X.Y.Z --execute
```

That command (configured in `release.toml`):

1. Bumps the crate version in `Cargo.toml`. This is the single source of the application
   and package version. Addon contract versions remain independent manifest authorities.
2. Rewrites `CHANGELOG.md`, renaming `## [Unreleased]` to `## [X.Y.Z] - DATE` and opening a fresh
   empty `## [Unreleased]`.
3. Rewrites the root README badge and bundled documentation snapshot version and date from the
   same release version and date.
4. Commits the change as `release: vX.Y.Z`.
5. Tags `vX.Y.Z` and pushes the commit and tag.

Pushing the tag triggers `.github/workflows/release.yml`.

## Post-publication verification

The candidate and tag workflow must pass every available CI, safety,
release-note, packaging, repository, and authorization gate before publication.
Installed UI, package, field, platform, or production checks that require a
downloadable artifact occur after the GitHub Release exists. They do not block
creating the artifact once the pre-publication gates pass.

Track each independently closeable artifact check in a separate issue held in
Release verification. Publication makes the exact package available for testing
but does not prove that it installs or behaves correctly. Record the tag,
checksum, environment, and observations. If the package fails, file linked
implementation work and keep verification open until a fixed release passes.

Note: the README version badge is a static shields.io badge (`version-X.Y.Z-2ea44f`). The
`cargo release` rollover bumps it in lockstep with the version via a `[[pre-release-replacements]]`
entry in `release.toml`, so it never drifts from the released version. Do not hand-edit the badge
version; let the release command set it.

The bundled documentation landing page is governed the same way. During the dry run, inspect
both its `Applies to` version and `Released` date. Do not hand-edit those fields before release.

## What the pipeline guarantees

The release workflow performs these as gated steps and fails the release if any does not hold:

1. The tag version matches the version in `Cargo.toml`. A tag without a version bump fails here.
2. The complete `CHANGELOG.md` section for the version exists and is non-empty. A release with
   nothing logged fails here.
3. The version has a valid Highlights excerpt within the six-item and 120-word budget.
4. A Windows x64 MSI installer is built with `cargo-wix` and checksummed.
5. Linux x86_64 assets are built and checksummed: a `.deb` package (`cargo-deb`), an AppImage
   (assembled from `packaging/appimage/`), and a plain tarball. Before upload,
   the `.deb` control record must contain non-empty Package, Version,
   Architecture, Maintainer, and Description fields.
6. A GitHub Release is created, with notes taken only from Highlights, a tag-specific link to the
   complete changelog, and every asset plus the combined `SHA256SUMS` attached.
7. Both platform jobs install exact `mdbook 0.5.4` and `mdbook-linkcheck2 0.13.0`; each
   release-profile Cargo build regenerates and embeds the complete site or fails before packaging.

You are responsible for the version number and that the changelog is current. Everything else is
the machine's job, and steps 1 through 3 catch the common omissions before any asset is built.

## Asset shape

Current decision (2026-07-10): Windows x64 MSI; Linux x86_64 `.deb`, AppImage, and tarball; a
combined `SHA256SUMS` file. No container images (this is a desktop application). macOS is out of
scope per the specification. Linux aarch64 is deferred until an end user needs it. PixelBeacon
and the catalog collector are not separate release assets: both ship embedded inside the
application binary and are installed from the application UI. The encounter-capture addon remains
a repository-source developer tool for manual installation. Change this shape only with a dated
decision recorded in `CHANGELOG.md`.

## Supporting scripts

Three pinned scripts back the pipeline and are shared with local development:

- `scripts/changelog-section.sh <version>`: prints the changelog body for a version; used by the
  verify gate to protect the complete release record.
- `scripts/release-notes.sh <version> [owner/repository] [changelog-file]`: validates Highlights
  and prints the compact GitHub release body; exercised by `scripts/release-notes.test.sh`.
- `scripts/linux-build-deps.sh`: installs the system libraries required to build the GUI and
  input backends on Linux (X11/XCB, xkbcommon, Wayland, GL, evdev/udev headers). The dependency
  list lives only in this script so CI and developer machines cannot drift apart.
- `scripts/validate-debian-package.sh <package.deb>`: rejects a built Debian
  package with a missing required control field before upload;
  `scripts/validate-debian-package.test.sh` exercises its success and failure
  contract in pull-request CI.

## Governance

`.github/workflows/**`, `rust-toolchain.toml`, `release.toml`, `scripts/**`, `packaging/**`, and
this file are pinned. An agent must not modify them without an explicit dated decision recorded in
`CHANGELOG.md`. If branch protection ever blocks the release commit, the rollover stays local
(it happens before the tag push), so the workflow itself never needs write access to `main`.
