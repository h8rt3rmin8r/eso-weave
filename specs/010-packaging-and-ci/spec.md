# Feature Specification: Packaging and Distribution

**Feature Branch**: `010-packaging-and-ci`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "Packaging and Distribution per master specification section 13, providing the packaging artifacts that the already-pinned release pipeline (.github/workflows/release.yml) and docs/releasing.md reference but that do not yet exist: a WiX MSI configuration for cargo-wix plus assets/icon.ico; cargo-deb metadata, an AppImage AppDir, and evdev-permission documentation for Linux; the supporting scripts (changelog-section.sh, linux-build-deps.sh) and release.toml the pinned workflow and docs already call. Version is single-sourced from Cargo.toml. Builds run only in release CI on a tag; verification here is that configs are well-formed and consistent with release.yml, scripts run, the release binary compiles, and CI parity holds. Does not modify the pinned release.yml, releasing.md, or rust-toolchain.toml, and does not cut a release tag. Depends on feature 009."

## Clarifications

### Session 2026-07-11

Resolved under the Build-Phase Autopilot Protocol from the master specification
(section 13), the existing pinned release pipeline, and the constitution (no
options were escalated).

- Q: What is the source of truth this slice conforms to? -> A: The already-pinned
  `.github/workflows/release.yml` and `docs/releasing.md`. They reference files
  that do not yet exist (`scripts/changelog-section.sh`, `scripts/linux-build-deps.sh`,
  `release.toml`, the WiX config, the cargo-deb metadata, the AppImage AppDir,
  `assets/icon.ico`). This slice creates exactly those referenced artifacts so the
  pipeline is complete; it does not change the pinned workflow or docs.
- Q: Why can the packaging not be fully verified here? -> A: The MSI (cargo-wix),
  the `.deb` (cargo-deb), and the AppImage (appimagetool) are built by the release
  CI jobs on a tagged version, on Windows and Linux runners with toolsets not
  present in this environment. Verification here is limited to: the configs and
  scripts are well-formed and consistent with the pinned workflow, the scripts run
  and produce the expected output, the release binary compiles, `assets/icon.ico`
  is a valid icon, and the CI parity gate (fmt, clippy, test) holds. The actual
  installers are proven by the release pipeline on a tag.
- Q: Where does the WiX MSI configuration live and what does it guarantee? -> A: A
  cargo-wix source (`wix/main.wxs`, the tool's default location, so the pinned
  `cargo wix --no-build` finds it) that installs the single application binary with
  its icon (`assets/icon.ico`), a Start Menu shortcut, standard add/remove and
  upgrade-in-place behavior, and writes only under the chosen program-files
  location. It never writes to game or Documents directories; PixelBeacon
  management is a runtime action of the application.
- Q: How is the evdev permission requirement surfaced on Linux? -> A: A provided
  udev rule packaged with the `.deb` (and documented for the AppImage) that grants
  the input-device access the interception backend needs, plus package
  documentation stating the requirement (membership in the `input` group or the
  udev rule). The rule and its documentation are the deliverable; enforcing it at
  runtime is out of scope.
- Q: How is the version single-sourced? -> A: The crate version in `Cargo.toml` is
  the single source. `release.toml` drives cargo-release to bump it, and the MSI
  and the embedded addon version derive from it. The pinned verify job already
  checks that the tag matches the `Cargo.toml` version, so this slice adds no
  second version source.
- Q: What does `release.toml` do, and does this slice run it? -> A: It configures
  cargo-release to bump the `Cargo.toml` version, roll `CHANGELOG.md` (rename
  `## [Unreleased]` to `## [X.Y.Z] - DATE` and open a fresh `## [Unreleased]`),
  commit as `release: vX.Y.Z`, and tag and push. This slice creates and validates
  the config but does not run cargo-release or cut a tag; that is an explicit
  operator action.
- Q: What exactly does the `changelog-section.sh` argument match? -> A: The
  bracketed heading text of a `## [<arg>]` section. It prints the body between that
  heading and the next `## ` heading, excluding the heading line itself. So
  `changelog-section.sh Unreleased` prints the current `## [Unreleased]` body
  (testable now), and `changelog-section.sh 0.1.0` prints the `## [0.1.0] - DATE`
  body the release pipeline asks for after cargo-release has rolled the changelog.
  A heading with a trailing ` - DATE` still matches on the bracketed version; an
  absent section yields empty output.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - The release pipeline is complete (Priority: P1)

A maintainer who pushes a version tag gets a release built by the existing
pipeline, because every file the pinned `release.yml` and `releasing.md` reference
now exists and is consistent with them.

**Why this priority**: The pinned pipeline is inert without the artifacts it
calls; completing them is the whole point of this slice. It is the minimum viable
outcome.

**Independent Test**: Enumerate every path the pinned `release.yml` and
`releasing.md` reference (the two scripts, the WiX config, the cargo-deb metadata,
the AppImage AppDir, the icon, `release.toml`) and confirm each now exists and is
well-formed, and that the release binary builds.

**Acceptance Scenarios**:

1. **Given** the pinned `release.yml`, **When** its referenced paths are checked,
   **Then** `scripts/changelog-section.sh`, `scripts/linux-build-deps.sh`, the WiX
   source, the cargo-deb metadata, `packaging/appimage/AppDir`, and
   `assets/icon.ico` all exist.
2. **Given** the pinned `releasing.md`, **When** its referenced `release.toml` is
   checked, **Then** it exists and configures the version bump, changelog roll,
   commit, and tag.
3. **Given** the application, **When** the release binary is built, **Then** it
   compiles cleanly with the release profile.

### User Story 2 - Changelog section extraction works (Priority: P1)

The verify gate and the release notes read the changelog body for a version;
`changelog-section.sh` returns exactly that body so the gate can reject an empty
section and the notes carry the right content.

**Why this priority**: This script gates the release (an empty section fails the
build) and produces the release notes, so its correctness is release-blocking.

**Independent Test**: Run `changelog-section.sh` against the current `CHANGELOG.md`
for the `Unreleased` section (and a version heading) and confirm it prints the
section body and nothing from other sections.

**Acceptance Scenarios**:

1. **Given** a `CHANGELOG.md` with a populated section, **When**
   `changelog-section.sh <version>` runs, **Then** it prints that section's body.
2. **Given** a version with no section, **When** the script runs, **Then** it
   prints nothing (so the gate's empty-check fails the release).

### User Story 3 - Linux build dependencies are reproducible (Priority: P2)

CI and a developer machine install the same Linux system libraries from one
script, so the two never drift and a fresh Linux checkout can build the GUI and
input backends.

**Why this priority**: The build dependency list is shared between CI and
developers; a single source prevents drift, but it supports the build rather than
being an end-user artifact.

**Independent Test**: Confirm `linux-build-deps.sh` is a valid shell script that
installs the libraries the GUI (windowing, GL) and the input/pixel-bus backends
(X11/XCB, xkbcommon, Wayland, evdev/udev) require.

**Acceptance Scenarios**:

1. **Given** the script, **When** it is syntax-checked, **Then** it is a valid
   POSIX shell script.
2. **Given** the script, **When** its package list is reviewed, **Then** it covers
   the windowing, GL, X11/XCB, xkbcommon, Wayland, and evdev/udev dependencies.

### User Story 4 - Linux users learn the evdev requirement (Priority: P2)

A Linux user installing the package learns that the input interception needs
device access (input-group membership or the provided udev rule), so the app can
intercept keys.

**Why this priority**: Without the permission the interception silently does
nothing; documenting it and shipping the rule is what makes the Linux package
usable, layered on top of the package itself.

**Independent Test**: Confirm the package documentation states the evdev
requirement and that a udev rule is provided and referenced by the packaging.

**Acceptance Scenarios**:

1. **Given** the Linux package documentation, **When** it is read, **Then** it
   states the input-device permission requirement and how to satisfy it.
2. **Given** the `.deb` packaging, **When** its contents are described, **Then** a
   udev rule is included.

### Edge Cases

- What happens if the tag version does not match `Cargo.toml`? The pinned verify
  job fails the release; this slice adds no second version source that could
  disagree.
- What happens if the `Unreleased` section is empty at release time? The pinned
  gate, backed by `changelog-section.sh`, fails the release before any asset is
  built.
- What happens on a pure-Wayland Linux session? Interception uses the X11/XWayland
  path (an existing scope boundary); the build dependencies still install the
  Wayland libraries the windowing needs.
- What happens if the MSI is installed while the game is running? The MSI installs
  only the application and never touches game or Documents directories, so it is
  unaffected; PixelBeacon changes remain a runtime action.
- What happens to the version when a release is cut? `release.toml` bumps the one
  `Cargo.toml` version and rolls the changelog; there is no other version to keep
  in sync.

## Requirements *(mandatory)*

### Functional Requirements

Windows MSI:

- **FR-001**: A WiX MSI source MUST exist at the location the pinned
  `cargo wix --no-build` step resolves by default, installing the single
  application binary with its icon, a Start Menu shortcut, and standard
  add/remove and upgrade-in-place behavior.
- **FR-002**: The MSI configuration MUST write only under the installation
  location and MUST NOT write to game or Documents directories.
- **FR-003**: `assets/icon.ico` MUST exist as a valid Windows icon derived from
  the existing logo art and MUST be the application and installer icon.

Linux packages:

- **FR-004**: `Cargo.toml` MUST carry cargo-deb metadata sufficient for
  `cargo deb --no-build` to build a `.deb` of the application binary, including a
  desktop entry and icon and the packaged udev rule.
- **FR-005**: A `packaging/appimage/AppDir` MUST exist with the entry script,
  desktop entry, and icon that the pinned AppImage step assembles into an
  AppImage after the binary is copied in.
- **FR-006**: A udev rule granting the input-device access the interception
  backend needs MUST be provided and packaged, and the Linux package
  documentation MUST state the evdev permission requirement (input-group
  membership or the udev rule).

Release scripts and configuration:

- **FR-007**: `scripts/changelog-section.sh <heading>` MUST print the `CHANGELOG.md`
  body of the `## [<heading>]` section (the text between that heading line and the
  next `## ` heading, excluding the heading itself), matching on the bracketed
  version even when the heading carries a trailing ` - DATE`, and nothing from
  other sections; it MUST print empty output when the section is absent.
- **FR-008**: `scripts/linux-build-deps.sh` MUST be a valid shell script that
  installs the Linux system libraries required to build the GUI and the input and
  pixel-bus backends (windowing, GL, X11/XCB, xkbcommon, Wayland, evdev/udev),
  as the single shared source used by CI and developers.
- **FR-009**: `release.toml` MUST configure cargo-release to bump the `Cargo.toml`
  version, roll `CHANGELOG.md` (rename `## [Unreleased]` to `## [X.Y.Z] - DATE`
  and open a fresh `## [Unreleased]`), commit as `release: vX.Y.Z`, and tag and
  push.

Version and scope:

- **FR-010**: The application version MUST remain single-sourced from
  `Cargo.toml`; this slice MUST NOT introduce a second version source that could
  disagree with the tag-versus-`Cargo.toml` check the pinned pipeline performs.
- **FR-011**: This slice MUST NOT modify the pinned `release.yml`,
  `docs/releasing.md`, or `rust-toolchain.toml`, and MUST NOT cut a release tag or
  run cargo-release.
- **FR-012**: Creating the pinned artifacts (`scripts/**`, `packaging/**`,
  `release.toml`) MUST be recorded as dated decisions in `CHANGELOG.md`.

### Key Entities *(include if data involved)*

- **Release Pipeline Contract**: The pinned `release.yml` and `releasing.md` and
  the set of paths they reference, which this slice completes.
- **WiX MSI Source**: The installer definition (binary, icon, shortcut, upgrade
  behavior, install location).
- **cargo-deb Metadata**: The `.deb` definition in `Cargo.toml` (binary, desktop
  entry, icon, udev rule).
- **AppImage AppDir**: The directory the AppImage is assembled from (entry script,
  desktop entry, icon).
- **Release Scripts**: `changelog-section.sh`, `linux-build-deps.sh`, and
  `release.toml`.
- **Application Icon**: `assets/icon.ico`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every path referenced by the pinned `release.yml` and `releasing.md`
  exists after this slice, verifiable by enumerating them, in 100 percent of cases.
- **SC-002**: `changelog-section.sh` prints exactly the requested section's body
  for a populated section and empty output for an absent one, for representative
  inputs, in 100 percent of cases.
- **SC-003**: `linux-build-deps.sh` is a valid shell script covering every named
  dependency category, verifiable by syntax check and review.
- **SC-004**: `assets/icon.ico` is a valid Windows icon, verifiable by its format.
- **SC-005**: The release binary compiles cleanly with the release profile, and the
  CI parity gate (fmt, clippy, test) passes.
- **SC-006**: The pinned `release.yml`, `releasing.md`, and `rust-toolchain.toml`
  are unchanged, and no release tag is cut, verifiable by the diff.

## Assumptions

- Scope is master specification section 13 plus completing the pinned release
  pipeline. The actual MSI, `.deb`, and AppImage builds are performed by the
  release CI on a tag and are not run in this environment; verification here is
  static consistency, script execution, a valid icon, a clean release build, and
  CI parity.
- The pinned `release.yml`, `docs/releasing.md`, and `rust-toolchain.toml` are
  correct and are the contract; this slice conforms to them and does not modify
  them.
- Feature 009 provides the GUI binary that is packaged; the Linux build
  dependencies include what the eframe/glow GUI and the X11 and evdev backends
  need.
- Creating the pinned `scripts/**`, `packaging/**`, and `release.toml` for the
  first time is permitted under the Build-Phase Autopilot Protocol when recorded as
  dated `CHANGELOG.md` decisions.
- Cutting a release (running cargo-release and pushing a `vX.Y.Z` tag) remains an
  explicit operator action outside this slice.
