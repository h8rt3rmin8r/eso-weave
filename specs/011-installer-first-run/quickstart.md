# Quickstart: Validating the Installer and First-Run Experience

Prerequisites: a Windows 10/11 x64 machine, the WiX Toolset v3 available to
cargo-wix, and a checkout of this branch. Commands run from the repo root.

## 1. CI parity (all platforms)

```pwsh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected: all green. The `startup` module tests exercise `panic_message` and the
`Notifier` seam (via `MockNotifier`) without opening any dialog. Ref:
[contracts](contracts/installer-and-startup.md) C-APP-4, C-APP-5.

## 2. Build the MSI locally (Windows)

```pwsh
cargo build --release --locked --bin eso-weave
cargo install cargo-wix --locked   # if not already installed
cargo wix --no-build --nocapture
```

Expected: `cargo wix` links `WixUIExtension` and `WixUtilExtension` with no
error, and writes an `.msi` under `target\wix\`. A link error here would signal
the D4 fallback (pinned-workflow change); treat as unlikely. Ref: research D1, D4.

## 3. Interactive install walkthrough

Run the produced MSI and confirm, in order (C-INST-1..C-INST-9):

- Welcome, License, Install-location, Ready, Progress, and Exit pages all appear.
- The license page keeps Next disabled until you accept.
- The exit page confirms a successful install and shows a checked "Launch ESO
  Weave" checkbox.
- Finishing with the box checked opens the application window with no console
  window (C-APP-1); the running process is NOT elevated (check Task Manager
  "Elevated" column) (C-INST-5).
- A desktop shortcut and a Start Menu entry both start the application
  (C-INST-7).

## 4. Silent install (no launch)

```pwsh
msiexec /i <path-to>.msi /qn
```

Expected: install completes and the application does NOT launch (C-INST-6).
Uninstall via Apps and Features or `msiexec /x` and confirm shortcuts are removed
(C-INST-8).

## 5. Startup-failure surfacing

Force an early startup panic (temporary local edit, reverted after) before the
`gui_started` flag is set, build release, and launch:

Expected: a native error dialog appears identifying the failure (C-APP-2), and
`%APPDATA%\eso-weave\logs\YYYY-MM.log` contains a matching error line (C-APP-3).

## 6. Documentation check

Read the README install section and confirm it states the Start Menu and desktop
shortcut locations and the log directory (C-DOC-1). Confirm `docs/releasing.md`
still describes one MSI (C-DOC-2).
