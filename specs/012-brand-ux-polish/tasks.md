---

description: "Task list for Brand and UX Polish (012)"
---

# Tasks: Brand and UX Polish

**Input**: Design documents from `specs/012-brand-ux-polish/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Minimal, targeted unit tests for new pure helpers only (theme token
completeness, window-icon decode); the UI layer stays manually validated per the
quickstart. No new tests for `wix`/assets (validated by build and inspection).

**Organization**: Grouped by user story. US1 and US2 are both P1; US3 is P2; US4
is P3. The design sign-off and shared brand assets are foundational and block the
user stories.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions

Single-crate desktop app: `src/`, `assets/`, `packaging/`, `wix/`, `docs/`.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Dependency and scaffolding changes that later tasks build on.

- [ ] T001 Add a minimal PNG decode dependency to [Cargo.toml](Cargo.toml): `image = { version = "0.25", default-features = false, features = ["png"] }` (do not touch the pinned `[package.metadata.deb]` section).
- [ ] T002 Add the Windows exe-icon build dependency to [Cargo.toml](Cargo.toml): `[target.'cfg(windows)'.build-dependencies] winresource = "0.1"`.
- [ ] T003 [P] Create the `assets/brand/` and `assets/brand/fonts/` directories and add a `README` recording the reproducible icon-generation recipe (SVG -> ICO/PNG via ImageMagick/rsvg).

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Lock the visual direction and produce the shared brand assets and
tokens that every user story consumes. The sign-off is a hard gate.

- [ ] T004 Produce the design sign-off visual proof: a rendered HTML/SVG mockup of the themed window on dark and light, the weave-knot mark at 16/32/48/256px on light and dark, and the installer wizard; capture final exact hex tokens. HALT for operator approval before any egui/WiX code change.
- [ ] T005 Author the abstract weave-knot master `assets/brand/eso-weave-mark.svg` and the theme-safe badged variant `assets/brand/eso-weave-mark-badge.svg` from the approved proof.
- [ ] T006 Write the brand standard document [docs/brand/ESO-Weave-Brand-v1.md](docs/brand/ESO-Weave-Brand-v1.md) with the final dark/light color tokens, typography (Inter, OFL 1.1), spacing/radius scale, and iconography/accent rules.
- [ ] T007 [P] Add the bundled font `assets/brand/fonts/Inter-Regular.ttf` (and any needed weight) plus its `OFL.txt` license.
- [ ] T008 Regenerate `assets/icon.ico` (16/32/48/64/128/256) from the badge master via the recorded recipe.
- [ ] T009 [P] Regenerate `packaging/linux/eso-weave.png` (256) and `packaging/appimage/AppDir/eso-weave.png` (256) from the badge master.
- [ ] T010 [P] Add the bundled window-icon PNG (for example `assets/brand/window-icon-256.png`) rendered from the badge master.
- [ ] T011 [P] Refresh the wordmark/banner `assets/eso-weave-banner.png` and `assets/eso-weave-logo-clear.png` / `assets/eso-weave-logo-white.png` to the new type system and palette.

**Checkpoint**: Direction approved; SVG master, brand doc, font, and all raster
icons exist. User stories can now proceed.

---

## Phase 3: User Story 1 - Polished, cohesive application window (Priority: P1)

**Goal**: The running window expresses the teal-on-ink theme (dark + light) with
aligned skill columns, pointer cursors, and a window icon.

**Independent test**: Launch the app on dark and light; confirm the palette, the
aligned seven-row skill grid, pointer cursor on every clickable control, and the
title-bar/taskbar icon.

- [ ] T012 [US1] Create `src/app/theme.rs`: brand token structs (per data-model), a `visuals(theme) -> egui::Visuals`, and spacing/corner-radius setup for dark (default) and light.
- [ ] T013 [P] [US1] Add a unit test in `src/app/theme.rs` asserting every color role is defined for both themes and accent/text meet the documented contrast target.
- [ ] T014 [US1] Add font installation in `src/app/theme.rs` (or a helper) building `egui::FontDefinitions` with bundled Inter as the proportional family plus default-font fallback; call it once from `src/app/ui.rs`.
- [ ] T015 [US1] Wire the theme at the `apply_prefs` seam in [src/app/ui.rs](src/app/ui.rs) (replace `Visuals::dark()/light()` at line ~82 with `theme::visuals(..)` + spacing/radius), and install fonts on first paint.
- [ ] T016 [US1] Add a pointer-cursor helper (for example `fn clickable(resp) -> Response` applying `.on_hover_cursor(CursorIcon::PointingHand)`) and apply it to all buttons, checkboxes, and combo headers in [src/app/ui.rs](src/app/ui.rs).
- [ ] T017 [US1] Replace the per-row `ui.horizontal` skill loop in [src/app/ui.rs](src/app/ui.rs) (~line 199) with a single `egui::Grid`; columns label/active/weave/override/value; always allocate the value cell so rows match width; keep intents unchanged.
- [ ] T018 [P] [US1] Retune status colors (`src/app/ui.rs` beacon dot) and log-level colors ([src/app/log_view.rs](src/app/log_view.rs)) to be sourced from the theme tokens and legible in both themes.
- [ ] T019 [US1] Set the window icon in [src/main.rs](src/main.rs): decode the bundled PNG via the `image` helper into `egui::IconData` and pass it through `NativeOptions { viewport: ViewportBuilder::default().with_icon(..) }`.

---

## Phase 4: User Story 2 - Modern, legible brand mark everywhere (Priority: P1)

**Goal**: The same mark appears on the exe file, shortcuts, and Linux launcher, and
is legible at all sizes. (Window icon is delivered in US1; assets in Phase 2.)

**Independent test**: Inspect the mark at 16/32/48/256px on light and dark; confirm
the exe file icon and each shortcut/launcher show the mark.

- [ ] T020 [US2] Add `build.rs` at repo root embedding `assets/icon.ico` as the Windows exe icon via `winresource`, guarded so it is a no-op on non-Windows targets.
- [ ] T021 [US2] Verify the shortcut/ARP icon references in [wix/main.wxs](wix/main.wxs) still resolve to the regenerated `assets/icon.ico` (no path change expected; confirm the new ICO is picked up).
- [ ] T022 [P] [US2] Confirm the deb/AppImage asset maps ([Cargo.toml](Cargo.toml) `[package.metadata.deb]`, `packaging/appimage/AppDir`) reference the regenerated PNGs (paths unchanged; content refreshed).
- [ ] T023 [US2] Validate icon legibility at 16/32/48/256px on light and dark backgrounds; if any size degrades, adjust the badge master and regenerate (loop T005/T008/T009/T010).

---

## Phase 5: User Story 3 - Trustworthy installer experience (Priority: P2)

**Goal**: Readable license page, opt-in (default off) desktop shortcut, branded
wizard art. All are pinned-artifact changes.

**Independent test**: Build and run the MSI; confirm the license page, the
default-off shortcut checkbox honored on install, and branded wizard art.

- [ ] T024 [US3] Reformat [packaging/windows/License.rtf](packaging/windows/License.rtf) to a proportional font with headings and paragraph spacing, preserving the full Apache-2.0 text verbatim.
- [ ] T025 [US3] Author the wizard bitmaps `packaging/windows/banner.bmp` (493x58) and `packaging/windows/dialog.bmp` (493x312) from the brand system.
- [ ] T026 [US3] In [wix/main.wxs](wix/main.wxs): add `WixUIBannerBmp` and `WixUIDialogBmp` variables pointing at the new bitmaps.
- [ ] T027 [US3] In [wix/main.wxs](wix/main.wxs): switch the UI to `WixUI_FeatureTree`, move `ApplicationDesktopShortcut` into its own `Feature Id="DesktopShortcut" Level="2"` (off by default, opt-in via the Custom Setup step), and make the `Application` feature `Absent="disallow"` with `ConfigurableDirectory="APPLICATIONFOLDER"`; the Start Menu shortcut stays in the always-installed feature. (Operator chose the Custom Setup feature over a single install-page checkbox, which would require replacing the whole WixUI dialog set and cannot be validated without a local WiX build.)
- [ ] T028 [US3] Record dated `### Decisions` bullets in [CHANGELOG.md](CHANGELOG.md) under `[Unreleased]` for the pinned changes: `wix/main.wxs` (shortcut opt-in + wizard bitmaps), `License.rtf` reformat, and the regenerated packaging icons/bitmaps.

---

## Phase 6: User Story 4 - Brand standard keeps future work consistent (Priority: P3)

**Goal**: Register the slice and make the brand doc the authoritative reference.

**Independent test**: The brand doc resolves any color/type decision; the plan
index and master spec reference the brand standard.

- [ ] T029 [US4] Add a `plan-002.md` row to [docs/plans/README.md](docs/plans/README.md) and create [docs/plans/plan-002.md](docs/plans/plan-002.md) registering the brand-and-UX-polish slice.
- [ ] T030 [P] [US4] Add a light note to the GUI section of [docs/ESO-Weave-Specification-v0.1.0.md](docs/ESO-Weave-Specification-v0.1.0.md) that a brand standard (`docs/brand/ESO-Weave-Brand-v1.md`) now governs visual identity.

---

## Phase 7: Polish and Cross-Cutting Concerns

**Purpose**: Verify the whole slice end to end.

- [ ] T031 Run CI parity in the foreground: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`.
- [ ] T032 Execute the [quickstart.md](quickstart.md) validation: run the app (theme, columns, cursor, window icon on dark and light), build the release exe (file icon), and build/run the MSI (license, shortcut checkbox, wizard art).
- [ ] T033 [P] Text-hygiene sweep: confirm all new/edited text files are UTF-8 without BOM, LF, and contain no em/en dashes.
- [ ] T034 Confirm the view-model (`src/app/mod.rs`) and its unit tests are unchanged (presentation-only invariant, FR-018).

---

## Dependencies and Execution Order

- Phase 1 (Setup) -> Phase 2 (Foundational, includes the T004 sign-off gate) ->
  Phases 3-6 (user stories) -> Phase 7 (Polish).
- T004 (design sign-off) blocks all of Phases 3-6.
- US1 (Phase 3) depends on the font (T007) and window-icon PNG (T010) and tokens
  (T006). US2 (Phase 4) depends on the ICO/PNGs (T008-T010). US3 (Phase 5) depends
  on the wizard bitmaps (T025) and license text; it is otherwise independent of
  US1/US2 code. US4 (Phase 6) depends only on the brand doc existing (T006).
- Within a phase, `[P]` tasks touch different files and can run together.

## Parallel Opportunities

- Phase 2: T007, T009, T010, T011 in parallel after T005 (SVG master).
- Phase 3: T013 and T018 in parallel with the core theme wiring once T012 lands.
- Phase 4 vs Phase 5: US2 wiring and US3 installer work touch disjoint files and
  can proceed together after Phase 2.

## Implementation Strategy

- MVP = User Story 1 (the polished window): highest visible value, independently
  demoable, no installer needed.
- Then US2 (mark everywhere) and US3 (installer), then US4 (doc/registration).
- Commit in coherent increments with CI parity each time; halt once before pushing.
