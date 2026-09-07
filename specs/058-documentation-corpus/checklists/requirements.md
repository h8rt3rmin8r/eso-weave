# Requirements Checklist: Documentation Corpus Reorganization

**Purpose**: Verify that S058 requirements are complete, testable, and bounded.

**Created**: 2026-09-07

**Feature**: [spec.md](../spec.md)

## Scope and Traceability

- [x] CHK001 The specification traces to issue #80 and merged S057.
- [x] CHK002 Published, project, archive, website, and root README lifecycles are explicit.
- [x] CHK003 Every pre-migration file and specification section requires ledger coverage.
- [x] CHK004 All 27 legacy plans and the active S058 plan have measurable outcomes.
- [x] CHK005 #81, #82, #84, #77, and runtime changes are explicitly excluded.

## Testability and Preservation

- [x] CHK006 Navigation, path, ledger, plan-count, README-size, and text-hygiene outcomes are measurable.
- [x] CHK007 Removal of the monolith is gated on named authoritative destinations.
- [x] CHK008 Historical deletion requires a retained replacement and evidence.
- [x] CHK009 Stale references and accidental project/archive publication have negative-test requirements.
- [x] CHK010 GitHub and Pages path behavior is covered.

## Governance and Safety

- [x] CHK011 Constitution and autopilot path changes must remain synchronized.
- [x] CHK012 The dated changelog decision requirement covers governance and pinned process changes.
- [x] CHK013 Safety-critical runtime behavior is explicitly untouched and full Cargo parity remains required.
- [x] CHK014 UTF-8, LF, forbidden-dash, and mojibake constraints are explicit.

## Result

PASS. No unresolved clarification remains; autopilot decisions are recorded in the specification.
