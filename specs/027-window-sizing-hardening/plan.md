# Implementation Plan: UI Window-Sizing and Layout Hardening

**Branch**: `027-window-sizing-hardening` (trunk-based; commits land on `main`)
| **Date**: 2026-07-24 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from
`specs/027-window-sizing-hardening/spec.md`

## Summary

Bundle four interacting GUI defects/enhancements (GitHub issues #4, #5, #6, #7)
into one slice so the window always shows every interactive control, the live
log viewer behaves correctly, the save confirmation is quiet, and controls are
shorter. The technical spine is a single measured "central-panel content extent"
(the minimum width and height needed to show every control without clipping),
computed once per frame and consumed by the window minimum size (#4), the
log-pane upper limit (#5d), and shrinking automatically when controls get
shorter (#7). The save-confirmation fix (#6) is an independent, pure scheduler
change. All sizing math is extracted into pure, unit-tested helpers mirroring the
existing `clamp_log_height` / `modal_extent` pattern, and implemented test-first.

## Technical Context

**Language/Version**: Rust (edition 2021), toolchain pinned 1.96.0
(`rust-toolchain.toml`).

**Primary Dependencies**: `eframe` / `egui` 0.35 (immediate-mode GUI), single
binary+library crate `eso-weave` (no workspace).

**Storage**: JSON config store (user settings, includes the log-pane-height UI
preference) and JSON session-state store (window geometry). Both already exist;
this slice changes neither's written content.

**Testing**: `cargo test` with out-of-crate integration suites in `tests/`
(`tests/app_*.rs`, `tests/app_session_state.rs`). Pure helpers are unit-tested;
visual behavior is validated manually per `quickstart.md`.

**Target Platform**: Windows 10/11 x64 and Linux x64 desktop.

**Project Type**: Desktop application (GUI layer of an otherwise
headless-testable app).

**Performance Goals**: 60 fps idle-repaint budget unchanged; the per-frame
content-extent measurement is a rect read plus a running max, negligible cost.

**Constraints**: All behavior verifiable at the desk without the running game;
no in-game validation owed by this slice. UTF-8 without BOM, LF, no em/en
dashes. No changes to pinned artifacts.

**Scale/Scope**: One GUI window, roughly four source modules touched
(`src/main.rs`, `src/app/{ui,mod,theme}.rs`; `widgets.rs` needs no logic change),
plus one test file extended and one added. No new crates, no new runtime
dependencies.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **I. Spec-Driven Development (NON-NEGOTIABLE)**: PASS. This slice is a numbered
  `specs/027-*` feature spec'd through the full sequence and traces to master
  spec Section 11 (GUI) and Section 12 (Config and Session State, window geometry
  persistence). The `/speckit-analyze` gate will run and must pass.
- **II. Safety-Critical Surfaces Are Sacrosanct (NON-NEGOTIABLE)**: PASS, not
  touched. This slice changes no input-suppression scope, no hook-thread work, no
  PixelBeacon uninstall path, no AddOns discovery, no fishing degrade path. No
  safety-critical test is weakened or skipped.
- **III. Test-First With Explicit Seams**: PASS. Each new pure helper
  (content-minimum extent, six-line log minimum, scheduler notify flag) gets a
  failing unit test before its implementation. The pure-helper seam keeps the
  sizing math verifiable without a live window.
- **IV. CI Parity Before Every Commit (NON-NEGOTIABLE)**: PASS. `cargo fmt --all
  -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and
  `cargo test --all --locked` run in the foreground to completion before the
  single commit.
- **V. Bounded Scope: Outside The Game**: PASS. GUI-only; no game memory,
  network, or added in-game functionality. Weave engine untouched.

**Platform / Config / Text hygiene**: PASS. Single crate preserved (no workspace
promotion). Config still stores user settings only (log height is already a UI
preference; window geometry is session state); neither's schema or written
content changes. All files UTF-8 no BOM, LF, no em/en dashes.

**Pinned artifacts**: none changed by this slice. The `v0.7.0` release
(`Cargo.toml` version bump, tag, assets) is performed at release time per
`docs/releasing.md` under separate explicit authorization, not by this plan's
code.

**Result**: Constitution Check PASS. No violations; Complexity Tracking not
required.

## Project Structure

### Documentation (this feature)

```text
specs/027-window-sizing-hardening/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output (manual visual validation guide)
├── contracts/
│   └── ui-window-sizing.md   # Behavioral contract for the sizing/notify helpers
├── checklists/
│   ├── requirements.md  # Spec quality checklist
│   └── ux.md            # UX layout requirements-quality checklist
└── tasks.md             # Phase 2 output (/speckit-tasks)
```

### Source Code (repository root)

```text
src/
├── main.rs              # Viewport min-inner-size wiring (#4); boot floor; RestoreBounds feed
└── app/
    ├── mod.rs           # Pure helpers: content-min extent + six-line log min (#4, #5);
    │                    #   SaveScheduler notify flag + silent layout marks (#6);
    │                    #   ToggleLogPanel grow/shrink handling (#5a)
    ├── ui.rs            # Log bottom-panel min/max + grow-on-open (#5); toast gate (#6);
    │                    #   per-frame content-extent capture + MinInnerSize command (#4)
    ├── theme.rs         # Style-level interact_size / padding reduction (#7)
    └── widgets.rs       # toggle_switch height already rides interact_size (#7, no logic change)

tests/
├── app_session_state.rs   # Extended: SaveScheduler notify flag (#6)
└── app_window_sizing.rs    # New: content-min extent + six-line log min pure helpers (#4, #5)
```

**Structure Decision**: Single-crate desktop app, unchanged. All edits live in
the existing `src/app` GUI module and `src/main.rs`. Sizing math is pulled into
pure functions in `src/app/mod.rs` (the module that already hosts
`clamp_log_height` and `modal_extent`), and exercised by a new
`tests/app_window_sizing.rs` integration suite alongside the extended
`tests/app_session_state.rs`.

## Design decisions (recorded per autopilot decision policy)

- **D1 (control-height figure, #7)**: FINAL: `interact_size.y` reduced from 22.0
  to 17.6 points (a 20 percent reduction), and `button_padding.y` from 5.0 to 4.0,
  set once in `theme::apply`. The reduction is computed by the pure
  `reduced_interact_height(base, font_line_height)` helper as
  `max(base * 0.8, font_line_height + 3.0)`; with the body line height of 14.0 the
  0.8 scale wins (17.6 > 17.0), so the full 20 percent applies while the legibility
  floor guarantees text is never clipped. Rationale: FR-012 requires legibility
  over a fixed percentage; centralizing in the style (not per-call) satisfies
  FR-011 and lets `toggle_switch` (which reads `interact_size.y`) shrink for free.
- **D2 (measurement source, #4)**: Measure the laid-out content extent from the
  central panel's tight content bounds each frame (`Ui::min_rect().size()` inside
  the central panel), track a running max over the first stable frames, and raise
  the viewport minimum via `ViewportCommand::MinInnerSize`. Rationale: a measured
  floor auto-tracks new rows and the shorter controls (FR-002), where a hand-tuned
  constant silently regressed (the Weapon Bar row is the concrete regression).
  A conservative compile-time constant remains only as the pre-measurement boot
  floor.
- **D3 (log grow-on-open, #5a)**: On `ToggleLogPanel(true)` send
  `ViewportCommand::InnerSize` adding the log pane's minimum height to the current
  inner size; on `ToggleLogPanel(false)` subtract the same amount. Rationale:
  FR-004 and FR-008 require the toggle to be height-neutral and never squeeze the
  central area.
- **D4 (log geometry from one source, #5b/#5d)**: Compute the six-line minimum
  height from the log text row height plus frame margins in a single pure helper,
  used by both the panel `min_size` and `clamp_log_height`, and set the panel
  `max_size` to `window_h - measured_central_min_height` so the pane top can never
  cross the Skills area. Rationale: FR-005 and FR-007; keeping both sides on one
  helper prevents the two clamps drifting apart.
- **D5 (notify flag, #6)**: Add a third boolean to `SaveScheduler` set by
  meaningful marks and left unset by the two layout marks (window geometry, log
  height), returned from `maybe_flush`; the toast fires only when it is set.
  Layout writes get a silent mark path. Rationale: the existing
  `(config, session)` split cannot separate a log-height write (config) from a
  real toggle (config), so a dedicated signal is required (FR-009, FR-010,
  FR-013). Chosen over inferring from intent type at the toast site because the
  scheduler already coalesces and is the correct single decision point, and it is
  unit-testable.
- **D6 (wider width while open, #5c)**: FINAL: `LOG_WIDTH_BONUS = 100.0` points.
  While the log viewer is open, the enforced minimum width is
  `content_min.width + 100.0` via `ViewportCommand::MinInnerSize`, restored to the
  base minimum on close. Rationale: FR-006; a fixed increment is simpler and
  sufficient versus measuring log line lengths, and 100 points sits in the middle
  of the 80 to 120 target band.

## Complexity Tracking

No constitution violations. Table not required.
