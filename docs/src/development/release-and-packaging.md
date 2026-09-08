# Release and Packaging

The Release Pipeline is the release workflow that verifies a tag build and then
publishes the approved release assets.

This page explains stable artifact and pipeline guarantees for contributors. It
does not authorize a release and does not duplicate the command-by-command
maintainer ritual.

## Release state flow

The release moves through this ordered text flow:

`human-authorized preparation -> vX.Y.Z tag -> verify -> Windows and Linux builds -> checksums -> GitHub Release`

Failure at a required stage prevents publication. Build jobs do not write back
to `main`.

## Version and content gates

The tag is the pipeline trigger and `Cargo.toml` is the application and artifact
version authority. Before any package build, verification requires:

1. A tag shaped as `vX.Y.Z` whose value matches the Cargo package version.
2. A non-empty matching section in `CHANGELOG.md`.
3. Exactly one non-empty Highlights subsection for release notes.
4. One through six top-level Highlights bullets totaling no more than 120 words.
5. Exact mdBook 0.5.4 and mdbook-linkcheck2 0.13.0 tools generate and validate
   the canonical site during each platform production build.

The release body contains that concise Highlights excerpt and a tag-specific
link to the complete changelog. Detailed engineering records remain in the
changelog rather than being copied into the release page.

## Artifact matrix

| Platform | Artifact | Producer | Published integrity evidence |
| --- | --- | --- | --- |
| Windows x64 | Versioned `.msi` | Release build plus `cargo-wix` | SHA-256 entry in `SHA256SUMS` |
| Linux x86_64 | Versioned `.deb` | Release build plus `cargo-deb` | SHA-256 entry in `SHA256SUMS` |
| Linux x86_64 | Versioned AppImage | Release binary assembled from `packaging/appimage/AppDir` | SHA-256 entry in `SHA256SUMS` |
| Linux x86_64 | Versioned `.tar.gz` | Release binary, README, and license archive | SHA-256 entry in `SHA256SUMS` |

PixelBeacon is embedded in the application binary and installed from the
application interface. It is not a separate release asset. Container images,
macOS packages, and Linux aarch64 packages are outside the current artifact
contract.

The searchable documentation site is also embedded directly in every executable.
Generated HTML remains ignored build output. A release-profile Cargo build fails
if the exact documentation tools are absent, mismatched, or cannot produce the
site, so package jobs cannot silently ship the development fixture.

## Workflow ownership and permissions

The verification job and both platform build jobs use read-only repository
permissions. Only the final release job receives `contents: write`, after it
depends on verification and both builds. That job downloads the build artifacts,
combines their per-file hashes into `SHA256SUMS`, generates release notes, and
creates the GitHub Release.

The workflow, release configuration, scripts, packaging metadata, and maintainer
procedure are pinned governance artifacts. Changes require the repository's
dated decision process. This published explanation does not replace that rule.

## Failure boundaries

| Failure | Pipeline result |
| --- | --- |
| Tag and Cargo version differ | Verification fails before package builds |
| Changelog section is absent or empty | Verification fails before package builds |
| Highlights are missing, duplicated, malformed, too numerous, or too long | Verification fails before package builds |
| Documentation tools are absent, mismatched, or generation fails | The platform binary and packages are not produced |
| Either platform build fails | Final release job cannot run |
| Required artifact or hash is absent | Assembly or publication fails |
| Release creation lacks permission | Packages remain workflow artifacts; no GitHub Release is created |

The shell contract in `scripts/release-notes.test.sh` exercises valid extraction,
CRLF input, section boundaries, Unreleased preview, malformed lists, duplicate or
empty Highlights, six-item and 120-word boundaries, and invalid version or
repository arguments.

## Maintainer authority

The sole authoritative release procedure remains
[`docs/project/releasing.md`](https://github.com/h8rt3rmin8r/eso-weave/blob/main/docs/project/releasing.md).
It contains the human-authorized preparation and release command. Do not infer
credentials, bypass checks, or cut a release from this conceptual page.

User package selection, checksum verification, installation, update, and removal
belong to [Installation](../getting-started/installation.md).
