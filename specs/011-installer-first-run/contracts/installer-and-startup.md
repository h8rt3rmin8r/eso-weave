# Contract: Installer and Startup Behavior

This is the observable behavior contract for slice 011. WiX has no automated unit
harness, so the installer clauses are verified by build plus manual install
(quickstart); the startup clauses are verified by unit tests behind the seam and
by manual first-run checks.

## Installer contract

- **C-INST-1**: Running the MSI interactively presents, in order, a welcome page,
  a license page, an install-location page, a ready/verify page, a progress page,
  and an exit page. (FR-001)
- **C-INST-2**: The exit page states that ESO Weave was installed successfully.
  (FR-002)
- **C-INST-3**: The Next control on the license page is disabled until the user
  accepts the license. (FR-003)
- **C-INST-4**: The exit page shows a "Launch ESO Weave" checkbox, checked by
  default. Finishing with it checked starts the application; finishing with it
  unchecked does not. (FR-004)
- **C-INST-5**: The application started from the exit checkbox runs as the invoking
  (non-elevated) user. (FR-004, D3)
- **C-INST-6**: A silent or unattended install (`msiexec /qn`) completes without
  launching the application. (FR-005)
- **C-INST-7**: A completed install creates a desktop shortcut and a Start Menu
  entry, each of which starts the application. (FR-006, FR-007)
- **C-INST-8**: Standard install, uninstall, and upgrade-in-place all succeed, and
  the install writes only under the chosen Program Files location, the Start Menu,
  and the Desktop (never game or Documents directories). (FR-008)
- **C-INST-9**: A failure to create the desktop shortcut does not fail the overall
  install. (FR-009)

## Startup contract

- **C-APP-1**: Launching the installed application from any shortcut or the exit
  checkbox shows only the application window, with no console window at any point.
  (FR-010)
- **C-APP-2**: If startup fails before the GUI event loop begins, the user sees a
  native error dialog identifying the failure. (FR-011)
- **C-APP-3**: Any startup failure is also written to the application log at error
  level. (FR-011, FR-012)
- **C-APP-4**: `panic_message` returns a non-empty, stable string for a given
  payload and location, and omits the location cleanly when none is provided.
  (testable seam)
- **C-APP-5**: The panic hook invokes the notifier when the "gui started" flag is
  false and does not invoke it when the flag is true; it logs in both cases.
  (testable seam via `MockNotifier`)

## Documentation contract

- **C-DOC-1**: The README install section states where the Start Menu and desktop
  shortcuts are and where log files are written. (FR-013)
- **C-DOC-2**: `docs/releasing.md` continues to describe a single Windows MSI
  artifact (no change required). (D8)
