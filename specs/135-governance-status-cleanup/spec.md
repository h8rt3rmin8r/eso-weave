# Feature Specification: S135 Governance Status Cleanup

**Feature Branch**: `codex/135-governance-status-cleanup`

**Created**: 2026-10-17

**Status**: Implemented, ready for review

**Input**: Implement the recommended S135 documentation consistency slice for issue #319, reconcile stale governance and lifecycle references, validate and publish a review PR, and stop before merge.

## User Scenarios & Testing

### User Story 1 - Identify historical audits (Priority: P1)

A maintainer reading governance records can distinguish a dated historical issue inventory from the current delivery lifecycle.

**Why this priority**: Old open-issue claims can cause completed work to be scheduled again.

**Independent Test**: Read each scoped audit without external context and identify its date, scope and authoritative source for current state.

**Acceptance Scenarios**:

1. **Given** an audit of issues through #47, **When** it is read now, **Then** its historical date and scope are explicit and it makes no current-backlog claim.
2. **Given** new S135 work, **When** a reader follows the planning record, **Then** the selected issue and active slice are unambiguous.

### User Story 2 - Distinguish closed work from evidence limits (Priority: P1)

A contributor can recognize closed, not-planned verification work while understanding that unperformed installed or game behavior remains unverified.

**Why this priority**: Issue closure does not establish runtime evidence or authorize another verification task.

**Independent Test**: Compare each changed lifecycle reference to its closure reason and retained evidence statement.

**Acceptance Scenarios**:

1. **Given** verification work closed as not planned, **When** its reference is read, **Then** it is not presented as an outstanding task or a passed observation.
2. **Given** an actual historical observation or source contract, **When** current lifecycle wording is reconciled, **Then** the historical evidence and technical contract remain intact.

### Edge Cases

- Evidence is unavailable or ambiguous: retain the last demonstrated claim and identify the limitation.
- A dated snapshot uses old present tense: retain it when its historical scope is clear.
- A machine-readable evidence label differs from issue state: preserve evidence semantics and document the lifecycle distinction.
- A metadata correction touches the constitution: follow its patch-amendment process without changing principles or gates.

## Requirements

### Functional Requirements

- **FR-001**: Historical issue inventories MUST state their audit date and covered issue range.
- **FR-002**: Current delivery guidance MUST point to live issue and Project authorities instead of an undated copied backlog.
- **FR-003**: Each changed lifecycle claim MUST cite observed issue/PR state and closure evidence.
- **FR-004**: Closed not-planned verification MUST remain distinguishable from completed implementation and successful field evidence.
- **FR-005**: Historical results, raw evidence classifications and technical contracts MUST retain their meaning.
- **FR-006**: A bounded audit inventory MUST identify changed references and deliberate historical/evidence retentions.
- **FR-007**: S135 issue, slice, plan, indexes, ledger and changelog MUST agree without assigning a release commitment.
- **FR-008**: Documentation and text checks MUST pass before review handoff; final hosted checks and review MUST be observed on the handoff head.
- **FR-009**: Application behavior, dependencies, release identity, packaging and workflows MUST remain unchanged.

### Key Entities

- **Audit record**: A dated scope, referenced evidence, and disposition of each examined surface.
- **Lifecycle reference**: A statement about work state, separate from what evidence proves.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Every changed lifecycle claim has one traceable source and no unsupported current-work assertion.
- **SC-002**: Every inventoried surface has a correction or explicit retention rationale.
- **SC-003**: Zero unperformed observations are promoted to passed evidence and zero closed verification tasks are reintroduced.
- **SC-004**: One issue-backed review PR passes required checks with all findings resolved, and remains unmerged.

## Assumptions

- The owner authorizes issue creation, commit, branch push and PR publication for this named slice.
- Historical archives and completed feature packets are historical records, not a mutable current backlog.
- The slice is documentation maintenance; a substantive behavior gap discovered during inventory becomes separate issue work.
- No field testing or release operation is part of this slice.

## Clarifications

### Session 2026-10-17

No critical ambiguities remain. Functional scope, entities/lifecycle, reader journeys, quality, external evidence, error handling, constraints, terminology and completion signals are clear. No user questions were necessary; all 16 specification-quality items remain passing. The owner selected documentation consistency, not new product behavior or field testing.
