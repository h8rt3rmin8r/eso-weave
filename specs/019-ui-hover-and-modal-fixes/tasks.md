---

description: "Task list for GUI Hover Reflow and Settings Modal Fixes"
---

# Tasks: GUI Hover Reflow and Settings Modal Fixes

**Input**: Design documents from `specs/019-ui-hover-and-modal-fixes/`

**Tests**: None added. The GUI layer carries no unit-tested logic; both fixes are
verified observationally against the running app. The existing suite must stay
green.

## Phase 1: User Story 1 - No hover reflow (Priority: P1)

- [ ] T001 [US1] In `src/app/theme.rs::apply`, set `expansion = 0.0` on all five
  `WidgetVisuals` states (noninteractive, inactive, hovered, active, open) and
  change `hovered.bg_stroke` width from `1.2` to `1.0`.

## Phase 2: User Story 2 - Settings modal width and scrollbar (Priority: P2)

- [ ] T002 [US2] In `src/app/ui.rs`, add `.auto_shrink([false, false])` to the
  settings `ScrollArea::vertical()`.

## Phase 3: Verification

- [ ] T003 Run `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all --locked`; all green.
- [ ] T004 Run the app; confirm no hover reflow on the Install/Uninstall/menu
  buttons in both themes, and the settings body fills the modal width with the
  scrollbar at the right edge.

## Dependencies

T001 and T002 are independent and touch different files. T003/T004 follow both.
