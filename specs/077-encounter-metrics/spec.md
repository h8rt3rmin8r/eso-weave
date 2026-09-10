# Feature Specification: Versioned Encounter Metrics

**Feature Branch**: `codex/s077-encounter-metrics`
**Created**: 2026-09-10
**Status**: In progress
**Input**: Issue #134 and the S069 encounter-analysis handoff

## User Scenarios and Testing

### User Story 1 - Rebuild deterministic encounter facts (Priority: P1)

A user selects one immutable imported encounter and a compatible catalog. ESO
Weave calculates observed outgoing DPS, effective HPS, damage share by ability,
effect uptime, and cast order without changing the raw encounter store.

**Independent Test**: Calculate the checked-in ten-second fixture twice. Canonical
projection bytes and all metric values are identical.

**Acceptance Scenarios**:

1. **Given** a ten-second encounter with 3,000 outgoing damage and 800 effective
   healing, **when** it is projected, **then** observed DPS is 300 and effective
   HPS is 80.
2. **Given** equal outgoing damage from abilities 100 and 999999, **when** damage
   share is projected, **then** each share is 0.5 and the unknown ID remains.
3. **Given** overlapping effect intervals whose clipped union covers six seconds,
   **when** uptime is projected, **then** effect 200 uptime is 0.6 rather than a
   double-counted sum.
4. **Given** casts 100, 999999, and 100 in authoritative sequence order, **when**
   projected, **then** the ordered cast sequence preserves that order.

---

### User Story 2 - Keep incomplete evidence visible (Priority: P1)

A user can still inspect observed metrics from a truthfully partial encounter,
but no metric spanning declared loss can look complete.

**Independent Test**: Project a valid partial fixture with one declared missing
sequence range and verify that every encounter-wide metric is degraded and
exposes the exact range.

**Acceptance Scenarios**:

1. **Given** a validated discontinuity, **when** an encounter-wide metric spans
   it, **then** the value is retained, quality is `degraded`, and the declared
   range is included.
2. **Given** duplicate sequences, an undeclared gap, a malformed discontinuity,
   or backward monotonic time, **when** projection is attempted, **then** the
   input is rejected before output publication.

---

### User Story 3 - Reconcile catalog knowledge without raw mutation (Priority: P1)

A user can see which numeric references a specific catalog resolves and can
rebuild the same raw encounter against a later compatible catalog.

**Independent Test**: Project once with a catalog that does not contain 999999,
then with a compatible later catalog that does. The receipt changes from unknown
to known while the raw-content SHA-256 and metric values remain unchanged.

**Acceptance Scenarios**:

1. **Given** a compatible catalog, **when** projection succeeds, **then** its
   receipt records catalog version, semantic SHA-256, channel, API version, raw
   content SHA-256, and sorted known and unknown numeric IDs.
2. **Given** a catalog whose channel or API version differs from the capture,
   **when** projection is attempted, **then** it is rejected without output or
   raw-store mutation.
3. **Given** a later compatible catalog, **when** projection is rebuilt, **then**
   newly known IDs resolve without changing the immutable raw record.

### Edge Cases

- A zero-duration encounter reports rate and uptime values as unavailable rather
  than dividing by zero; cast order and the catalog receipt remain available.
- An encounter with no outgoing damage has an empty damage-share list.
- Healing overflow is subtracted with saturation so malformed arithmetic cannot
  underflow, although hostile import already constrains values.
- Source actor type 1 is the local player. Other source types, including pets,
  are excluded from the v1 outgoing rate and share algorithms.
- Effect events use their recorded duration (`end_ms - begin_ms`) anchored at
  event monotonic time, clipped to encounter bounds, grouped by target and ID,
  then unioned by ID. Invalid or zero-length intervals contribute no uptime.
- Unknown positive numeric IDs remain valid facts. Zero IDs are not catalog
  references and are omitted from the join receipt.
- Output path aliases, link-like targets, corrupt stores, corrupt catalogs, and
  failed output replacement preserve all prior files.

## Requirements

### Functional Requirements

- **FR-001**: The system MUST load exactly one encounter by complete session and
  encounter identity through the validated S076 raw-store boundary.
- **FR-002**: The system MUST calculate projections from immutable raw facts and
  MUST NOT update the raw encounter store or bundled catalog.
- **FR-003**: Every projection MUST name projection schema version 1 and algorithm
  version `s069-v1`.
- **FR-004**: Every metric MUST record its algorithm version, unit, first and last
  source sequence, quality, and all declared loss ranges it spans.
- **FR-005**: Observed DPS MUST equal outgoing local-player damage divided by
  encounter duration in seconds.
- **FR-006**: Effective HPS MUST equal outgoing local-player healing after
  saturated overflow subtraction divided by encounter duration in seconds.
- **FR-007**: Ability damage share MUST divide each positive-ID local-player
  damage total by total local-player damage and sort results by numeric ID.
- **FR-008**: Effect uptime MUST use clipped interval unions so overlap never
  double-counts encounter time.
- **FR-009**: Cast order MUST follow authoritative sequence, never JSON order or
  wall-clock timestamps.
- **FR-010**: Zero-duration rate and uptime values MUST serialize as unavailable;
  empty damage totals MUST produce no shares.
- **FR-011**: Every encounter-wide result spanning declared loss MUST be marked
  `degraded` and include exact sorted loss ranges; otherwise it MUST be `complete`.
- **FR-012**: Projection MUST fail on any raw integrity violation already rejected
  by S076, including duplicate sequence, undeclared gap, backward time, or an
  invalid discontinuity.
- **FR-013**: Projection MUST require a readable schema-compatible catalog whose
  channel and API version exactly match the capture.
- **FR-014**: The join receipt MUST record deterministic catalog and raw identity
  and sorted, deduplicated known and unknown positive numeric references.
- **FR-015**: A numeric effect reference MAY resolve as either an effect or an
  ability; other ability-bearing event kinds resolve as abilities.
- **FR-016**: Rebuilding against a later compatible catalog MUST permit unknown
  IDs to become known without changing raw content identity or metric values.
- **FR-017**: The command MUST atomically publish canonical UTF-8 JSON to an
  explicit output path and print the same projection as a JSON receipt.
- **FR-018**: The output, raw store, and catalog paths MUST be distinct. Link-like
  output targets and unsafe replacement states MUST be rejected.
- **FR-019**: Failed validation, calculation, catalog access, or publication MUST
  leave raw, catalog, and any prior output unchanged.
- **FR-020**: Equal validated inputs MUST produce byte-identical projection JSON
  regardless of invocation time.
- **FR-021**: The feature MUST add no upload, telemetry, scanning, retention,
  recommendation, UI, capture, or gameplay-action behavior.
- **FR-022**: Documentation MUST identify synthetic determinism as repository
  evidence and keep live Combat Metrics parity under issue #131.

### Key Entities

- **EncounterProjection**: Versioned deterministic derived result for one raw
  encounter and one catalog snapshot.
- **MetricResult**: Metric ID, optional numeric value, unit, source range,
  quality, and visible loss ranges.
- **CatalogJoinReceipt**: Raw hash and compatible catalog identity with known and
  unknown numeric reference sets.
- **LossRange**: Exact missing sequence interval and stable reason.

## Success Criteria

- **SC-001**: The canonical fixture produces DPS 300, effective HPS 80, damage
  shares 0.5 and 0.5, effect uptime 0.6, and cast order 100, 999999, 100.
- **SC-002**: Repeated fixture runs produce byte-identical projections.
- **SC-003**: Every declared-loss fixture metric is degraded and exposes the
  exact range; all invalid ordering variants fail before publication.
- **SC-004**: A later catalog resolves 999999 while raw SHA-256 and metrics remain
  identical.
- **SC-005**: Complete Rust CI parity, documentation policy, link, encoding,
  whitespace, forbidden-dash, JSON, and spelling checks pass.

## Clarifications

### Session 2026-09-10

- Q: Where is derived state stored? A: S077 publishes an explicit canonical JSON
  projection outside both SQLite authorities. A derived database remains
  unnecessary until the history UI defines its query and lifecycle needs.
- Q: How is local-player contribution recognized? A: Use recorded combat source
  type 1 (`COMBAT_UNIT_TYPE_PLAYER`). Do not assume actor ID 1.
- Q: How are effect records interpreted? A: Anchor recorded begin/end duration at
  event monotonic time, clip to the encounter, group by target and ID, then union
  by ID. This is deterministic despite omitted absolute clock origin and slot ID.
- Q: Are pets included in v1 outgoing metrics? A: No. Pet ownership attribution
  is not present in raw capture schema v1.
- Q: Is projection time stored? A: No. Adding wall-clock creation time would break
  byte reproducibility.

## Assumptions

- S076 store validation remains the single raw-integrity authority.
- Capture schema v1 encodes `COMBAT_UNIT_TYPE_PLAYER` as source type 1.
- Metrics are descriptive observations, never action authorization.

## Dependencies

- Issue #133 and S076 immutable encounter import (complete).
- Issue #134 owns this slice.
- Issue #131 retains live Combat Metrics parity and tolerance evidence.
- Issue #135 consumes this versioned projection in a later slice.

## Out of Scope

Capture/import changes, actor or pet identity expansion, live parity claims,
retention policy, a derived SQLite schema, history UI, recommendations, uploads,
telemetry, and generated-input behavior.
