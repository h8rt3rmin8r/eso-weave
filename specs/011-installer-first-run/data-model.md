# Phase 1 Data Model: Installer and First-Run Experience

This slice adds no persistent application data and no config schema change. The
"entities" here are installer and startup constructs and the one code seam.

## Installer constructs (wix/main.wxs)

### Wizard UI

- **Source**: `WixUI_InstallDir` dialog set via `UIRef`.
- **Bound property**: `WIXUI_INSTALLDIR = APPLICATIONFOLDER`.
- **License variable**: `WixUILicenseRtf = packaging\windows\License.rtf`.
- **States**: Welcome, License (Next disabled until accepted), InstallDir, Verify,
  Progress, Exit.

### Launch-on-finish action

- **Checkbox**: `WIXUI_EXITDIALOGOPTIONALCHECKBOX` (default `1`), text from
  `WIXUI_EXITDIALOGOPTIONALCHECKBOXTEXT = "Launch ESO Weave"`.
- **Target**: `WixShellExecTarget = [#EsoWeaveExe]`.
- **Custom action**: `LaunchApplication` (`WixCA` / `WixShellExec`,
  `Impersonate="yes"`, `Return="asyncNoWait"`).
- **Trigger condition**: `WIXUI_EXITDIALOGOPTIONALCHECKBOX = 1 and NOT Installed`,
  published on ExitDialog `Finish`. Runs de-elevated in the UI sequence; absent in
  silent installs.

### Desktop shortcut component

- **Directory**: `DesktopFolder` (standard WiX id, name `Desktop`).
- **Component**: `ApplicationDesktopShortcut` (`Win64="yes"`).
- **Shortcut**: `Target=[#EsoWeaveExe]`, `WorkingDirectory=APPLICATIONFOLDER`,
  `Icon=ProductIcon`.
- **Keypath**: HKCU `Software\ESO Weave\ESO Weave` value `desktop_shortcut`
  (integer `1`).
- **Feature link**: `ComponentRef` added to the `Application` feature.

### Preserved constructs (unchanged)

Start Menu shortcut component, `MajorUpgrade`, `ARPPRODUCTICON`, `perMachine`
scope, the single `ApplicationBinary` component and its `KeyPath` exe.

## Startup seam (src/startup/mod.rs)

### `panic_message` (pure function)

- **Signature**: `fn panic_message(payload: &str, location: Option<String>) -> String`.
- **Behavior**: Produces a stable, single-block message combining the panic
  payload and optional `file:line:col` location. No I/O.
- **Validation rules**: Non-empty output even when payload is empty; location
  omitted cleanly when `None`.

### `Notifier` trait

- **Signature**: `trait Notifier { fn notify(&self, title: &str, body: &str); }`.
- **Real impl (Windows)**: Calls `MessageBoxW` with an error icon; wide-string
  conversion of title and body.
- **Mock impl (tests)**: Records the last `(title, body)` for assertions; performs
  no UI.

### Panic hook installer

- **Inputs**: A `Notifier` (boxed) and a shared `AtomicBool` "gui started" flag.
- **Behavior**: On panic, always logs the `panic_message` at `tracing::error`
  (target `eso_weave`); invokes `notify` only when the flag is `false`.
- **Lifecycle**: Installed in `main()` before any worker thread is spawned; the
  flag is set `true` immediately before `eframe::run_native`.

## State transition: first launch

```text
process start
  -> install panic hook (gui_started = false)
  -> load config, init logging, build subsystems, spawn workers
  -> gui_started = true
  -> eframe::run_native (window shown)
```

A panic anywhere before `gui_started = true` produces both a dialog and a log
line; a panic after produces a log line only.
