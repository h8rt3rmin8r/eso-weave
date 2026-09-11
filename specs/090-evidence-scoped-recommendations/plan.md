# Implementation Plan: Evidence-Scoped Encounter Recommendations

**Branch**: `codex/s090-evidence-scoped-recommendations` | **Date**: 2026-09-11 |
**Spec**: [spec.md](spec.md)

**Input**: Feature specification from
`specs/090-evidence-scoped-recommendations/spec.md`

## Summary

Implement issue #136 as a pure, versioned, local recommendation layer over one
immutable S077 `EncounterProjection`. The layer produces at most one dominant
damage review prompt and one low-uptime review prompt, applies conservative and
explicit sample, loss, and unknown-ID gates, carries full provenance on every
item, and remains structurally separate from observed facts and all gameplay
authority. The S078 worker bundles a projection and report from the same in-memory
value, while the encounter-history window renders facts first and provisional
recommendations second. S090 also completes the Plan 039 lifecycle and establishes
the recommendation plan as the sole active build plan.

## Technical Context

**Language/Version**: Rust 2021 edition on pinned Rust 1.96.0

**Primary Dependencies**: Rust standard library, existing `serde`, `egui`, and
`kittest` dependencies only

**Storage**: Existing read-only `catalog.sqlite` plus immutable user-owned
`encounters.sqlite`; recommendations add no persistence

**Testing**: Rust unit/integration tests, headless `kittest`, existing repository
text and documentation policy scripts

**Target Platform**: Windows 10/11 x64 and Linux x64

**Project Type**: One Rust package with desktop binary and maintainer binary

**Performance Goals**: Linear in the bounded projection collections, at most two
advice items, no I/O or allocation proportional to raw event count

**Constraints**: Local-only, deterministic, no network/model/telemetry calls, no
raw or catalog mutation, no persisted recommendation state, no input or automation
authority, UTF-8 without BOM, LF, no forbidden dash characters

**Scale/Scope**: One selected encounter detail, at most 100,000 original events,
bounded projected metric collections, two provisional review rules

## Constitution Check

### Pre-Design Gate

- **Spec-driven sequence**: PASS. Issue #136, canonical encounter documentation,
  S090 specification, requirements checklist, clarification record, and
  recommendation-safety checklist precede this plan.
- **Safety-critical surfaces**: PASS. The recommendation subsystem has no input,
  addon, Pixel Bus, or automation dependency and exposes no action surface.
- **Test-first**: PASS. Pure contract and headless UI tests precede implementation.
- **CI parity**: REQUIRED before every Rust commit.
- **Bounded desktop scope**: PASS. The feature consumes only an in-memory local
  projection and adds no addon bridge, upload, memory access, packet access, or
  gameplay mutation.
- **Text hygiene**: REQUIRED for every changed text file.
- **Autopilot and publication**: PASS. The user explicitly authorized automatic
  branch push and PR publication for S090 and a maximum second Codex review round.

No constitution exception is required.

## Project Structure

### Documentation for this feature

```text
specs/090-evidence-scoped-recommendations/
|-- spec.md
|-- plan.md
|-- research.md
|-- data-model.md
|-- quickstart.md
|-- analysis.md
|-- checklists/
|   |-- requirements.md
|   `-- recommendation-safety.md
|-- contracts/
|   `-- recommendation-report.md
`-- tasks.md
```

### Source code

```text
src/
|-- lib.rs
|-- recommendation/
|   `-- mod.rs
|-- app/
|   |-- encounter_history.rs
|   `-- ui.rs
`-- encounter/
    `-- metrics.rs                 # unchanged fact authority

tests/
|-- encounter_recommendations.rs
`-- app_encounter_history.rs

docs/
|-- src/reference/encounter-data-and-metrics.md
|-- src/development/architecture.md
|-- project/encounter-model.json
|-- project/build-plans/
|   |-- README.md
|   `-- plan-040.md
`-- archive/build-plans/
    |-- README.md
    `-- plan-039.md
```

**Structure Decision**: Add a top-level `recommendation` domain because Encounter
Metrics explicitly does not own advice. The new domain depends only on the public
encounter projection types. The application worker composes both without making
the encounter storage service depend back on recommendations, avoiding a cycle and
preserving facts as the independent authority.

## Design Decisions

1. Use a pure `generate(&EncounterProjection) -> RecommendationReport` function.
   Equal inputs produce equal reports without I/O, clocks, randomness, or mutable
   state.
2. Keep facts, evidence gates, and advice as distinct typed data. UI prose is a
   deterministic projection of rule and reason enums rather than free-form input.
3. Bundle `EncounterProjection` and `RecommendationReport` into one app-owned
   detail value immediately after metric calculation. This prevents catalog
   replacement from mixing a projection with a report built from different data.
4. Keep all unsuppressed advice explicitly provisional. `ready` means the
   repository evidence gates passed, not that live causation or optimality was
   proven.
5. Suppress globally only for insufficient duration, insufficient casts, or
   material declared loss. Unknown IDs qualify the report, omit unknown targets,
   and suppress damage advice only when unknown abilities own at least 25 percent
   of observed damage. This avoids unrelated unknown IDs blocking a valid
   known-effect prompt.
6. Generate at most two stable rules in fixed order: dominant known-ability damage
   share at or above 40 percent, then lowest known-effect uptime at or below 50
   percent. Numeric ID breaks equal-value ties.
7. Copy complete projection and catalog provenance into every advice citation.
   Do not reopen the catalog for names because that could mix versions and the
   projection does not contain localized names.
8. Add no settings toggle. Encounter detail selection is already an explicit local
   action, and the feature has no persistence or external effect.
9. Archive Plan 039 and establish Plan 040 in the same slice so the repository has
   exactly one truthful active build plan.

## Complexity Tracking

| Choice | Why needed | Simpler option rejected |
| --- | --- | --- |
| Top-level recommendation domain | Preserves Encounter Metrics as fact-only and gives isolation a testable dependency boundary | Adding rule logic to `metrics.rs` would conflate observations and advice |
| App-owned combined detail | Guarantees projection and advice share one exact in-memory authority | Regenerating in egui could run every frame and mix catalog versions |
| Typed reasons and citations | Required for stable tests, visible qualification, and safe wording | Boolean eligibility and free-form strings would hide why advice changed |
| Exact integer loss threshold | Avoids floating-point ambiguity and overflow at the 10 percent boundary | A floating ratio could classify boundary cases inconsistently |

## Post-Design Constitution Check

The design keeps local observations immutable, advice disposable, versioned, and
display-only, and all automation boundaries untouched. Tests cover the new policy
before implementation and the full Rust merge gate remains mandatory. No
constitution exception is required.

## Delivery Sequence

1. Add failing pure recommendation contract tests.
2. Implement the typed recommendation domain and pass pure tests.
3. Add failing app worker and headless rendering tests.
4. Compose and render the report without adding action controls.
5. Update canonical machine-readable and published documentation.
6. Archive Plan 039, activate Plan 040, and update chronology and changelog.
7. Run `/speckit.analyze`, resolve findings, then run repository and CI parity
   gates.
8. Commit, push the authorized branch, open the official PR, and complete CI and
   review handling without exceeding the authorized second review round.
