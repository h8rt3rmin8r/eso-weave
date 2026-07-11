# Requirements Quality Checklist: Weapon-Bar-Aware Adaptive Timing

**Purpose**: Validate that the requirements for slice 014 are complete, clear, consistent, and measurable across detection and relay, decode and timing, the pinned-contract and safety surfaces, and the offline-validation boundary, before planning.
**Created**: 2026-07-11
**Feature**: [spec.md](../spec.md)

## Requirement Completeness (detection and relay)

- [ ] CHK001 - Are the values the addon must determine (active bar, per-bar weapon class) fully enumerated? [Completeness, Spec FR-001, FR-002]
- [ ] CHK002 - Is the set of normalized weapon classes defined as a fixed, named list? [Completeness, Spec FR-002]
- [ ] CHK003 - Is the edge-detect requirement (re-emit only on a real change) stated so per-attack event noise cannot churn the signal? [Completeness, Spec FR-004]
- [ ] CHK004 - Are the indeterminate states (locked, none, post-load, death) and the hold-last-good behavior specified? [Completeness, Spec FR-005]
- [ ] CHK005 - Is the re-baseline trigger set (after loading screen, on death and revive) defined? [Completeness, Spec FR-005]

## Requirement Clarity

- [ ] CHK006 - Is "normalized weapon-class code" defined as decoupled from raw game enum integers, with the mapping owned by the addon? [Clarity, Spec FR-003]
- [ ] CHK007 - Is "faster-heavy" vs "slower-heavy" weapon class expressed as an ordering the presets must satisfy, not a vague adjective? [Clarity, Spec FR-010]
- [ ] CHK008 - Is the auto-versus-manual precedence unambiguous (auto follows detection; manual used when auto off; manual preserved)? [Clarity, Spec FR-011, FR-012]
- [ ] CHK009 - Is the no-signal fallback concretely defined (front profile, unknown classes)? [Clarity, Spec FR-013]

## Requirement Consistency

- [ ] CHK010 - Do the addon relay (FR-006) and the app decode (FR-008) agree on the same block and the same class set? [Consistency, Spec FR-006, FR-008]
- [ ] CHK011 - Is the per-bar meaning (timing profiles, not skill slots) consistent across user stories, requirements, and assumptions? [Consistency, Spec Assumptions]

## Acceptance Criteria Quality

- [ ] CHK012 - Can "a bar change switches the effective profile" be objectively verified against a decoded signal? [Measurability, Spec SC-001]
- [ ] CHK013 - Can "faster class yields a shorter heavy delay than a slower class" be verified from the presets? [Measurability, Spec SC-002]
- [ ] CHK014 - Is "manual values preserved while auto on" verifiable by toggling auto off and reading them back? [Measurability, Spec SC-003]

## Pinned Contract and Safety Surfaces

- [ ] CHK015 - Is it required that the new block is appended without changing the meaning of the existing status, fishing, and latency blocks? [Constraint, Spec FR-006]
- [ ] CHK016 - Is preservation of the managed-marker line (safe uninstall) required alongside the addon change? [Safety, Spec FR-007]
- [ ] CHK017 - Is the addon manifest version bump required? [Completeness, Spec FR-007]
- [ ] CHK018 - Is a dated changelog decision required for each pinned-contract change (pixel bus, reader, manifest)? [Constraint, Spec FR-017]
- [ ] CHK019 - Is the bounded-scope boundary (only the rendered pixel signal, no game memory or network) restated for the new signal? [Constraint, Spec Assumptions]

## Timing Model

- [ ] CHK020 - Is the front and back timing-profile model and its runtime selection by the active bar specified? [Completeness, Spec FR-009]
- [ ] CHK021 - Are weapon-class presets required for every defined class, including a flagged estimate for the unquantified class? [Coverage, Spec FR-010, Edge Cases]
- [ ] CHK022 - Is persistence of the per-bar profiles and the auto-timing preference specified? [Completeness, Spec FR-015]

## Research and Offline-Validation Boundary

- [ ] CHK023 - Is the appendix deliverable (global-cooldown context, per-class defaults with sources, in-game validation owed) specified, and the open item marked closed? [Completeness, Spec FR-016]
- [ ] CHK024 - Is it clear that the shipping preset values and the pixel signal require in-game validation, and that this is a follow-up, not a design blocker? [Clarity, Spec Assumptions]
- [ ] CHK025 - Is the one-hand-and-shield preset flagged as an estimate pending measurement? [Coverage, Spec Edge Cases]

## Constitution Guardrails

- [ ] CHK026 - Is it required that correctness logic (class mapping mirror, decode, profile selection, presets) lives in tested modules with the rendering layer thin? [Constraint, Spec FR-018]
- [ ] CHK027 - Are the text-hygiene constraints (UTF-8 no BOM, LF, no em/en dashes) in force for all new artifacts including the addon Lua? [Constraint, Constitution]

## Notes

- Items are requirements-quality checks (unit tests for the spec), not implementation tests.
- Resolve any unchecked item by updating spec.md before `/speckit-plan`, or record a deliberate deferral.
