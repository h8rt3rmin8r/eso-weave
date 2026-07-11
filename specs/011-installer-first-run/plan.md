# Implementation Plan: Installer and First-Run Experience

**Branch**: `011-installer-first-run` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/011-installer-first-run/spec.md`

## Summary

The released Windows MSI installs silently (no wizard, no completion screen),
surfaces only a nested Start Menu shortcut, and ships a console-subsystem GUI
that flashes a console window and hides any startup crash. This slice makes the
install visibly complete, easy to act on, and safe to fail: a WixUI wizard with a
license-gated flow and a de-elevated launch-on-finish option, a desktop shortcut,
a windowless release build, and a startup panic hook that shows a native dialog
and writes a log line. It is a Windows-only refinement of the packaging delivered
in slice 010; Linux packaging is untouched.

## Technical Context

**Language/Version**: Rust 1.96 (edition 2021), single crate `eso-weave`.

**Primary Dependencies**: `eframe` 0.35 (GUI); `windows-sys` 0.59 with
`Win32_UI_WindowsAndMessaging` already enabled (provides `MessageBoxW`); `tracing`
for logging. Installer: cargo-wix (WiX Toolset v3), which links `WixUIExtension`
and `WixUtilExtension` by default.

**Storage**: N/A for this slice. Logs continue to write to
`%APPDATA%\eso-weave\logs\YYYY-MM.log` via the existing logging facility.

**Testing**: `cargo test` with a `Notifier` trait seam and mock; a pure
`panic_message` function unit-tested. MSI behavior is validated by build plus
manual install (no Rust test harness for WiX).

**Target Platform**: Windows 10/11 x64 (this slice). Linux x64 unchanged.

**Project Type**: Desktop application (single Rust crate, per-OS backends).

**Performance Goals**: N/A; this slice changes packaging and startup only.

**Constraints**: Master spec section 13 (Start Menu entry, application icon,
upgrade-in-place, uninstall, never write to game or Documents directories) MUST
be preserved. Text hygiene (UTF-8 no BOM, LF, no en/em dashes) applies to every
new and edited file, including `License.rtf`. No change to worker-thread startup
ordering or the S002 input-backend contract.

**Scale/Scope**: One WiX source edited, one RTF added, one Rust bin file plus a
small `startup` module, README and (verification-only) releasing docs.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. This slice is a numbered
  `specs/011-installer-first-run/` slice built through the full spec-kit sequence;
  it traces to master spec section 13 (Packaging and Distribution) and the
  section 10 GUI window contract. The `/speckit.analyze` gate will run before
  implementation.
- **II. Safety-Critical Surfaces Are Sacrosanct**: PASS. No safety-critical
  surface is touched. The panic hook only adds logging plus a native dialog and is
  gated so it does not alter the input hook thread, worker hand-off, PixelBeacon
  uninstall, AddOns discovery, or fishing degradation. Worker-thread startup
  ordering is unchanged.
- **III. Test-First With Explicit Seams**: PASS. The failure-notification path is
  introduced behind a `Notifier` trait with a mock, and `panic_message` is a pure
  function; both get failing tests first. The WiX layer has no unit seam and is
  validated by build and manual install, documented in quickstart.
- **IV. CI Parity Before Every Commit**: PASS (enforced at verify step). The Rust
  changes run fmt, clippy, and test in the foreground to completion.
- **V. Bounded Scope: Outside The Game**: PASS. No game memory, network, or
  in-game behavior is involved.

Platform and text-hygiene constraints: single crate preserved (no workspace
promotion). `wix/main.wxs` (self-declared pinned) and the new
`packaging/windows/License.rtf` (under the pinned `packaging/**`) change only with
a dated `CHANGELOG.md` decision, recorded in this slice and surfaced at the
pre-push halt.

**Result**: PASS. No violations; Complexity Tracking not required.

## Project Structure

### Documentation (this feature)

```text
specs/011-installer-first-run/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/
│   └── installer-and-startup.md   # Installer and startup behavior contract
└── checklists/
    ├── requirements.md  # Spec quality checklist
    └── packaging.md     # Requirements-quality checklist
```

### Source Code (repository root)

```text
wix/
└── main.wxs             # Add WixUI_InstallDir, license, launch action, desktop shortcut

packaging/
└── windows/
    └── License.rtf      # New: Apache-2.0 license text for the wizard license page

src/
├── main.rs              # Add windows_subsystem attr; install startup panic hook
└── startup/
    └── mod.rs           # New: panic_message (pure) + Notifier trait + Win32 impl + mock

README.md                # Install section: shortcut locations and log directory
```

**Structure Decision**: Single-crate desktop application preserved. The only new
Rust module is `src/startup/` holding the testable failure-notification seam; all
other changes edit existing files. No workspace promotion, no new crate.

## Complexity Tracking

Not required. Constitution Check passed with no violations.
