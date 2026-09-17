# Analysis: Ultimate Auto Potion Resource Watch

**Date**: 2026-09-17

**Status**: PASS (post-implementation)

## Coverage Matrix

| Concern | Specification | Plan and contract | Tasks | Result |
| --- | --- | --- | --- | --- |
| Independent setting and migration | FR-001 through FR-003 | Additive potion JSON model | T008, T014, T015 | Covered |
| Exact Ultimate normalization | FR-004 through FR-008 | Direct telemetry and cross-multiplication decisions | T005, T009 | Covered |
| OR order and diagnostics | FR-009 through FR-011 | Trigger-cause contract | T006, T010, T016 | Covered |
| Existing action safety | FR-012, FR-013, FR-017, FR-020 | Safety checklist and unchanged gate order | T007, T011, T020 | Covered |
| Settings and documentation | FR-014, FR-015 | Reused row model and canonical docs | T017 through T019 | Covered |
| Planning and delivery | FR-018, FR-019 | Plan 045 archive, Plan 046, changelog, PR workflow | T001 through T004, T021 through T034 | Covered |

## Findings

### Critical

None.

### High

None.

### Medium

None.

### Low

1. The existing `TriggerCause` stores one integer percentage, so S110 must document and test a rounding rule for exact ratios. Ceiling division is selected because it aligns with the inclusive threshold predicate.
2. Existing test fixtures construct `PotionReadings` directly. They must all gain explicit Ultimate evidence so defaults cannot accidentally alter `ResourcesUnavailable` assertions.
3. UI sizing is governed by existing responsive modal tests rather than screenshot automation. S110 adds the fourth row to those assertions and includes a manual keyboard and layout quickstart pass.

## Constitution Review

- The issue, specification, clarification, checklists, plan, research, data model, contract, quickstart, tasks, and this analysis exist before production implementation.
- Test-first tasks precede each production change.
- The only new action predicate consumes an existing current-evidence seam and fails closed.
- Safety-critical tests and full CI parity remain mandatory.
- No new addon, protocol, process, network, filesystem, telemetry, or synthesis authority is introduced.
- No pinned artifact change is planned.

## Conclusion

The S110 packet and implementation are internally consistent and complete. No unresolved clarification or critical, high, or medium finding blocks publication.

## Implementation Evidence

- The first focused `potion` test compile failed because the production configuration, reading, and resource variants did not yet expose Ultimate, preserving the required red-before-green sequence.
- Focused potion, application, settings, sizing, player-state, documentation, and local-extension tests pass with the fourth watch and exact-ratio boundary cases.
- `cargo fmt`, strict all-target Clippy, the full locked Rust suite, and the locked release build pass.
- The documentation unit suite, mdBook test and build, rendered-site policy, repository trust policy, release-notes contract, and typo checks pass.
- The deterministic Auto Potion screenshot was regenerated from the repository harness, inspected for the fourth row and keyboard evidence, and recorded with its new digest and byte count.
- The final diff check reports no whitespace errors, and repository policy verifies UTF-8, punctuation, lifecycle, and documentation integrity.
