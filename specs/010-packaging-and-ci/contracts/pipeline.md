# Contract: Pinned Release Pipeline References

This maps each reference in the pinned `.github/workflows/release.yml` and
`docs/releasing.md` to the file this slice creates that satisfies it. Every row
must resolve for the pipeline to run.

## release.yml references

| Reference in release.yml | Satisfied by |
| --- | --- |
| `scripts/changelog-section.sh "$version"` (verify gate) | `scripts/changelog-section.sh` |
| `scripts/changelog-section.sh "$ver" > RELEASE_NOTES.md` (release job) | `scripts/changelog-section.sh` |
| `sudo scripts/linux-build-deps.sh` (build-linux) | `scripts/linux-build-deps.sh` |
| `cargo build --release --locked --bin eso-weave` | the existing crate (compiles) |
| `cargo wix --no-build --nocapture` | `wix/main.wxs` (+ `[package.metadata.wix]` icon) |
| `cargo deb --no-build --locked` | `Cargo.toml` `[package.metadata.deb]` |
| AppImage: `packaging/appimage/AppDir` + install binary to `usr/bin/eso-weave` | `packaging/appimage/AppDir/{AppRun, eso-weave.desktop, eso-weave.png}` |
| `cp README.md LICENSE "$dist/"` | existing `README.md`, `LICENSE` |
| tag-version-matches-Cargo.toml, changelog-non-empty gates | version in `Cargo.toml`; `changelog-section.sh` |

## releasing.md references

| Reference in releasing.md | Satisfied by |
| --- | --- |
| `cargo release X.Y.Z --execute` (configured in `release.toml`) | `release.toml` |
| bump `Cargo.toml`, roll `CHANGELOG.md`, commit `release: vX.Y.Z`, tag, push | `release.toml` `pre-release-replacements` + tag/commit settings |
| `scripts/changelog-section.sh <version>` | `scripts/changelog-section.sh` |
| `scripts/linux-build-deps.sh` dependency source | `scripts/linux-build-deps.sh` |
| AppImage assembled from `packaging/appimage/` | `packaging/appimage/AppDir` |

## Verifiable-here behaviors

| Behavior | How verified in this environment |
| --- | --- |
| `changelog-section.sh Unreleased` prints the Unreleased body | run it against `CHANGELOG.md` |
| `changelog-section.sh <absent>` prints nothing | run it with a nonexistent heading |
| `release.toml` parses and its replacements resolve | `cargo release <next> --dry-run` |
| Scripts are valid shell | `bash -n` |
| `assets/icon.ico` is a valid icon | ICO magic bytes `00 00 01 00` |
| `wix/main.wxs` is well-formed XML | XML parse |
| `.desktop` entries are well-formed | structural review / `desktop-file-validate` if present |
| release binary compiles | `cargo build --release --bin eso-weave` |
| CI parity | `cargo fmt`, `clippy`, `test` |

## Not verifiable here (proven by release CI on a tag)

- The MSI (cargo-wix + WiX toolset, Windows runner).
- The `.deb` (cargo-deb, Linux runner).
- The AppImage (appimagetool, Linux runner).
- The end-to-end GitHub Release creation.
