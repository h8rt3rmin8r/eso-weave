# Pipeline-Contract Requirements Checklist: Packaging and Distribution

**Purpose**: Validate that the requirements completely and consistently cover the
artifacts the pinned release pipeline references, and the constraints on not
modifying pinned files, before planning and implementation.
**Created**: 2026-07-11
**Feature**: [spec.md](../spec.md)

## Referenced-Artifact Completeness

- [x] CHK001 Is every path the pinned `release.yml` references required to exist (the two scripts, the WiX source at the cargo-wix default location, cargo-deb metadata, `packaging/appimage/AppDir`, `assets/icon.ico`)? [Completeness, Spec FR-001 to FR-008]
- [x] CHK002 Is `release.toml` (referenced by `releasing.md`) required to exist and configure the release rollover? [Completeness, Spec FR-009]
- [x] CHK003 Is the WiX source location specified so the pinned `cargo wix --no-build` (no path argument) resolves it? [Clarity, Spec FR-001, Clarifications]

## changelog-section.sh Contract

- [x] CHK004 Is the argument-to-section matching defined (matches `## [<arg>]`, tolerating a trailing ` - DATE`)? [Clarity, Spec FR-007, Clarifications]
- [x] CHK005 Is it specified that only the section body is printed (excluding the heading and other sections)? [Clarity, Spec FR-007]
- [x] CHK006 Is the empty-output-when-absent behavior specified, so the pinned verify gate can reject an empty section? [Completeness, Spec FR-007, US2]

## linux-build-deps.sh Coverage

- [x] CHK007 Does the dependency coverage requirement enumerate the GUI windowing/GL and the X11/XCB, xkbcommon, Wayland, and evdev/udev categories? [Completeness, Spec FR-008]
- [x] CHK008 Is the single-shared-source rationale (CI and developers cannot drift) stated? [Clarity, Spec FR-008, US3]

## Windows MSI and Icon

- [x] CHK009 Are the MSI guarantees specified (single binary, icon, Start Menu shortcut, upgrade-in-place, standard add/remove)? [Completeness, Spec FR-001]
- [x] CHK010 Is the never-write-to-game-or-Documents guarantee stated? [Consistency, Spec FR-002, Constitution V]
- [x] CHK011 Is `assets/icon.ico` required to be a valid Windows icon derived from the existing art and used as the app and installer icon? [Completeness, Spec FR-003, SC-004]

## Linux Packages

- [x] CHK012 Is the cargo-deb metadata required to be sufficient for `cargo deb --no-build` including a desktop entry, icon, and the udev rule? [Completeness, Spec FR-004]
- [x] CHK013 Is the AppImage AppDir content (entry script, desktop entry, icon) specified? [Completeness, Spec FR-005]
- [x] CHK014 Is a udev rule required and packaged, and is the evdev-permission documentation (input group or the rule) required? [Completeness, Spec FR-006, US4]

## Version and Governance

- [x] CHK015 Is the single-sourced-version-from-Cargo.toml rule stated, with no second version source that could disagree with the pinned tag check? [Consistency, Spec FR-010]
- [x] CHK016 Is `release.toml` required to drive the bump, changelog roll, commit, and tag? [Completeness, Spec FR-009]
- [x] CHK017 Is the do-not-modify-pinned-files constraint (`release.yml`, `releasing.md`, `rust-toolchain.toml`) and the do-not-cut-a-tag constraint stated? [Consistency, Spec FR-011, SC-006]
- [x] CHK018 Is the requirement to record creation of pinned artifacts as dated `CHANGELOG.md` decisions stated? [Completeness, Spec FR-012, Constitution]

## Verification Boundary

- [x] CHK019 Is the verification boundary explicit (installers built only by release CI on a tag; here limited to well-formedness, script execution, a valid icon, a clean release build, and CI parity)? [Clarity, Spec Assumptions, SC-005]
- [x] CHK020 Does every functional requirement have an acceptance scenario or measurable success criterion? [Traceability, Spec FR/US/SC]

## Notes

- This checklist tests requirement quality, not the produced installers (which the
  release CI proves on a tag).
- The named tools are the pinned pipeline's fixed contract, not this slice's choice;
  the requirements stay outcome-focused on the referenced files existing and being
  consistent with the pinned `release.yml`.
