# Implementation Plan: Stale HUD Retention

**Branch**: `codex/s098-stale-hud-retention` | **Date**: 2026-09-15 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/098-stale-hud-retention/spec.md`

## Summary

Add a bounded `stale_retention_seconds` UI preference and a single process-local presentation cache inside `AppModel`. The cache clones already-rendered HUD values only while game evidence is coherent, preserves them across runtime, focus, or signal loss until one monotonic deadline, and exposes a visible stale cause and age. Existing observation routing propagates the shared transition timestamp, while authoritative evidence and every input-producing controller keep their existing fail-closed semantics.

## Technical Context

**Language/Version**: Rust 1.96, edition 2021; Markdown and JSON documentation

**Primary Dependencies**: egui/eframe, serde_json, existing application view model and game observation model

**Storage**: Existing `config.json` UI section for one user preference; retained HUD data is memory-only

**Testing**: Rust unit and integration tests through `cargo test --all --locked`; documentation policy, mdBook, render-smoke, spelling, encoding, and diff gates

**Target Platform**: Windows 10/11 x64 and Linux x64

**Project Type**: Single-crate desktop application

**Performance Goals**: No new I/O, worker, polling loop, or per-widget timer; at most one small cloned presentation snapshot and constant-time projection per frame

**Constraints**: Inclusive 0 through 999 seconds; default 120; monotonic timing; stale values never reach action authority; process exit drops the cache

**Scale/Scope**: One settings field, one presentation-retention model, one dashboard freshness row, focused tests, canonical documentation, changelog, and active-plan update

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Principle I, Spec-Driven Development**: PASS. Issue #171, this complete spec-kit package, Plan 043, and canonical documentation form the authority chain. Implementation starts after tasks and analysis.
- **Principle II, Safety-Critical Surfaces**: PASS. The design adds only a transition timestamp to existing observation routing. It does not retain evidence or change controller and input-gate behavior. Presentation types have no route back to action code.
- **Principle III, Test-First**: PASS. Settings, state transition, recovery, expiry, and safety-separation tests precede implementation.
- **Principle IV, CI Parity**: PASS. Full fmt, strict Clippy, and locked tests run before each Rust commit.
- **Principle V, Bounded Scope**: PASS. No addon, protocol, sampler, controller, input backend, encounter, catalog, or durable player-state changes.
- **Configuration and text hygiene**: PASS. The additive UI field needs no top-level schema bump; changed text remains UTF-8 without BOM and avoids forbidden dash characters.

## Design

### Configuration

Extend `UiPrefs` with `stale_retention_seconds: u16`, default 120. Parse only a JSON integer in 0 through 999. Missing data inherits the default; invalid data emits `NoticeKind::InvalidValue`. Serialize the field through `ui_to_value`. Render it with `egui::DragValue`, `range(0..=999)`, speed 1, and a seconds suffix.

### Presentation cache

Add a private interior-mutable retention state to `AppModel` because `view` is intentionally a read projection from the caller's perspective. The state contains the last coherent `HudPresentation` and optional stale interval. `HudPresentation` contains only existing rendered fields, including cooldown projections, and cannot be consumed by controller APIs.

`view_at(now_ms)` is the deterministic projection seam; ordinary `view()` delegates with the existing injected-clock `now_ms()`. Existing process and reader routes stamp the first coherent-to-incoherent transition on that same clock, so a delayed repaint cannot reset age. When evidence is coherent, the model records the new presentation and clears stale state. On a covered loss, it starts one interval from the stamped loss if a snapshot exists and retention is positive. A later cause changes only the explanation. Expiry uses the lesser of the original deadline and a deadline recomputed from the original loss timestamp plus the current configured interval, so settings can shorten but never extend or restart retention.

### Accessibility and safety

Add an optional `StatusLine` to `AppView` titled `HUD Freshness`. Its visible state includes `Stale`, a truthful cause, and whole-second age; the static tooltip explains retention and states that automation remains blocked. The Live HUD displays this row before retained values.

Reader and process loops continue to close gates and clear authoritative observations exactly as they do today. They also pass their existing monotonic `now_ms` into Game State so it can expose the loss transition time atomically with current observations. No presentation cache is passed into `GameState`, `WeaveEngine`, Fishing, Auto Potion, `InputEngine`, routing, persistence, or logs.

## Project Structure

### Documentation (this feature)

```text
specs/098-stale-hud-retention/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── issue-171.md
├── contracts/
│   ├── presentation-retention.md
│   └── settings.md
├── checklists/
│   ├── requirements.md
│   └── presentation-safety.md
└── tasks.md
```

### Source and repository integration

```text
src/app/settings_form.rs
src/app/mod.rs
src/app/strings.rs
src/app/ui.rs
tests/app_settings.rs
tests/app_view_model.rs
tests/app_strings.rs
tests/app_ui_sizing.rs
docs/src/reference/settings.md
docs/src/reference/configuration.md
docs/src/features/interface.md
docs/src/development/architecture.md
docs/src/development/test-strategy.md
docs/src/getting-started/troubleshooting.md
docs/project/build-plans/plan-043.md
docs/project/build-plans/README.md
CHANGELOG.md
CLAUDE.md
.specify/feature.json
```

**Structure Decision**: Extend the established application view projection and UI settings section. A cache in `GameState`, `WeaveEngine`, a controller, or a new worker would mingle presentation memory with authoritative evidence and is rejected.

## Verification Strategy

1. Add settings tests for default, both bounds, exact round trips, and invalid type or range fallback.
2. Add deterministic `view_at` tests that establish a coherent snapshot and independently exercise inactivity, runtime unknown, focus loss, focus unknown, signal loss, no snapshot, recovery, subsequent loss, expiry, repeated expiry, zero, and live interval edits.
3. Assert safety separation by checking authoritative weave fields clear and input/controller gates close while the rendered snapshot remains stale.
4. Add UI string and sizing coverage for the new settings row and freshness presentation.
5. Run full CI parity, release build, documentation gates, spec checks, text encoding and forbidden-dash scans, and complete-diff review.

## Complexity Tracking

No constitution violations or complexity exceptions are required.
