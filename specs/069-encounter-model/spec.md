# Feature Specification: External Encounter Model

**Feature Branch**: `codex/s069-encounter-model`

**Created**: 2026-09-09

**Status**: Draft

**Input**: Issue #113. Define the user-owned encounter plane, its deterministic
metrics, and an evidence-based Combat Metrics parity roadmap without placing
observations in the bundled catalog.

## Clarifications

Routine choices were resolved under the build-phase autopilot policy.

- S069 completes the repository-verifiable design and synthetic spike in #113.
  A same-parse comparison requiring a live ESO client and Combat Metrics moves
  to a separate verification issue and cannot block this contract.
- The synthetic spike proves serialization, ordering, loss handling, unknown-ID
  retention, and deterministic calculations. It is not represented as captured
  game evidence or Combat Metrics parity proof.
- Raw observations belong to a user-owned encounter store. Bundled
  `catalog.sqlite` data remains immutable and independently replaceable.
- The existing Pixel Bus remains a small safety and action-observation surface.
  Bulk encounter telemetry uses a future bounded SavedVariables exporter and
  never expands the screen protocol.
- Personal names, account identifiers, chat, and precise location are absent by
  default. Encounter-local actor IDs are random or derived within one import and
  cannot correlate a person across exports.
- Sequence identity, monotonic time, duplicate rejection, explicit
  discontinuities, and end reasons define order. Wall-clock time is optional
  provenance and never resolves event order.
- Raw accepted events are immutable. Derived metrics are disposable projections
  rebuilt from raw facts and a named calculation version.
- A discontinuity does not delete observed events. It marks affected metrics
  incomplete and prevents high-confidence parity or recommendation claims.
- Combat Metrics and LibCombat are pinned implementation references under the
  Artistic-2.0 license. ESO Weave defines its own schema and calculations and
  copies no implementation code or saved-fight format.

## User Scenarios and Testing

### User Story 1 - Preserve a trustworthy encounter timeline (Priority: P1)

As a future importer, I want every event to have stable encounter-local
identity, deterministic order, and explicit loss markers so duplicate, late, or
missing records cannot silently create false combat results.

**Independent Test**: Validate a synthetic encounter containing combat,
healing, effects, resources, bar changes, death, resurrection, and a declared
sequence gap, then replay it in a different input order and obtain the same
canonical event order and loss state.

**Acceptance Scenarios**:

1. **Given** events with unique sequence numbers, **When** they arrive out of
   order, **Then** canonical order is monotonic sequence followed by the
   contract tie-break rule.
2. **Given** the same session and sequence twice, **When** import validates the
   batch, **Then** the duplicate is rejected rather than counted twice.
3. **Given** a declared discontinuity, **When** calculations run, **Then** raw
   facts remain queryable and affected metrics are marked incomplete.

### User Story 2 - Recompute useful metrics from raw facts (Priority: P1)

As an analyst, I want DPS, HPS, per-ability contribution, effect uptime, and
ordered casts to trace to exact raw observations and named formulas so results
can be rebuilt and audited.

**Independent Test**: Calculate the required metrics from the checked-in dummy
encounter and compare exact results with the checked-in expected projection.

**Acceptance Scenarios**:

1. **Given** accepted damage and healing events, **When** a calculation version
   runs, **Then** totals and rates use the declared duration and inclusion
   rules.
2. **Given** gained and faded effect transitions, **When** uptime is computed,
   **Then** intervals are clipped to the encounter window and never overlap
   twice for the same instance.
3. **Given** a missing interval, **When** deterministic results are emitted,
   **Then** observed totals remain numeric while confidence and coverage record
   the discontinuity.

### User Story 3 - Join catalogs without losing unknown facts (Priority: P1)

As a catalog consumer, I want observations to keep their numeric IDs even when
the attached catalog cannot resolve them so newer catalog versions can enrich
old encounters without rewriting raw history.

**Independent Test**: Attach one catalog snapshot that lacks a fixture ability,
then a newer snapshot that resolves it, and confirm the raw event identity and
hash remain unchanged.

**Acceptance Scenarios**:

1. **Given** an unknown ability ID, **When** import completes, **Then** the event
   is retained with unresolved catalog state.
2. **Given** a newer catalog containing that ID, **When** metadata is projected,
   **Then** the event resolves through a new join receipt without mutation.
3. **Given** a PTS encounter and live catalog, **When** a join is attempted,
   **Then** channel mismatch remains visible and cannot be silently promoted.

### User Story 4 - Protect user ownership and privacy (Priority: P1)

As a user, I want encounter data to remain local, deletable, and free of
personal identifiers by default so analysis never becomes covert telemetry.

**Independent Test**: Validate the contract and fixture against forbidden
identity fields, upload defaults, retention, deletion, backup, and export rules.

**Acceptance Scenarios**:

1. **Given** a default capture, **When** actor records are serialized, **Then**
   names, account IDs, chat, and precise location are absent.
2. **Given** a user deletes an encounter, **When** storage maintenance runs,
   **Then** its raw and derived records are removed without touching the bundled
   catalog.
3. **Given** no explicit export action, **When** an encounter is stored, **Then**
   no network upload or cross-user sharing occurs.

### User Story 5 - Plan evidence-based parity (Priority: P2)

As a maintainer, I want each Combat Metrics-style feature mapped to source
events, deterministic calculations, confidence, privacy impact, and a delivery
phase so implementation order follows evidence rather than feature labels.

**Independent Test**: Validate that every parity row names its evidence,
calculation or observation status, loss sensitivity, privacy class, confidence,
and owning follow-up outcome.

**Acceptance Scenarios**:

1. **Given** a proposed metric, **When** it enters the roadmap, **Then** every
   required raw fact and formula is named.
2. **Given** a feature that depends on unavailable or invasive data, **When** it
   is classified, **Then** it is deferred or unsupported with a reason.
3. **Given** a future recommendation, **When** it cites an encounter, **Then**
   encounter, catalog, calculation, coverage, and confidence versions are all
   required.

## Edge Cases

- Events arrive out of serialization order but retain unique sequence numbers.
- A duplicate event has the same session and sequence but different payload.
- Monotonic time repeats or moves backward across a discontinuity.
- Capture begins or ends while the player is already in combat.
- An effect is refreshed, stacked, faded, or lost across a sequence gap.
- A pet changes owner or an actor is destroyed and its transient game unit ID
  is reused.
- Incoming combat omits source identity while retaining an ability ID or value.
- An ability ID is unknown, retired, aliased, or differs between live and PTS.
- A crash leaves a partial export or a derived projection from an older raw
  revision.
- A fixture includes a forbidden identity field or an export requests upload by
  default.
- An observed correlation is mistaken for a causal build recommendation.

## Requirements

### Functional Requirements

- **FR-001**: A version-controlled machine-readable encounter contract MUST
  define capture envelope, encounter, actor, event, build snapshot, derived
  metric, catalog join, privacy, integrity, retention, and roadmap semantics.
- **FR-002**: Catalog and encounter planes MUST have separate ownership,
  migrations, backup, deletion, and replacement rules.
- **FR-003**: Each batch MUST name schema version, capture implementation,
  addon/API/game version, channel, locale, session ID, sequence range, monotonic
  clock basis, record count, content hash, and discontinuity count.
- **FR-004**: Event identity MUST be `(session_id, sequence)` and MUST NOT use
  arrival position, timestamp, array index, localized name, or transient unit ID
  as durable identity.
- **FR-005**: Canonical ordering MUST use sequence. Duplicate sequence payloads,
  undeclared gaps, backwards monotonic time, and records outside the envelope
  MUST fail validation.
- **FR-006**: Declared gaps and loss markers MUST preserve accepted observations
  while lowering affected metric coverage and confidence.
- **FR-007**: Actor identity MUST be encounter-local. Names, account IDs, chat,
  and precise coordinates MUST be omitted by default and MUST NOT be required
  for any MVP metric.
- **FR-008**: Event kinds MUST cover damage, healing, effects, resources, skill
  or cast timing, bar changes, death, resurrection, boss health, performance,
  quickslots, encounter boundaries, and discontinuities with explicit source
  evidence and required fields.
- **FR-009**: Raw observations MUST be immutable after accepted import. Derived
  projections MUST name their calculation version and be fully rebuildable.
- **FR-010**: DPS, HPS, per-ability contribution, effect uptime, and ordered cast
  sequence MUST each define exact raw inputs, inclusion rules, formulas,
  duration boundaries, loss behavior, and numeric units.
- **FR-011**: Unknown stable IDs MUST remain queryable. Catalog attachment MUST
  use immutable catalog identity and a separate join receipt without mutating
  raw events.
- **FR-012**: Catalog channel or API mismatch MUST remain explicit. PTS evidence
  MUST NOT be relabeled as live through an encounter join.
- **FR-013**: The default policy MUST keep data local, disable upload, reject
  personal fields, permit per-encounter deletion, and keep exports explicit.
- **FR-014**: Imports MUST be bounded, content-hashed, non-executing, atomic,
  idempotent, and recoverable to the last known-good raw state.
- **FR-015**: The existing Pixel Bus MUST remain unchanged and MUST NOT carry
  bulk encounter events. Future capture belongs to a dedicated bounded addon and
  restricted SavedVariables import surface.
- **FR-016**: Telemetry capture MUST remain observational and MUST NOT call input
  hooks, synthesized input, automation controllers, or action authorization.
- **FR-017**: A deterministic synthetic fixture MUST cover every MVP event
  family, a discontinuity, an unknown ID, catalog re-resolution, exact metrics,
  and raw plus compressed storage measurements.
- **FR-018**: Synthetic evidence MUST be labeled synthetic and MUST NOT satisfy
  live ESO or Combat Metrics same-parse verification.
- **FR-019**: A parity matrix MUST map every proposed feature to source facts,
  calculation, confidence, privacy impact, loss sensitivity, support state, and
  owning delivery phase.
- **FR-020**: The roadmap MUST create ordered follow-up outcomes for capture,
  import, calculations, UI, and evidence-scoped recommendations.
- **FR-021**: Any AI or mathematical recommendation MUST remain a later consumer
  and cite encounter, catalog, calculation, coverage, and confidence versions.
- **FR-022**: Canonical documentation MUST explain ownership, ordering, loss,
  privacy, catalog joins, deterministic metrics, and parity boundaries.
- **FR-023**: Automated policy MUST reject missing event families, unsupported
  durable identity, private-by-default regressions, mutable raw facts, ambiguous
  loss behavior, untraceable metrics, and any bulk Pixel Bus transport.
- **FR-024**: S069 MUST NOT implement production telemetry capture, the encounter
  database, the catalog compiler, analysis UI, recommendations, or live-game
  verification.

### Key Entities

- **CaptureEnvelope**: Versioned, hashed metadata and bounds for one atomic event
  batch.
- **Encounter**: User-owned combat interval with start/end reasons, channel,
  coverage, privacy mode, and catalog attachment state.
- **Actor**: Encounter-local participant or pet identity with non-personal role
  and relationship facts.
- **RawEvent**: Immutable ordered observation identified by session and sequence.
- **Discontinuity**: Explicit missing or reset interval that constrains derived
  claims without deleting surrounding facts.
- **BuildSnapshot**: Optional versioned player configuration using stable IDs
  and consent-safe numeric facts.
- **CatalogJoinReceipt**: Immutable record of catalog version, channel match,
  resolution counts, and unresolved IDs for one projection.
- **MetricProjection**: Rebuildable deterministic value with formula version,
  scope, inputs, units, coverage, confidence, and loss sensitivity.
- **ParityRoadmapEntry**: Evidence and delivery classification for one analysis
  feature.

## Success Criteria

- **SC-001**: Every required entity and event family passes automated contract
  validation.
- **SC-002**: Reordering the synthetic input cannot change canonical order or
  any expected metric.
- **SC-003**: The fixture reproduces exact observed DPS, HPS, per-ability share,
  one effect uptime, and ordered casts, with discontinuity-aware coverage.
- **SC-004**: An unknown fixture ability resolves against a newer catalog while
  the raw event hash remains identical.
- **SC-005**: Raw and compressed fixture sizes are measured exactly, and
  projected one-hour and 100-encounter estimates remain explicitly synthetic.
- **SC-006**: No default schema field stores a personal name, account identifier,
  chat text, or precise coordinate, and upload defaults remain false.
- **SC-007**: Every parity row traces to raw evidence or is explicitly deferred
  or unsupported.
- **SC-008**: Live same-parse comparison has a separate Release verification
  owner and does not block S069 completion.
- **SC-009**: Documentation, policy, link, spelling, UTF-8, punctuation, and
  mojibake gates pass with no Rust source changes.

## Assumptions

- The S068 source contract is the authority for live/PTS provenance and catalog
  rights.
- Combat Metrics commit `6ec1deea4ef8801800dfe88ec79b1f94d0d6303b`
  and LibCombat commit `80817e6929c7626832f9b9114d3b12bad8d642c1`
  remain implementation references, not imported code or data.
- Real capture volume, event visibility, and Combat Metrics discrepancies require
  an installed client and stay provisional until separate verification.
- Future storage may use SQLite, but S069 defines logical ownership and
  constraints rather than choosing physical indices or migrations.

## Out of Scope

- Production addon or desktop capture implementation.
- Extending the Pixel Bus payload or using screen pixels for encounter logs.
- A physical encounter database or catalog schema migration.
- Bundling Combat Metrics, LibCombat, their saved fights, or game assets.
- Upload, sharing, leaderboards, telemetry services, or multi-account analysis.
- Live or PTS gameplay automation and any synthesized input.
- Build optimization, rotation coaching, causal claims, or AI advice.
