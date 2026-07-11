# Quickstart Validation: GUI Ergonomics, Information Design, and Auto-Save

This guide validates the slice end to end. The egui render layer is not unit-tested,
so the manual checklist below is the acceptance surface; the automated gate covers
the view-model and persistence logic.

## Prerequisites

- A working checkout on branch `013-gui-ergonomics-autosave`.
- Toolchain per `rust-toolchain.toml`.

## Automated gate

Run in the foreground to completion (never backgrounded):

```
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected: all pass. Relevant tests include status and color derivations
(`app_view_model`), the coalesce/save predicate and no-underscore label coverage
(`app_settings`), terminal log styling (`app_log_view`), the session-state
round-trip and safe-default fallback (`app_session_state`), and the
`ui.log_panel_height` round-trip with back-compat default (`config`).

## Manual validation (run the app)

```
cargo run
```

Controls and information design (User Story 1):

1. Every two-state control (suspend/resume, fishing, per-skill enabled, per-skill
   override, each boolean setting) is a toggle switch with distinct, colorized on and
   off states, not a button or bare checkbox.
2. Each section has a heading distinct from body text.
3. The Skills grid has a header row: Skill, Enabled, Weave, Override, Delay, and rows
   align under it.
4. With an override off, the Delay column shows the inherited default (muted,
   read-only), not a literal zero. With it on, editing Delay changes the value for
   the row's weave type.
5. The top region reads Status, Fishing, Pixel Beacon (Addon), title first, with a
   color-coded state field, spanning the Skills width.

Auto-save and session (User Story 2):

6. There is no Save or Apply control anywhere.
7. Change a skill toggle, a timing value, the theme, and the suspend and fishing
   states; close and relaunch; all are restored.
8. Drag a value control continuously; observe a single save toast when it settles,
   not a stream.
9. Confirm on disk that `config.json` holds no suspend or fishing state and that a
   separate `state.json` holds the session state.
10. With the game window unfocused, a restored running or fishing-on state performs
    no input (focus-scoped invariant).

Settings modal (User Story 3):

11. Settings opens as a modal over a dimmed backdrop, not a view swap; it closes on
    outside click, Escape, and the close control.
12. Options are grouped into labeled clusters; no label contains an underscore; each
    option shows an inline help line; the beacon location and environment options are
    present.

Live log (User Story 4):

13. The log is darker than the app and monospace, with per-level colors; the divider
    shows a grab affordance and resize cursor; dragging clamps between the minimum
    and the bottom of the interactive area; the height is restored after a restart.

Tooltips (User Story 5):

14. Hovering any control, section title, or Skills column header shows a concise,
    consistent tooltip.

## Text hygiene

- All new and changed text files are UTF-8 without BOM, LF line endings, and contain
  no em-dashes or en-dashes.
