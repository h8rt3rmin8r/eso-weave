# Quickstart: Settings Modal, Logging, and Keys (Manual Validation)

The visual behavior is validated against the running app (use the PowerShell
CopyFromScreen capture workaround). The pure helpers and the log linkage are
covered by `cargo test`.

## Automated checks

```
cargo test --all --locked
```

Confirm:

- `Key::display_name` tests (non-empty, no underscore, spot-checked mappings).
- `modal_extent` tests (fits a small window, more pixels but smaller fraction on a
  mid window, capped on a large window).
- Log linkage tests (SetLogFilter updates the settings level; applying settings
  updates the panel filter; toggling the panel does not change the level).

## Manual validation (running app)

1. **F2 selectable**: open Settings, expand a keybinding dropdown, and confirm F2
   appears; confirm the Toggle Fishing binding shows F2 as its selected value and
   can be rebound. (US1, FR-001.)
2. **Friendly names**: confirm each key reads as its friendly name (Number 1,
   Space, F1, F2, etc.), not a raw string, in both the selected value and the
   options. (US2, FR-002.)
3. **Log linkage**: change the live-log level dropdown; open Settings and confirm
   the Log level matches. Change the Settings Log level; confirm the live-log
   dropdown matches. Hide and show the live-log panel and confirm the level is
   unchanged. (US3, FR-003, FR-004.)
4. **Modal scaling**: resize the window small, open Settings, confirm the modal
   fits the window; resize large (toward ultrawide), open Settings, confirm the
   modal is larger in pixels but a smaller fraction and not absurdly large; confirm
   height and width both scale. Resize while the modal is open and confirm it
   tracks. (US4, FR-005, FR-006.)
5. **Green toast**: change any setting and confirm the Settings saved toast appears
   green and legible, in both dark and light themes. (US5, FR-007.)
6. **Dropdown widths**: change each settings dropdown (theme, environment, log
   level, a keybinding) and confirm the resting field width does not change.
   (FR-008.)

## Regression

7. Confirm settings still auto-apply and persist, and the live log still shows
   captured events.
