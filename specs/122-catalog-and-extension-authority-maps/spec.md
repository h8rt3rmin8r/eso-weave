# Feature Specification: Catalog and Local Extension Authority Maps

**Feature Branch**: `codex/s122-catalog-and-extension-authority-maps`
**Created**: 2026-10-06
**Status**: Specified
**Input**: Owner-authorized S122 bundles #221 and #223, completing Plan048's
remaining figures with spec-kit, autopilot, official PR, green CI, all external
findings and at most two review rounds before owner merge.

## User Scenarios & Testing

### User Story 1 - Understand catalog review and selection (Priority: P1)

A maintainer follows evidence transformations, review gates and explicit runtime
selection without mistaking candidate integrity for trusted origin or promotion.

**Independent Test**: Reconcile each arrow and nearby equivalent with the source,
collector, compiler, candidate and catalog update authorities.

**Acceptance Scenarios**:

1. Given source snapshots or bounded collector evidence, following the lifecycle
   distinguishes normalized bundles and verified candidates from active selection.
2. Given a candidate, selection requires explicit origin acknowledgement and
   installation; the previous verified or bundled generation remains recoverable.
3. Given Live or PTS data, channel separation and local-only collector rights stay
   explicit; PTS preview does not become runtime selection.

### User Story 2 - Understand shared local authorities (Priority: P1)

A client developer sees how HTTP and MCP frame the same authenticated, read-only
authorities within one running loopback service generation.

**Independent Test**: Reconcile both adapters, shared state and query authorities
with the map and retained operations table.

**Acceptance Scenarios**:

1. Given discovery, reading the map distinguishes endpoint/generation location
   from authentication and separately supplied bearer credentials.
2. Given either framing, both paths share canonical player-state and fixed
   catalog/encounter authorities with common bounded query/result semantics.
3. Given an unavailable database or restart, fixed identity remains visible and
   old-generation discovery is not current; neither state authorizes input.

### User Story 3 - Read and maintain both figures (Priority: P2)

Readers use figures offline, without color or scripting, at narrow width and
enlarged scale. Maintainers identify source changes requiring figure updates.

**Independent Test**: Local generated pages, desktop and 320-pixel widths, normal
and expanded images, two themes and 200 percent zoom preserve contained readable
labels and full equivalents. Policy rejects omitted boundaries and inventory drift.

**Acceptance Scenarios**:

1. With script resources unavailable, static figures and full prose remain.
2. At narrow width and 200 percent zoom, expansion keeps labels readable and
   introduces no page overflow.
3. Source authority and update-trigger records name every depicted relationship.

### Edge Cases

- Invalid sources/candidates and failed installation preserve prior selection.
- Local-only collector rights survive review and compilation.
- Discovery is non-secret and cannot authenticate or reveal credentials.
- Unavailable fixed databases remain listed; unknown/stale state is not authority.
- Shared routes converge explicitly without misleading crossings or ordering.

## Requirements

### Functional Requirements

- **FR-001**: One catalog lifecycle at the candidate-pipeline page names immutable
  sources or bounded collector evidence, normalization, verified candidate,
  explicit trust acknowledgement, atomic selection and rollback.
- **FR-002**: Preserve independent Live/PTS, local-only rights and immutable review
  artifacts; candidate integrity does not authenticate download origin.
- **FR-003**: One authority map near the operations table names clients, separate
  discovery, bearer authentication, one loopback generation, both framings,
  canonical player-state, fixed databases and shared bounded query execution.
- **FR-004**: Shared result semantics and limits imply no remote/write access,
  arbitrary paths, credential discovery, agent hosting or gameplay actions.
- **FR-005**: Retain existing authoritative prose and tables with a complete
  adjacent equivalent for each actor, edge, qualifier and failure/rollback path.
- **FR-006**: Owned offline figures use existing expansion, remain readable at
  content width and 200 percent zoom, and carry meaning beyond color.
- **FR-007**: Govern bytes, placement, accessible alternatives, critical labels,
  equivalents and finite inventory with policy and browser coverage.
- **FR-008**: Record source authorities/update triggers and chronologically
  regroup unused S113/#223 and S114/#221 reservations into S122.
- **FR-009**: Preserve application, addon, data, transport and release behavior;
  run automated gates, resolve reviews within the cap, stop before owner merge.
  No installed or live-game verification.

### Key Entities

- Catalog lifecycle: evidence transformations, review and selection/rollback gates.
- Authority map: client, discovery, generation, auth, framing and shared authorities.
- Governed figure: owned image, alternative, equivalent, sources and update triggers.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Both journeys appear once and every relationship has adjacent prose
  and a current source-contract reference.
- **SC-002**: Both load locally and pass normal/expanded rendering at 320 and
  1280 pixels in Navy and Light themes.
- **SC-003**: At 200 percent zoom both source and expanded labels measure at least
  14 effective pixels, stay contained and expose full nearby equivalents.
- **SC-004**: Policy rejects missing assets/equivalents and removed trust, channel
  or read-only qualifiers; no production behavior changes.

## Assumptions

- The owner authorized regrouping; current implementation and prose govern.
- Two figures at their respective authority pages share infrastructure.
- No release, final merge or game-session check is authorized by this slice.

## Clarifications

### Session 2026-10-06

- Q: One or two figures? A: Two, one per issue, with shared coverage.
- Q: Replace operations table? A: Keep it and add relationship prose.
- Q: Runtime contracts? A: Describe current contracts, add no operation or restriction.
- Q: Reservations? A: S113/S114 are historical unused reservations, regrouped into S122.
- Q: Review authority? A: Automatic push/official PR and one additional round;
  owner performs final review and merge.
