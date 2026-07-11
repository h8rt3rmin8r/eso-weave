---

description: "Task list for the installer and first-run experience feature"
---

# Tasks: Installer and First-Run Experience

**Input**: Design documents from `/specs/011-installer-first-run/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md,
contracts/installer-and-startup.md, quickstart.md

**Tests**: Included for the Rust startup seam only (constitution principle III,
test-first with explicit seams). The WiX installer has no unit harness and is
validated by build plus manual install per quickstart.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1..US5)

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Wire the new module so subsequent Rust tasks compile.

- [ ] T001 Create `src/startup/mod.rs` with empty stubs (`panic_message`, `Notifier` trait, `install_hook`) and declare `mod startup;` in `src/main.rs` so the crate compiles.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: None beyond setup. The user stories are independent and edit mostly
separate files; the only shared file is `wix/main.wxs` (US1, US2) and `src/main.rs`
(US3, US4), sequenced within their phases.

*(No foundational-only tasks.)*

---

## Phase 3: User Story 1 - Install visibly completes (Priority: P1)

**Goal**: The MSI presents a guided wizard ending in a clear completion
confirmation, with a license-acceptance gate.

**Independent Test**: Build the MSI and run it interactively; confirm welcome,
license (Next disabled until accepted), install-location, ready, progress, and
exit pages appear and the exit page confirms success (contract C-INST-1..3).

- [ ] T002 [P] [US1] Add `packaging/windows/License.rtf` containing the Apache-2.0 license text as RTF (UTF-8 without BOM, LF, no en/em dashes).
- [ ] T003 [US1] In `wix/main.wxs`, add `<UIRef Id="WixUI_InstallDir" />`, `<Property Id="WIXUI_INSTALLDIR" Value="APPLICATIONFOLDER" />`, and `<WixVariable Id="WixUILicenseRtf" Value="packaging\windows\License.rtf" />`.

---

## Phase 4: User Story 2 - The application is easy to find and start (Priority: P1)

**Goal**: A desktop shortcut plus a de-elevated launch-on-finish option, alongside
the preserved Start Menu entry.

**Independent Test**: After install, confirm a desktop shortcut and Start Menu
entry both start the app, the exit-page checkbox launches it de-elevated, and a
silent install does not launch it (contract C-INST-4..7).

- [ ] T004 [US2] In `wix/main.wxs`, add `<Directory Id="DesktopFolder" Name="Desktop" />` and an `ApplicationDesktopShortcut` component (Shortcut to `[#EsoWeaveExe]`, `WorkingDirectory=APPLICATIONFOLDER`, `Icon=ProductIcon`, HKCU `desktop_shortcut` keypath), and add its `ComponentRef` to the `Application` feature.
- [ ] T005 [US2] In `wix/main.wxs`, add the launch-on-finish wiring: `WIXUI_EXITDIALOGOPTIONALCHECKBOXTEXT`, `WIXUI_EXITDIALOGOPTIONALCHECKBOX=1`, `WixShellExecTarget=[#EsoWeaveExe]`, a `LaunchApplication` custom action (`WixCA`/`WixShellExec`, `Impersonate="yes"`, `Return="asyncNoWait"`), and the `<UI><Publish Dialog="ExitDialog" Control="Finish" Event="DoAction" Value="LaunchApplication">WIXUI_EXITDIALOGOPTIONALCHECKBOX = 1 and NOT Installed</Publish></UI>`.

---

## Phase 5: User Story 3 - The application opens cleanly (Priority: P2)

**Goal**: Release builds are windowless; no console flash on launch.

**Independent Test**: Launch a release build from a shortcut and confirm only the
application window appears, with no console window (contract C-APP-1).

- [ ] T006 [US3] Add `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` at the crate-bin root of `src/main.rs`, immediately after the module doc block.

---

## Phase 6: User Story 4 - First-run failures are never silent (Priority: P2)

**Goal**: A startup panic shows a native dialog and writes a log line; the logic
is unit-tested behind a seam.

**Independent Test**: Force an early startup panic in a release build; confirm a
native error dialog appears and a matching error line is in the log (contract
C-APP-2..3). Unit tests cover `panic_message` and the notifier gating (C-APP-4..5).

- [ ] T007 [P] [US4] Write failing unit tests in `src/startup/mod.rs` for `panic_message` (non-empty output, location omitted when `None`) and for hook gating via `MockNotifier` (notifier invoked when gui-started flag is false, not when true; logs in both cases).
- [ ] T008 [US4] Implement the pure `panic_message(payload: &str, location: Option<String>) -> String` in `src/startup/mod.rs` to pass its tests.
- [ ] T009 [US4] Implement the `Notifier` trait in `src/startup/mod.rs` with a Windows `MessageBoxW` impl (via `windows-sys` `Win32_UI_WindowsAndMessaging`) and a `MockNotifier` for tests.
- [ ] T010 [US4] Implement `install_hook` in `src/startup/mod.rs`: `std::panic::set_hook` that always logs `panic_message` at `tracing::error` (target `eso_weave`) and invokes the notifier only while a shared `AtomicBool` gui-started flag is false.
- [ ] T011 [US4] In `src/main.rs`, install the panic hook before spawning worker threads and set the gui-started flag true immediately before `eframe::run_native`, without altering worker-thread startup ordering.

---

## Phase 7: User Story 5 - Users can find the application and its logs (Priority: P3)

**Goal**: Docs state shortcut locations and the log directory.

**Independent Test**: Read the README install section and confirm it names the
Start Menu and desktop shortcuts and the log directory (contract C-DOC-1).

- [ ] T012 [P] [US5] Update the install section of `README.md` to state the Start Menu and desktop shortcut locations and the Windows log directory (`%APPDATA%\eso-weave\logs\YYYY-MM.log`).
- [ ] T013 [P] [US5] Verify `docs/releasing.md` still describes a single Windows MSI artifact and leave it unchanged (contract C-DOC-2).

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Record decisions, prove CI parity, and validate the built MSI.

- [ ] T014 Update `CHANGELOG.md` `[Unreleased]`: an Added entry for the installer wizard, launch option, desktop shortcut, windowless build, and startup failure dialog; plus a dated Decisions entry for the pinned-artifact changes to `wix/main.wxs` and `packaging/windows/License.rtf`.
- [ ] T015 Run CI parity in the foreground to completion: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`.
- [ ] T016 On Windows, build the MSI (`cargo build --release --locked --bin eso-weave` then `cargo wix --no-build --nocapture`), confirm `WixUIExtension`/`WixUtilExtension` link cleanly, and walk the quickstart install checks (wizard, license gate, de-elevated launch, desktop + Start Menu shortcuts, silent-install no-launch, startup-failure dialog + log line).

---

## Dependencies

- **T001** (setup) precedes all Rust tasks (T006-T011).
- **US1**: T002 (License.rtf) should land before or with T003 (which references it). T003, T004, T005 all edit `wix/main.wxs`, so they run sequentially in that order.
- **US4**: T007 (failing tests) precedes T008-T010; T011 wires the hook after T008-T010. T006 and T011 both edit `src/main.rs`, so T006 precedes T011.
- **Polish**: T014-T016 run after all story phases. T016 (MSI build/walkthrough) is Windows-only and runs last.

## Parallel Opportunities

- T002 (License.rtf) is parallel with T001.
- T007 (startup tests) is parallel with the WiX tasks (T003-T005), different files.
- T012 and T013 (docs) are parallel with each other and with the Rust work.

## Implementation Strategy

- **MVP**: US1 + US2 (both P1) deliver the core fix: a visible, license-gated
  install that ends in a confirmation and leaves the app trivially launchable.
- **Increment 2**: US3 + US4 (P2) polish the launch and make failures loud.
- **Increment 3**: US5 (P3) documents the result.

Total tasks: 16. US1: 2, US2: 2, US3: 1, US4: 5, US5: 2, setup/polish: 4.
