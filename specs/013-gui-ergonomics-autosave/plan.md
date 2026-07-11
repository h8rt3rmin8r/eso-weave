# Implementation Plan: GUI Ergonomics, Information Design, and Auto-Save

**Branch**: `013-gui-ergonomics-autosave` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/013-gui-ergonomics-autosave/spec.md`

## Summary

Rework the ESO Weave main window so its controls read as what they are and nothing
the user changes is ever lost. Two-state controls become a reusable colorized
toggle switch; sections gain real headings; the Skills grid gains labeled columns
and shows the inherited default instead of a placeholder zero; the top region is
renamed (Status, Fishing, Pixel Beacon (Addon)), colorized, and spread full width;
the live log becomes a resizable, darker, monospace terminal panel; and the
Settings screen becomes a full-frame modal over a dimmed backdrop, organized into
labeled clusters with no underscores and inline help under every option. All
persistence becomes automatic and coalesced (no Apply button), with user settings
staying in `config.json` and the newly-persisted live suspend and fishing intents
kept in a separate state file so the configuration file continues to hold user
settings only. The egui layer stays thin; every correctness-bearing behavior lands
in the tested view-model and subsystem modules.

## Technical Context

**Language/Version**: Rust (2021 edition, workspace toolchain pinned by
`rust-toolchain.toml`)

**Primary Dependencies**: `eframe`/`egui` 0.35 (glow renderer), `serde` +
`serde_json`, `tracing`/`tracing-subscriber`, `dirs`

**Storage**: `config.json` (existing, user settings only, per-subsystem opaque JSON
sections; additive `ui.log_panel_height`) plus a new separate session state file
(`state.json`) for the live suspend and fishing intents

**Testing**: `cargo test --all --locked` (unit tests in `src/**` and integration
tests in `tests/**`); the egui render layer is validated by the feature quickstart
checklist, not unit tests

**Target Platform**: Windows 10/11 x64 and Linux x64 (x11 and wayland)

**Project Type**: single-crate desktop application

**Performance Goals**: smooth interactive UI at the existing repaint cadence;
coalesced saves so a continuous drag produces a single settle-write rather than
one write per frame

**Constraints**: the egui layer stays thin and logic-free; the focus-scoped input
invariant is preserved on session restore; all text UTF-8 without BOM, LF, no
em-dashes or en-dashes; config stores user settings only

**Scale/Scope**: one window, one settings modal, seven skill rows, roughly six
settings clusters; presentation, persistence wiring, and information design only

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development (NON-NEGOTIABLE)**: PASS. This slice is built through
  the full spec-kit sequence under build plan 003; it traces to master
  specification section 10 (GUI). The `/speckit.analyze` gate will run before
  implementation and is not weakened.
- **II. Safety-Critical Surfaces Are Sacrosanct (NON-NEGOTIABLE)**: PASS. No
  safety-critical surface is modified. The one adjacent behavior, restoring a live
  suspend or fishing intent on launch, is covered by a test asserting that a
  restored non-suspended or fishing-active state performs no input synthesis or
  suppression while the game window is unfocused, so the focus-scoped invariant is
  reaffirmed, not weakened.
- **III. Test-First With Explicit Seams**: PASS. New correctness logic (status and
  color state derivations, the coalesced-save trigger, session-state load and store,
  log-height clamping, and the centralized label, help, and tooltip strings) lands
  in the tested view-model and subsystem modules behind the existing seams. The
  egui layer remains the untested-by-design thin shell.
- **IV. CI Parity Before Every Commit (NON-NEGOTIABLE)**: PASS. `cargo fmt`,
  `cargo clippy -D warnings`, and `cargo test --all --locked` run in the foreground
  to completion before the commit.
- **V. Bounded Scope: Outside The Game**: PASS. No game memory, network, or addon
  behavior is touched; this is a presentation and local-persistence slice.

Configuration and text hygiene:

- The configuration file continues to store user settings only. The newly-persisted
  live suspend and fishing intents are session/runtime state and are therefore
  written to a separate `state.json`, never to `config.json`. This decision resolves
  the apparent conflict between the operator's request to persist session state and
  the constitution's rule that no session, runtime, or derived state is written to
  the config file. The log-panel height is a user layout preference and is added to
  the config UI section.
- The separate state file follows the same hygiene as config: JSON, UTF-8 without
  BOM, LF, and it falls back to safe defaults (not suspended, not fishing) when
  absent, unreadable, or holding a state the subsystem cannot resume.

Pinned artifacts: none are modified by this slice.

## Project Structure

### Documentation (this feature)

```text
specs/013-gui-ergonomics-autosave/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 validation guide
├── checklists/
│   ├── requirements.md  # Spec quality checklist (from specify)
│   └── gui-autosave.md  # Domain requirements-quality checklist (from checklist)
└── tasks.md             # Phase 2 output (from /speckit-tasks)
```

No `contracts/` directory: this slice exposes no new external interface and modifies
no pinned contract. The user-facing surface is the application window itself,
validated through `quickstart.md`.

### Source Code (repository root)

```text
src/
├── main.rs                 # add explicit initial and minimum window size
├── app/
│   ├── mod.rs              # view-model: status/color state derivations, intents,
│   │                       #   coalesced-save trigger, session restore wiring,
│   │                       #   centralized label/help/tooltip strings
│   ├── ui.rs               # thin egui layer: menu bar, status grid, skills grid,
│   │                       #   resizable log panel, settings modal, toasts
│   ├── widgets.rs          # NEW: toggle switch, heading, inline-help, toast helpers
│   ├── theme.rs            # wire Inter Medium/SemiBold weights; heading style
│   ├── settings_form.rs    # live auto-apply form (replaces draft-and-Apply)
│   ├── log_view.rs         # terminal styling derivation (monospace, darker bg)
│   └── strings.rs          # NEW (optional): the single source of UI strings
├── config/
│   ├── mod.rs              # additive ui.log_panel_height; unchanged save choke point
│   └── state.rs            # NEW: separate session-state file (load/store/defaults)
└── ...

tests/
├── app_view_model.rs       # status/color derivations, skills default display
├── app_settings.rs         # auto-save/coalesce trigger, no-underscore label map
├── app_log_view.rs         # terminal styling derivation
├── app_session_state.rs    # NEW: session state round-trip + safe-default fallback
└── config.rs               # ui.log_panel_height round-trip + back-compat default
```

**Structure Decision**: Single-crate desktop app, extending the existing
`src/app/` view-model plus thin egui layer split. New presentation helpers live in
`src/app/widgets.rs`; the separate session-state persistence lives in
`src/config/state.rs`; UI strings are centralized (in `mod.rs` or a small
`strings.rs`) so tooltips and inline help stay consistent and testable.

## Complexity Tracking

No constitution violations require justification. The one notable design decision
(session state in a separate state file rather than the config file) increases the
number of persisted artifacts by one, which is the simplest option that satisfies
both the operator's persistence request and the constitution's config-content rule;
folding session state into `config.json` was rejected because it violates that rule.
