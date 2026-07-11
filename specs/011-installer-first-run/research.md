# Phase 0 Research: Installer and First-Run Experience

All decisions below were resolved from the master specification (section 13 and
the section 10 GUI window contract), the constitution, the current
`wix/main.wxs`, `Cargo.toml`, and the pinned `.github/workflows/release.yml`. No
NEEDS CLARIFICATION items remain.

## D1: Installer wizard dialog set

- **Decision**: Add `WixUI_InstallDir` via `<UIRef Id="WixUI_InstallDir" />` and
  `<Property Id="WIXUI_INSTALLDIR" Value="APPLICATIONFOLDER" />`.
- **Rationale**: `WixUI_InstallDir` provides exactly the welcome, license,
  install-location, verify-ready, progress, and exit dialogs the spec requires
  (FR-001), and its install-location page binds to the existing
  `APPLICATIONFOLDER` directory id so no directory restructuring is needed. It is
  the standard WiX dialog set for a single-feature per-machine install.
- **Alternatives considered**: `WixUI_Minimal` (no install-location page, weaker
  for FR-001); `WixUI_FeatureTree` (feature selection is overkill for a
  single-feature product); a fully custom UI (unjustified maintenance cost).

## D2: License page source and acceptance gating

- **Decision**: Add `packaging/windows/License.rtf` containing the repository's
  Apache-2.0 license text as RTF, referenced with
  `<WixVariable Id="WixUILicenseRtf" Value="packaging\windows\License.rtf" />`.
- **Rationale**: The `WixUI_InstallDir` license dialog requires an RTF and
  natively disables Next until the user accepts (FR-003). Sourcing the actual
  Apache-2.0 text keeps the installer honest and matches the `license` field in
  `Cargo.toml`. The RTF is authored ASCII/UTF-8 without BOM, LF line endings, with
  no en/em dashes (the Apache-2.0 text is plain ASCII), satisfying text hygiene.
- **Alternatives considered**: A short "licensed under Apache-2.0" summary RTF
  (weaker for an acceptance gate); pointing at the repo `LICENSE` directly (WiX
  requires RTF, not plain text, for this dialog).

## D3: Launch-on-finish under an elevated per-machine install

- **Decision**: Use the WixUI ExitDialog optional checkbox plus a `WixShellExec`
  custom action published on the ExitDialog `Finish` control:
  - `WIXUI_EXITDIALOGOPTIONALCHECKBOXTEXT = "Launch ESO Weave"`
  - `WIXUI_EXITDIALOGOPTIONALCHECKBOX = 1` (default checked)
  - `WixShellExecTarget = [#EsoWeaveExe]`
  - `<CustomAction Id="LaunchApplication" BinaryKey="WixCA" DllEntry="WixShellExec" Impersonate="yes" Return="asyncNoWait" />`
  - `<Publish Dialog="ExitDialog" Control="Finish" Event="DoAction" Value="LaunchApplication">WIXUI_EXITDIALOGOPTIONALCHECKBOX = 1 and NOT Installed</Publish>`
- **Rationale**: This resolves the one genuine design risk (an elevated install
  launching the GUI elevated). The ExitDialog runs in the InstallUISequence in the
  context of the invoking (non-elevated) user, so `WixShellExec` with
  `Impersonate="yes"` starts the application DE-ELEVATED, which is correct for a
  companion app whose input hook needs no elevation. A silent or unattended
  install has no UI sequence, so the ExitDialog never fires and the application is
  never auto-launched (FR-005). The `and NOT Installed` guard restricts the launch
  to fresh installs, not repair/uninstall.
- **Alternatives considered**: A deferred custom action in the execute sequence
  (runs as SYSTEM/elevated, launching the GUI elevated); a "runas"-style
  re-launch trick (fragile, unnecessary once ExitDialog impersonation is used);
  auto-launch with no checkbox (violates the user-controlled requirement FR-004
  and the silent-install rule FR-005).

## D4: WiX extensions and the pinned release workflow

- **Decision**: Rely on cargo-wix linking `WixUIExtension` and `WixUtilExtension`
  by default; do not modify `.github/workflows/release.yml`. Verify at MSI build
  time (`cargo wix --no-build`) that the UI and `WixShellExec` action link
  cleanly.
- **Rationale**: cargo-wix passes both extensions to `light` by default, so the
  `UIRef` (WixUIExtension) and the `WixCA`/`WixShellExec` action (WixUtilExtension)
  resolve without a workflow change, keeping a pinned artifact untouched.
- **Fallback**: If the release build fails to link the extensions, the minimal
  pinned-workflow change (adding the extension flags) would be recorded as a dated
  CHANGELOG decision and surfaced at the pre-push halt. Treated as unlikely.

## D5: Desktop shortcut

- **Decision**: Add a `DesktopFolder` directory and an `ApplicationDesktopShortcut`
  component with a `<Shortcut>` to `[#EsoWeaveExe]` and an HKCU `RegistryValue`
  keypath; add its `ComponentRef` to the `Application` feature.
- **Rationale**: Mirrors the existing Start Menu shortcut component pattern
  (HKCU keypath satisfies the per-user shortcut component rule) and writes only to
  the user's Desktop, not to game or Documents directories (FR-006, FR-008). MSI
  does not fail when the Desktop exists, so FR-009 holds inherently.
- **Alternatives considered**: An advertised shortcut (unnecessary; the app is a
  single exe); a per-machine common Desktop shortcut (HKLM keypath complicates the
  component rule with no benefit for a single-user companion app).

## D6: Hide the console window

- **Decision**: Add `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`
  at the crate-bin root of `src/main.rs`, immediately after the module doc block.
- **Rationale**: Release builds (what ships in the MSI) become windowless, removing
  the console flash (FR-010), while debug builds keep the console so developers
  still see stdout/stderr and tests are unaffected. Gating on `not(debug_assertions)`
  is the idiomatic way to avoid harming the dev loop.
- **Alternatives considered**: Unconditional `windows_subsystem = "windows"` (kills
  the dev console and complicates debugging); a separate release-only bin target
  (more build surface than needed).

## D7: Startup failure surfacing (panic hook)

- **Decision**: Introduce `src/startup/mod.rs` with:
  - `panic_message(payload: &str, location: Option<String>) -> String`: pure,
    unit-tested formatter.
  - `trait Notifier { fn notify(&self, title: &str, body: &str); }` with a real
    Windows implementation calling `MessageBoxW` (from the already-enabled
    `windows-sys` `Win32_UI_WindowsAndMessaging` feature) and a `MockNotifier` for
    tests.
  - A hook installer that sets `std::panic::set_hook` to always log the formatted
    message at `tracing::error`, and to invoke the notifier ONLY while an
    `AtomicBool` "gui started" flag is false. `main()` sets that flag true
    immediately before `eframe::run_native`.
- **Rationale**: Once the console is hidden (D6), an unhandled startup panic would
  otherwise be invisible, reproducing the original complaint at the app layer. The
  hook guarantees a visible dialog plus a log line for startup failures (FR-011,
  FR-012). Gating the dialog on the pre-GUI window avoids popping dialogs for
  mid-session worker-thread panics (those are still logged). The seam keeps the
  logic unit-testable without spawning the real dialog. This adds no new dependency
  and does not change worker-thread startup ordering (the hook is installed before
  threads spawn, and only reads a flag).
- **Alternatives considered**: `catch_unwind` around startup (does not cover
  panics on spawned threads and is clumsier than a global hook); always showing the
  dialog on any panic (noisy for long-running worker panics); a third-party dialog
  crate such as `rfd` (unnecessary dependency when `MessageBoxW` is already
  available).

## D8: Documentation

- **Decision**: Update the README install section to state the Start Menu and
  desktop shortcut locations and the Windows log directory
  (`%APPDATA%\eso-weave\logs\YYYY-MM.log`). Confirm `docs/releasing.md` still
  describes a single MSI artifact and leave it unchanged (verification only).
- **Rationale**: FR-013 and SC-006 require users to locate a working shortcut and
  the logs. The artifact shape is unchanged (still one MSI), so the pinned
  releasing doc needs no edit.
