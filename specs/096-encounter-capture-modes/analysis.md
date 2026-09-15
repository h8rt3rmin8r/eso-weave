# Specification Analysis: Encounter Capture Modes

**Date**: 2026-09-15
**Result**: PASS, no unresolved critical conflict

## Authority Chain

| Authority | Finding | Resolution |
| --- | --- | --- |
| Issue #183 | Requires single and continuous capture, but still names a desktop toggle | Operator approved the previously recommended safe deviation: in-game control and historical-only desktop status |
| Epic #182 | Requires operating modes and failure behavior through an independently closeable child | S096 completes the remaining child without expanding addon topology |
| S092 command decision | Desktop command vocabulary is zero | No transport, binding, generated input, or live SavedVariables write is introduced |
| S094/S095 capture contract | Selected raw facts remain exact, bounded, and independently replayable | Terminal encounters remain frozen self-contained records inside a new outer spool |
| Constitution 4.0.0 | Explicitly requires one-shot capture | A scoped major 5.0.0 amendment precedes implementation and retains every safety boundary |
| Plan 042 | Names #183 after sufficient lossless-capture progress | #186 is complete; SavedVariables remains the supported fallback while #190 stays independent |

## Requirement Coverage

| Requirements | Design evidence | Task coverage |
| --- | --- | --- |
| FR-001 to FR-004 | state-machine contract, data model | T005, T013-T014, T029-T030 |
| FR-005 to FR-008 | state-machine contract | T006-T008, T015-T017 |
| FR-009 to FR-015 | envelope and recovery contracts | T009-T011, T018-T020, T022-T024 |
| FR-016 to FR-019 | batch-import contract and store model | T025-T038, T039-T045 |
| FR-020 to FR-021 | control checklist and presentation contract | T040-T048 |
| FR-022 | all test phases | T005-T012, T022-T028, T039-T043 |
| FR-023 | canonical documentation phase | T001-T004, T050-T055 |
| FR-024 | local and hosted gate phases | T056-T069 |

All functional requirements have at least one design artifact and one ordered
implementation or verification task. No task lacks an originating requirement.

## Consistency Checks

- Exactly two modes appear across the spec, model, contracts, quickstart, and tasks.
- Stopped, waiting, capturing, interrupted, and failed are states, not modes.
- The selected mode is addon-owned and never duplicated in desktop configuration.
- Terminal encounter records retain schema v2 and addon-format version 3; the
  outer controller alone advances to state schema 1 and addon version 4.
- Normal callback-started records retain S095 replay. Mid-combat records are
  explicitly partial and replay-indeterminate from an exact API opening fact.
- The 32 MiB, 100,000 event, and 100,000 raw limits apply to the whole spool.
- Encounter ordinals and interruption sequences are authoritative and contiguous.
- Active data is status-only on desktop and cannot become terminal imported evidence.
- Import validates every member before one transaction and never edits the source.
- Legacy storage migrations add metadata without rewriting canonical bytes or hashes.
- Issue #190 is not promoted from Release verification or used as proof.

## Security and Failure Analysis

- Shared input remains behind the existing stable 128 MiB no-follow parser boundary.
- New identifiers are capped before display; diagnostics stay value-free.
- Closed enums, checked sums, exact keys, and prefix-monotonic snapshots prevent
  controller ambiguity and historical rewrite.
- No ring buffer or automatic pruning can hide storage pressure.
- Failed sessions clear effective authority and never auto-retry.
- Only last durably flushed authority can resume; pre-flush crash loss stays unknown.

## Complexity Review

The outer spool, atomic batch transaction, and append-only session snapshots are
proportional to the required multi-encounter semantics. A singleton overwrite,
timestamp ordering, or per-record transaction would violate explicit acceptance
criteria. No new crate, addon, transport, desktop configuration authority, or
metric subsystem is introduced.

## Gate Decision

The two initial conflicts are resolved by already approved product direction and
a required scoped constitution amendment. No `[NEEDS CLARIFICATION]` marker,
unmapped requirement, contradictory bound, or unowned verification remains.
Implementation may proceed after the 5.0.0 amendment is applied.
