# Feature Specification: Documentation Release Verification

**Feature Branch**: `codex/s065-docs-release-verification`

**Created**: 2026-09-08

**Status**: Ready for Review

**Input**: Issue #84: validate the bundled offline documentation in released Windows and Linux v0.15.0 packages and close parent epic #83 only when every evidence gate is satisfied.

## User Scenarios & Testing

### User Story 1 - Trust the Published Packages (Priority: P1)

As a release operator, I can prove that every advertised v0.15.0 package is present, byte-identical to its published checksum, and derived from the tagged release workflow.

**Independent Test**: Download every asset from the v0.15.0 GitHub Release, verify `SHA256SUMS`, and record names, sizes, digests, workflow run, tag, and release commit.

**Acceptance Scenarios**:

1. **Given** the published v0.15.0 release, **When** its assets are inventoried, **Then** one Windows MSI, one Linux deb, one Linux AppImage, one Linux tarball, and `SHA256SUMS` are present.
2. **Given** the downloaded assets, **When** each package is hashed, **Then** every digest matches the published checksum record exactly.

### User Story 2 - Use the Offline Guide on Windows (Priority: P1)

As a Windows operator, I can install the released MSI, open Help > Documentation, and use the complete local guide without depending on a documentation sidecar or remote content.

**Independent Test**: Install the v0.15.0 MSI on Windows 11, exercise the Documentation action and browser content, inspect the loopback endpoint, and confirm listener cleanup after exit.

**Acceptance Scenarios**:

1. **Given** the installed MSI, **When** Documentation is activated, **Then** the default browser opens the complete guide at an ephemeral `127.0.0.1` address under `/eso-weave/`.
2. **Given** the guide is open, **When** navigation, search, themes, images, fonts, code, deep links, and missing paths are exercised, **Then** local content works and safe failures are returned without external runtime assets.
3. **Given** Documentation has already been opened, **When** it is activated again and the app later exits, **Then** the address is reused and the listener is removed on exit.

### User Story 3 - Use the Offline Guide on Linux (Priority: P1)

As a Linux operator, I can repeat the released-package workflow with the deb and a portable artifact and receive the same documentation behavior.

**Independent Test**: Install the deb and run the AppImage or tarball under Ubuntu 24.04, exercise the same UI, endpoint, offline-content, reuse, and cleanup matrix, and record package-specific results.

**Acceptance Scenarios**:

1. **Given** the released deb, **When** it is installed and launched, **Then** Help > Documentation serves and opens the complete bundled guide.
2. **Given** the released AppImage or tarball, **When** it is launched without a sidecar documentation tree, **Then** the same guide and listener contract holds.
3. **Given** either Linux package exits, **When** the former address and process list are inspected, **Then** no documentation listener or application process remains.

### User Story 4 - Preserve Auditable Evidence (Priority: P1)

As a maintainer, I can review one durable receipt that distinguishes observed package evidence from repository tests and closes the documentation epic only when all requirements are demonstrated.

**Independent Test**: Review the receipt against issue #84, the v0.15.0 release, the S065 spec-kit artifacts, and the parent epic completion gate.

**Acceptance Scenarios**:

1. **Given** successful package verification, **When** the S065 pull request is reviewed, **Then** the receipt names exact environments, artifacts, checksums, install paths, browsers, commands, observations, and limitations.
2. **Given** any failed or unavailable criterion, **When** S065 reaches delivery review, **Then** #84 and #83 remain open and any product defect has a separately filed implementation issue.

## Edge Cases

- The release exists but an expected asset or checksum line is absent.
- A package digest differs from `SHA256SUMS`.
- The MSI or deb installs but launches a different version or path.
- Browser launch succeeds while a page silently fetches remote assets.
- Repeated Documentation actions allocate different ports.
- The listener binds beyond IPv4 loopback or survives application exit.
- A portable Linux package relies on files outside its distributed payload.
- UI automation can inspect behavior but cannot honestly substitute for a screen-reader observation.

## Functional Requirements

- **FR-001**: Verification MUST use the published v0.15.0 GitHub Release and tag commit `2ec90787e5b817897736d85a6c75c25baf365cba`.
- **FR-002**: The receipt MUST inventory and hash the MSI, deb, AppImage, tarball, and `SHA256SUMS` assets.
- **FR-003**: Every downloaded package digest MUST match the published checksum before execution.
- **FR-004**: Windows verification MUST use the released MSI and record the operating system, installation path, executable version, and browser.
- **FR-005**: Linux verification MUST use the released deb plus at least one released portable artifact and record the distribution, package paths, executable versions, desktop, and browser.
- **FR-006**: Both platforms MUST exercise Help > Documentation through the shipped UI using pointer or keyboard interaction.
- **FR-007**: Both platforms MUST demonstrate local search, nested navigation, deep links, images, fonts, code highlighting, theme behavior, responsive presentation, and the bundled 404 page.
- **FR-008**: Evidence MUST show that documentation runtime assets load only from loopback and that installed packages require no documentation sidecar or remote manual.
- **FR-009**: Evidence MUST show one reused ephemeral IPv4 loopback listener, GET and HEAD success, safe unknown-method and unknown-path responses, and listener cleanup at application exit.
- **FR-010**: Keyboard reachability, visible focus, accessible names, light and dark presentation, responsive layout, and non-color communication MUST be spot-checked on Windows and Linux; unavailable assistive-technology evidence MUST be reported rather than inferred.
- **FR-011**: The receipt MUST distinguish direct observation, automated inspection, and repository-contract evidence.
- **FR-012**: A product mismatch MUST create a linked implementation issue and MUST keep #84 and #83 open pending a fixed release and complete rerun.
- **FR-013**: S065 MUST make no runtime, packaging, release-workflow, or pinned-tooling change unless a separately scoped defect requires it.
- **FR-014**: S065 MUST archive completed plan 034 with v0.15.0 publication evidence, establish active plan 035, and update the plan indexes and migration ledger consistently.
- **FR-015**: The official pull request MUST close #84 and #83 only if every completion criterion is satisfied.
- **FR-016**: Spec-kit analysis, applicable documentation policy, text hygiene, receipt checks, and hosted CI MUST pass before merge readiness.

## Key Entities

- **Release artifact**: One immutable downloadable v0.15.0 package or checksum file with a published name, size, and digest.
- **Platform observation**: A bounded Windows or Linux package interaction tied to an exact environment and evidence method.
- **Documentation endpoint**: The application-owned ephemeral IPv4 loopback URL serving immutable embedded documentation.
- **Verification receipt**: The durable mapping from issue criteria to commands, observations, results, and limitations.

## Success Criteria

- **SC-001**: Five expected release assets are present and all four package digests match `SHA256SUMS`.
- **SC-002**: The Windows MSI, Linux deb, and one Linux portable package each open the complete bundled guide from the shipped UI.
- **SC-003**: Search, navigation, assets, themes, deep links, safe failures, reuse, loopback binding, and cleanup pass on both platforms.
- **SC-004**: No tested package requires a documentation sidecar or remote runtime asset.
- **SC-005**: Every #84 completion criterion has explicit evidence or prevents closure.
- **SC-006**: The final diff contains only spec-kit, verification, changelog, and project-lifecycle records unless a separate defect changes the plan.

## Assumptions

- Windows 11 and Ubuntu 24.04 WSLg are valid supported-host evidence for this verification run.
- The deb and AppImage provide the required installed and portable Linux coverage; the tarball remains checksum and payload evidence unless needed as fallback.
- Network isolation must be scoped to the tested process or environment and must not disconnect unrelated user work.
- Existing repository tests remain supporting contract evidence but do not replace released-package interaction.

## Clarification Decisions

- S065 is evidence-only and does not amend the constitution; governance amendment #104 is separate.
- The PR closes #84 and parent #83 only after complete evidence, rather than treating release publication as verification.
- WSLg is used for Linux package UI execution because it provides Ubuntu 24.04 with a graphical session on the current host.
- Any inaccessible criterion is recorded as a blocker, never converted into an assumed pass.
- On 2026-09-08, the release operator directly confirmed that the opened
  v0.15.0 documentation works, ended further desktop interaction, accepted the
  recorded platform variances, and directed closure. S065 therefore records
  that operator decision explicitly instead of claiming an unperformed manual
  screen-reader or elevated MSI-upgrade session.
