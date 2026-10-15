# Feature Specification: Native v15 Format Coverage

**Feature Branch**: `codex/s131-native-v15-format-coverage`
**Created**: 2026-10-15
**Status**: Specification validated; producer grammar established by pinned static v15 serialization evidence
**Input**: Owner kickoff of S131 for #296 and #297. Establish grammar first. Automatic push and official PR publication are authorized, with at most two Codex review rounds. Stop for owner final review and merge.

## User Scenarios & Testing

### User Story 1 - Process inline ability records automatically (Priority: P1)

A player already using normal/inline native v15 logging receives the same automatic catchup, live processing and source-qualified results as normal/separate users, without changing game settings.

**Why this priority**: Completes the normal-format gap tracked in #296.
**Independent Test**: A reviewed producer-backed normal/inline specimen and explicitly synthetic partitions produce deterministic completed fights through the existing automatic pipeline.

**Acceptance Scenarios**:

1. Given reviewed normal/inline records, when consumed at arbitrary byte boundaries, then exact original evidence, context, offsets, identities and supported measurements remain available.
2. Given a partial record, restart or live append, when processing resumes, then completed fights appear exactly once without manual interaction or source mutation.
3. Given malformed known input, unknown records or a limit violation, when consumed, then controlled outcomes preserve prior commits and recover only at verified boundaries.

### User Story 2 - Process both verbose profiles (Priority: P1)

A player using verbose/separate or verbose/inline native v15 receives automatic processing with the same validated semantic facts and evidence retention as the normal profiles.

**Why this priority**: Completes #297 while sharing validation and processing behavior with #296.
**Independent Test**: Producer-backed evidence establishes exact field names, case, delimiters, grouping and optional forms for both verbose profiles; partitions and restart scenarios produce the declared results.

**Acceptance Scenarios**:

1. Given reviewed verbose records, when processed, then their typed facts use shared validation while their original named-field lines remain verbatim.
2. Given quoted/grouped values, optional fields and supported run/profile transitions, when consumed, then their interpretation follows the reviewed grammar without guessing or positional rewriting of raw evidence.
3. Given unknown or unsupported information, when results are read, then available facts retain source qualification and missing quantities remain explicitly unavailable.

### User Story 3 - Preserve history and consumer contracts (Priority: P1)

Existing normal/separate and historical retained data remain readable and stable while all supported profiles use the existing bounded automatic processing, cancellation and read-only access services.

**Why this priority**: Format expansion must preserve accumulated evidence and the existing operating model.
**Independent Test**: Compare legacy canonical bytes, hashes, identities and ordering, then run automatic ingest, failure/restart, disable and authenticated read scenarios across the supported profiles.

**Acceptance Scenarios**:

1. Given existing normal/separate and older retained rows, when new profiles are added, then reading and any necessary migration preserve original canonical evidence and identities.
2. Given catchup, live append, disable, cancellation or failed persistence, when work advances, then fights and progress remain atomic within existing bounds and stop boundaries.
3. Given current documentation and machine contracts, when consumers inspect capabilities, then each format and measurement claim matches substantiated implementation and its stated evidence level.

### Edge Cases

- Arbitrary byte splits, incomplete UTF-8, LF/CRLF, quoted commas, literal producer strings, legacy accepted escapes, nested equipment arrays, flat unit/ability fields, optional values and empty feeds.
- Run/profile transitions, repeated metadata, self-target abbreviations, unknown types, malformed known layouts and unsupported versions.
- Corrupt or incompatible checkpoints, context/token/fight limits and verified recovery after malformed segments.
- Copy/extracted runs, replacement/truncation, transactional failures, deletion suppression, cancellation and concurrent stored reads.

## Requirements

### Functional Requirements

- **FR-001**: Establish and pin reviewed producer grammar or sanitized representative producer evidence before implementing normal/inline, verbose/separate and verbose/inline version 15. Cover exact field names/case, separators, quoting/escaping, grouped unit/ability values, optional values and supported run/profile transitions. Distinguish observations, declarations, inferences and synthetic tests.
- **FR-002**: Support normal/inline v15 through the existing pure incremental parser and automatic runtime. Preserve exact records, ordered context, physical offsets, source facts and stable logical identities.
- **FR-003**: Support verbose/separate and verbose/inline v15 by normalizing a typed semantic view for shared validation/projection. Retain exact original lines independently; never rewrite raw evidence into positional input.
- **FR-004**: Preserve arbitrary meaningful byte partitioning, incomplete tails, incremental combat completion and validated checkpoint/restart equivalence for each supported profile.
- **FR-005**: Distinguish unknown retained records, malformed known layouts, unsupported profiles/versions and resource-limit outcomes. Preserve existing bounds and verified recovery; no permissive fallback may conceal malformed known input.
- **FR-006**: Preserve normal/separate behavior and all historical canonical bytes, hashes, identities, ordering and capabilities through compatible reading or tested transactional migration. Preserve logical deduplication, controlled collisions and deletion suppression.
- **FR-007**: Reuse automatic bounded discovery, catchup, live append, atomic progress and cancellation. Add no imports, viewers, prompts, format-fixing workflow or game logging/serialization/anonymity changes.
- **FR-008**: Preserve source-qualified measurements in storage, authenticated read-only HTTP and MCP. Missing native/addon/API facts remain unavailable rather than fabricated or zero-filled.
- **FR-009**: Reconcile affected source capabilities, contracts, API/MCP documentation and current guide with substantiated behavior and evidence distinctions.
- **FR-010**: Pass applicable parser, storage, runtime, API, documentation and repository gates, resolve findings within at most two Codex rounds and stop before owner merge. Do not request owner field testing or claim synthetic/repository evidence proves installed-game behavior.

### Key Entities

- **Grammar evidence**: Pinned source or representative producer records, profile/version, demonstrated shapes and remaining uncertainty.
- **Serialization profile**: Normal or verbose field encoding combined with separate or inline ability information.
- **Semantic record**: Validated typed facts, independent of exact retained source bytes.
- **Native run/fight and source generation**: Existing identity, provenance, context, progress and completion contracts extended only where evidence requires it.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All three added profiles have a reviewed, pinned grammar/evidence inventory before implementation begins; synthetic fixtures establish no unsupported producer claim.
- **SC-002**: Single-feed, meaningful byte partitions and checkpoint/resume scenarios agree on completed fights, exact evidence, identities and supported source facts for every supported profile.
- **SC-003**: Existing normal/separate and historical-store comparisons show zero unintended canonical-byte, hash, identity or ordering changes.
- **SC-004**: Automatic catchup/live append and cancellation/failure scenarios preserve atomic progress without manual actions or source-setting changes.
- **SC-005**: Every affected capability and documentation claim names implemented behavior accurately; unsupported measurements remain unavailable.
- **SC-006**: Required local and final-head hosted gates pass, no unresolved review finding remains, and delivery stops for owner review and merge.

## Assumptions

- Base S130 is merged at `747885ced3e7e3423d252eb89434a1b15262b575`; its parser, schema 7 store and independent automatic services are the starting point.
- Both #296 and #297 belong to S131. Missing grammar is a research dependency, not permission to silently narrow scope or invent a format.
- ESO owns source logging and serialization; this desktop only observes existing files and retains local data.
- No release, tag, owner field-testing task or merge is authorized.

## Clarifications

### Session 2026-10-15

- Q: Which profiles belong to S131? A: Normal/inline, verbose/separate and verbose/inline v15, preserving normal/separate.
- Q: What satisfies grammar first? A: Reviewed producer declarations or representative producer evidence with exact supported shapes; synthetic tests alone cannot establish producer fidelity.
- Q: May raw verbose evidence be rewritten to reuse positional parsing? A: No. Preserve raw lines and share validation through a typed semantic view.
- Q: Does format support change player workflows or game settings? A: No. Reuse automatic bounded processing and existing read-only consumers without new manual steps.
- Q: What is the delivery boundary? A: Authorized push/PR, at most two Codex review rounds with all findings resolved, then owner final review and merge.
