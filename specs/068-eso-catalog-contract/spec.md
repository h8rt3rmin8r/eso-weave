# Feature Specification: ESO Catalog Source Contract

**Feature Branch**: `codex/s068-eso-catalog-contract`

**Created**: 2026-09-09

**Status**: Implemented, pending external review

**Input**: Issue #112. Define honest source, coverage, provenance, channel, and
redistribution rules before the catalog compiler or collector chooses an
ingestion path.

## Clarifications

Routine choices were resolved under the build-phase autopilot policy.

- S068 completes the repository-verifiable research and contract work in #112.
- Experiments requiring live and PTS characters, language changes, or game UI
  flush timing move to a dedicated verification issue. Their absence cannot be
  converted into an exhaustive claim.
- API 101050 live and API 101051 PTS remain separate source channels. PTS data
  can become live only when an explicit promotion names the live API version and
  verifies the same source content or replaces it with a new live snapshot.
- ZOS API documentation and immutable stock UI source are primary technical
  evidence. Community repositories are implementation references only unless a
  separate data license clearly covers the imported records.
- ESO Weave may distribute stable identifiers, provenance, virtual icon paths,
  and project-created placeholders. It may not distribute ZeniMax icon bytes or
  an unlicensed community data mirror.
- User-local asset resolution may be implemented later, but acquired bytes stay
  outside distributed artifacts and require a source-specific terms check.

## User Scenarios and Testing

### User Story 1 - Select an honest source per category (Priority: P1)

As a catalog implementer, I want every proposed category to name its durable
identity, discovery method, visibility boundary, and completeness class so the
compiler and collector cannot imply a universal database from bounded evidence.

**Independent Test**: Validate the machine-readable matrix and confirm every
required category has a stable key, source family, visibility, completeness,
failure mode, and validation rule.

**Acceptance Scenarios**:

1. **Given** a category exposed by an API iterator, **When** the matrix records
   it, **Then** durable identity uses returned IDs and never iterator indexes.
2. **Given** a known-ID lookup or observed event, **When** coverage is recorded,
   **Then** it is bounded or opportunistic rather than exhaustive.
3. **Given** a claim that still needs game access, **When** S068 completes,
   **Then** the claim remains explicitly unverified and links to field
   verification rather than being inferred.

### User Story 2 - Reproduce source provenance and channel state (Priority: P1)

As a maintainer, I want immutable source revisions and hashes plus a strict live
and PTS promotion rule so a future catalog build can explain exactly which game
surface produced each record.

**Independent Test**: Validate distinct live and PTS tuples, immutable commit
references, SHA-256 documentation hashes, locale fields, and promotion rules.

**Acceptance Scenarios**:

1. **Given** the live and PTS stock UI branches, **When** evidence is captured,
   **Then** each resolves to a distinct immutable commit and documentation hash.
2. **Given** a PTS snapshot, **When** a compiler considers live output, **Then**
   automatic promotion is forbidden.
3. **Given** an inaccessible upstream attachment or moving branch, **When** an
   offline build runs, **Then** the pinned immutable mirror is the reproducible
   fallback and the moving source is only a freshness signal.

### User Story 3 - Keep graphics optional and redistribution safe (Priority: P1)

As a user and distributor, I want catalogs and interfaces to work with
project-created placeholders while optional local resolution uses only files on
my system, so unavailable art never blocks core data and ESO Weave does not ship
third-party graphics without permission.

**Independent Test**: Validate that icon references and icon bytes have distinct
rows, bytes default to no redistribution, and placeholder/local-only modes remain
available.

**Acceptance Scenarios**:

1. **Given** an API returns a virtual DDS path, **When** the catalog stores it,
   **Then** the path is metadata and no image bytes are implied.
2. **Given** no licensed icon source, **When** a UI needs an image, **Then** a
   project-created placeholder is the distributable fallback.
3. **Given** a future local resolver, **When** it obtains an icon, **Then** the
   bytes stay in a user-local cache and cannot enter release packages or source
   control.

### User Story 4 - Bound SavedVariables ingestion (Priority: P2)

As a collector implementer, I want an explicit snapshot envelope and failure
model so later imports are bounded, private, atomic, and never execute exported
Lua.

**Independent Test**: Review the contract for version/channel/locale metadata,
restricted parsing, size and count limits, corruption rejection, supported save
boundaries, and user-data separation.

**Acceptance Scenarios**:

1. **Given** a collector snapshot, **When** it is imported, **Then** source
   metadata and content hash are required before records are accepted.
2. **Given** malformed, oversized, partial, or executable Lua input, **When** it
   is parsed, **Then** the import fails without replacing the last known-good
   catalog.
3. **Given** character, account, or encounter observations, **When** they are
   retained, **Then** they remain user-owned and separate from distributable
   catalog content.

## Edge Cases

- A moving live branch advances after evidence capture.
- An ESOUI attachment is blocked while its content exists in the pinned stock UI
  repository.
- PTS publishes prototype APIs that do not ship to live.
- An iterator exposes locked entries but localized fields differ by client
  language.
- Known-ID lookups return an empty string for an unknown or retired ID.
- The same effect is emitted under multiple ability IDs or one ID changes
  presentation by caster context.
- A community repository licenses its code but does not license hosted game
  records.
- A local icon path exists but its bytes cannot be decoded or redistributed.
- A SavedVariables write is interrupted, stale, too large, or only flushed after
  logout or `/reloadui`.

## Requirements

### Functional Requirements

- **FR-001**: A version-controlled machine-readable matrix MUST cover player
  skills, crafted abilities, ability metadata, effects, items, item sets,
  champion skills, consumables and mundus effects, identity and combat
  statistics, constants, localization, icon references, and icon bytes.
- **FR-002**: Every category MUST record a durable stable key, enumeration
  method, visibility, completeness, version tuple, provenance, redistribution
  status, failure modes, and validation rules.
- **FR-003**: Iterator positions and mutable indexes MUST NOT be durable keys.
- **FR-004**: Exhaustive MUST be reserved for a source that can enumerate the
  complete named category for its exact version tuple. Unproved scope MUST be
  bounded, opportunistic, or unknown.
- **FR-005**: Live and PTS source channels MUST have distinct version tuples,
  immutable revisions, and SHA-256 evidence hashes.
- **FR-006**: PTS MUST NOT be promoted implicitly. Promotion MUST name the live
  API version, compare source hashes or replace the snapshot, and record reviewer
  approval.
- **FR-007**: The contract MUST distinguish ZeniMax technical source, community
  code, community data, and game art rights.
- **FR-008**: ESO Weave distribution MUST permit project-created placeholders,
  stable identifiers, and virtual icon paths while denying game icon bytes and
  unlicensed community data by default.
- **FR-009**: Noncommercial, educational, no-telemetry project facts MUST be
  recorded as resolving the commercial and data-collection branches only, not
  as a license to redistribute third-party content.
- **FR-010**: A future user-local resolver MAY use user-supplied files or another
  source after source-specific review, but acquired bytes MUST remain local and
  MUST NOT enter builds, releases, fixtures, or source control.
- **FR-011**: Community implementations MAY inform schemas and collection
  patterns only under their code licenses. Their records MUST remain excluded
  until a compatible data license and attribution rule are proven.
- **FR-012**: Installed-client archive extraction MUST remain optional,
  development-only research and MUST NOT be the default updater.
- **FR-013**: SavedVariables imports MUST use a restricted data parser, require a
  versioned envelope and hash, enforce byte and record limits, reject malformed
  or partial snapshots atomically, and never execute Lua.
- **FR-014**: User-owned character and encounter data MUST remain separate from
  redistributable catalog data and MUST NOT be uploaded by default.
- **FR-015**: Game-dependent experiments MUST be tracked separately with exact
  expected outputs. Missing field evidence MUST leave claims unverified rather
  than block placeholder, schema, or compiler work.
- **FR-016**: The canonical documentation MUST explain the supported source
  hierarchy, completeness vocabulary, channel promotion, rights boundary, and
  collector handoff.
- **FR-017**: Automated policy MUST reject missing categories, transient stable
  keys, invalid completeness or redistribution values, implicit PTS promotion,
  and any permission to distribute game icon bytes.
- **FR-018**: S068 MUST NOT implement the SQLite compiler, collector addon, icon
  resolver, update pipeline, user updater, or live/PTS game experiments.

### Key Entities

- **SourceSnapshot**: Immutable evidence for one source, revision, content hash,
  game/API version, channel, locale, and acquisition method.
- **CatalogCategory**: One class of records with durable identity, discovery,
  visibility, completeness, rights, failure, and validation attributes.
- **CoverageClaim**: Exhaustive, bounded, opportunistic, or unknown statement
  scoped to an exact source snapshot.
- **RedistributionDecision**: Allowed, attribution-required, user-generated-only,
  prohibited, or unresolved status for a specific content class.
- **PromotionReceipt**: Explicit reviewed evidence that a PTS snapshot may be
  replaced by or recognized as a named live snapshot.
- **CollectorEnvelope**: Restricted, versioned SavedVariables export metadata
  plus bounded records and a content hash.

## Success Criteria

- **SC-001**: All 15 required category groups pass automated contract validation.
- **SC-002**: The live 101050 and PTS 101051 source snapshots carry immutable
  commits and verified SHA-256 hashes and are never merged implicitly.
- **SC-003**: No matrix row uses a transient index as its durable key or makes an
  unsupported exhaustive claim.
- **SC-004**: Every content class has an explicit redistribution decision, and
  game icon bytes remain prohibited while placeholders remain allowed.
- **SC-005**: Compiler, collector, and icon work can consume the named contract
  without guessing source priority, version shape, completeness, or rights.
- **SC-006**: Game-dependent unknowns are isolated in one verification issue with
  a reproducible experiment matrix.
- **SC-007**: Documentation policy, link checks, UTF-8, punctuation, spelling,
  and mojibake gates pass.

## Assumptions

- API 101050 is live and API 101051 is PTS on 2026-09-09.
- The esoui/esoui repository mirrors the corresponding ZeniMax UI source and API
  documentation for technical reference but supplies no general content license.
- No written permission to redistribute ZeniMax icon bytes has been identified.
- ESO Weave remains individual, open source, educational, noncommercial,
  unsponsored, and free of telemetry.
- A dedicated tracker can hold field experiments without changing the source
  contract delivered by S068.
