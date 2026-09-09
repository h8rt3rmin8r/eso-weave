# Feature Specification: Reviewed Catalog Candidate Pipeline

**Feature Branch**: `codex/s073-reviewed-catalog-pipeline`

**Created**: 2026-09-09

**Status**: Ready for planning

**Input**: Issue #117, Plan 038, and the S068 through S072 catalog contracts

## Scope decisions from clarification

- S073 is maintainer tooling. Application notices, active-catalog selection,
  installation, rollback UI, and progress presentation remain issue #118.
- A pipeline run creates an immutable review candidate. It never commits,
  merges, tags, releases, or replaces the accepted application catalog.
- Live and PTS are explicit immutable channels. A larger API version never
  promotes PTS data and no command offers automatic promotion.
- Source acquisition accepts local pinned inputs in every mode. Network
  acquisition is opt-in, HTTPS-only, allowlisted by the checked request, bounded,
  redirect-free, and accepted only after its declared SHA-256 matches.
- The normalized input is either an existing S070 bundle or an S071 collector
  capture imported through the restricted parser. S073 does not invent a new
  stock-UI documentation parser.
- S072 icon resolution is optional and local-only. Candidate bundles record an
  icon receipt and missing-asset counts but never contain user or game image
  bytes. With no local icon source, a project-created placeholder generation is
  still valid.
- Scheduled automation may build and upload review candidates from explicitly
  configured inputs. Missing secrets or unavailable sources yield a report, not
  a mutation or silent stale-data promotion.

## User Scenarios & Testing

### User Story 1 - Build a reproducible offline candidate (Priority: P1)

A maintainer supplies a checked pipeline request and pinned local inputs, then
receives one complete immutable review candidate containing a verified catalog,
canonical manifest, checksums, source inventory, validation report, and semantic
diff without network access.

**Why this priority**: Offline fixture mode is the deterministic foundation for
every other mode and for Windows and Linux CI.

**Independent Test**: Run the pipeline twice from the same invented fixtures in
separate roots and compare the canonical manifest identity and every semantic
report.

**Acceptance Scenarios**:

1. **Given** a valid offline Live request and pinned normalized bundle, **When**
   the pipeline runs, **Then** it publishes one verified Live candidate under an
   immutable manifest identity.
2. **Given** identical inputs in another workspace, **When** the pipeline runs,
   **Then** candidate manifest, catalog semantic hash, validation, source
   inventory, and diff are identical.
3. **Given** an invalid or interrupted input, **When** any stage fails, **Then**
   no candidate becomes visible and every prior candidate remains unchanged.

---

### User Story 2 - Keep Live and PTS structurally separate (Priority: P1)

A maintainer builds Live and PTS candidates without either channel being
misnamed, compared against an incompatible baseline, or promoted into the
other channel.

**Why this priority**: Channel confusion could ship preview data as current
Live data and invalidate every downstream catalog claim.

**Independent Test**: Build invented Live and PTS requests, attempt every
cross-channel input and baseline combination, and verify rejection before
candidate publication.

**Acceptance Scenarios**:

1. **Given** a PTS request, **When** its bundle or collector capture declares
   Live, **Then** the run fails before catalog construction.
2. **Given** a Live request and a PTS baseline, **When** a diff is requested,
   **Then** the pipeline rejects the comparison rather than reporting a Live
   update.
3. **Given** a PTS API version larger than Live, **When** the candidate is built,
   **Then** it remains named and recorded as PTS with no promotion state.

---

### User Story 3 - Acquire and review approved source snapshots (Priority: P2)

A maintainer can use explicit local source files or opt into bounded approved
HTTPS retrieval, with every byte tied to an immutable revision, declared hash,
channel, license scope, and acquisition result.

**Why this priority**: Reproducibility and source-policy review depend on exact
inputs rather than mutable URLs or an unexplained cache.

**Independent Test**: Exercise a mock source fetcher with correct, oversized,
mismatched, redirected, unapproved-host, unavailable, and stale-cache cases.

**Acceptance Scenarios**:

1. **Given** a matching local source, **When** acquisition runs, **Then** the
   verified content-addressed cache entry is reused and recorded without its
   local path.
2. **Given** the same pinned remote bytes with a cold or warm cache, **When**
   candidates are built without refresh, **Then** both runs retain the same
   stable acquisition provenance and candidate identity.
3. **Given** opt-in HTTPS acquisition, **When** the response exceeds its limit,
   redirects, comes from an unapproved host, or fails its hash, **Then** it is
   rejected and no cache entry or candidate is published.
4. **Given** a network failure and an exact previously verified cache entry,
   **When** stale-cache reuse was explicitly allowed, **Then** the candidate
   records that reuse in both the source inventory and validation findings;
   otherwise the run fails.

---

### User Story 4 - Compose collector, compiler, diff, and icons (Priority: P2)

A maintainer can process an approved collector capture or normalized bundle
through one ordered command, then inspect a complete review bundle before any
separate publication decision.

**Why this priority**: S073 exists to compose the independent foundations from
S070 through S072 without weakening their boundaries.

**Independent Test**: Run invented collector and bundle requests through all
stages, inspect the candidate manifest, and verify that no user path, capture,
localized source text, or user icon byte enters the review candidate.

**Acceptance Scenarios**:

1. **Given** an S071 capture, **When** the capture mode runs, **Then** the strict
   importer produces the normalized input before S070 builds the catalog.
2. **Given** an accepted baseline, **When** the new catalog differs, **Then** the
   review bundle reports stable additions, removals, changes, coverage deltas,
   and icon-reference deltas.
3. **Given** no local icon directory, **When** references exist, **Then** S072
   produces placeholder mappings and catalog success is not blocked.
4. **Given** a local icon directory, **When** icons resolve, **Then** the local
   cache remains outside the candidate and only a redacted receipt is recorded.

---

### User Story 5 - Run least-privilege review automation (Priority: P3)

A maintainer can manually dispatch or schedule a candidate build whose workflow
has read-only repository permissions and uploads only the review-safe bundle.

**Why this priority**: Automation makes update detection repeatable, but it must
remain downstream of the deterministic local pipeline and human acceptance.

**Independent Test**: Validate workflow permissions and triggers, run the
offline fixture path, inspect the artifact allowlist, and prove no write,
release, or commit step exists.

**Acceptance Scenarios**:

1. **Given** a scheduled or manual run, **When** it succeeds, **Then** it uploads
   only the immutable review-safe candidate and cannot alter repository or
   release state.
2. **Given** a failed stage, **When** the workflow ends, **Then** its diagnostic
   identifies the stage and no partially trusted candidate is uploaded.

### Edge Cases

- A request, source, capture, normalized bundle, baseline, cache, and output
  path alias one another through symlinks, reparse points, case, or nesting.
- A source grows after metadata inspection or is replaced while being read.
- The source hash exists in cache but its content has been changed.
- The request declares duplicate source IDs, hashes, artifact names, or
  conflicting version tuples.
- The bundle release tuple disagrees with the request or source inventory.
- The baseline is corrupt, uses a newer schema, or belongs to another channel.
- The semantic diff contains suspicious removals, coverage regression, schema
  incompatibility, license-scope changes, or configured count threshold jumps.
- A candidate identity already exists with matching or conflicting bytes.
- A crash occurs after building one artifact but before immutable publication.
- Network input omits a content length, uses chunked transfer, redirects, stalls,
  exceeds the cap during streaming, or returns a non-success status.
- Local icon resolution fails, contains only placeholders, or produces
  user-local objects that are forbidden from the uploaded review bundle.

## Requirements

### Functional Requirements

- **FR-001**: The pipeline MUST accept explicit `live`, `pts`, `offline`, and
  `user-capture` modes through one versioned, strict request contract.
- **FR-002**: Every request MUST declare channel, game version, API version,
  catalog version, schema compatibility, locale set, tool version, source pins,
  baseline policy, icon policy, and output roots.
- **FR-003**: Request and generated manifests MUST reject unknown fields,
  duplicate identities, unsafe strings, invalid hashes, and inconsistent
  version tuples.
- **FR-004**: Live and PTS channel identity MUST remain explicit and immutable
  through acquisition, import, build, diff, filenames, manifests, and reports.
- **FR-005**: No code path may automatically promote PTS, select it as Live, or
  infer channel from numeric version ordering.
- **FR-006**: Local inputs MUST be regular, bounded files read from stable
  no-follow handles confined to an approved request root.
- **FR-007**: Optional network acquisition MUST require an explicit flag, HTTPS,
  an allowlisted immutable host and revision path, bounded redirect-free reads,
  timeouts, and exact SHA-256 verification before caching.
- **FR-008**: Source cache entries MUST be content-addressed, immutable,
  verified on reuse, staged until the candidate is installed, and published
  without replacing existing content only after installation succeeds.
- **FR-009**: Stale source reuse MUST require an explicit per-request decision
  and MUST appear in the source inventory and validation summary.
- **FR-010**: User-capture mode MUST call the existing S071 restricted importer
  and MUST never execute Lua or publish raw capture bytes.
- **FR-011**: Normalized bundle modes MUST call the existing S070 validation and
  compiler without weakening its byte, record, string, relationship, channel,
  integrity, foreign-key, deterministic, or atomic-publication checks.
- **FR-012**: The pipeline MUST verify the built catalog after construction and
  record both semantic and exact artifact hashes.
- **FR-013**: A baseline diff MUST use a verified same-channel catalog. An absent
  baseline MUST produce an explicit initial-candidate state.
- **FR-014**: The structured diff MUST cover entities, localized text,
  relations, coverage, and icon references and MUST retain added, removed, and
  changed stable keys. Coverage changes MUST identify completeness downgrades
  as regressions that consume the configured coverage-removal threshold.
- **FR-015**: Configured schema incompatibility, integrity failure, suspicious
  removals, coverage regression, source-rights change, or count threshold breach
  MUST block candidate publication with a stage-specific finding. Any changed
  localized-text redistribution class is a blocking source-rights change.
- **FR-016**: Optional S072 resolution MUST operate only on an explicit local
  source root, stage its generation until candidate installation succeeds, keep
  its cache outside the review candidate, and degrade missing images to the
  project placeholder.
- **FR-017**: The review candidate MUST contain only an allowlisted catalog,
  canonical manifest, build report, verification report, semantic diff, source
  inventory, validation report, checksums, and redacted icon receipt.
- **FR-018**: Candidate artifacts and reports MUST contain no capture bytes,
  source bytes, user-local image bytes, account or character identifiers, local
  filesystem paths, credentials, or unapproved localized records.
- **FR-019**: A complete candidate MUST be constructed and verified in a
  temporary directory, then published without replacement under the SHA-256 of
  its canonical manifest.
- **FR-020**: A failed or cancelled run MUST leave accepted catalogs, source
  cache entries, icon generations, and prior candidates unchanged.
- **FR-021**: Repeating equivalent input MUST produce the same candidate
  manifest identity and semantic reports on Windows and Linux.
- **FR-022**: Manual and scheduled automation MUST use least-privilege read-only
  repository permissions, pinned actions, bounded timeouts, and an artifact
  allowlist.
- **FR-023**: Automation MUST NOT commit, push, merge, tag, create releases,
  update tracked manifests, or accept candidates.
- **FR-024**: Every stage MUST report a stable name, status, and actionable
  failure without printing sensitive local paths or source content.
- **FR-025**: Maintainer documentation MUST cover all modes, request authoring,
  source acceptance, candidate inspection, explicit acceptance outside S073,
  rejection, cleanup, rollback, and reproduction.
- **FR-026**: Application startup, application configuration, active catalog
  selection, and user-facing update UI MUST remain unchanged in S073.

### Key Entities

- **Pipeline Request**: Strict maintainer input defining mode, version tuple,
  paths, source pins, policies, and thresholds.
- **Version Tuple**: Channel, game version, API version, catalog version, schema
  version, locale set, and tool version used consistently by every stage.
- **Source Pin**: Immutable source identity, URI or local file, revision,
  expected hash, byte cap, license scope, acquisition rule, and cache result.
- **Source Inventory**: Canonical redacted account of every required source and
  how its verified bytes were obtained.
- **Candidate Manifest**: Canonical allowlist of candidate artifacts and their
  hashes plus the complete version and policy identity.
- **Validation Finding**: Stable code, severity, stage, message, and blocking
  state without sensitive payload data.
- **Review Candidate**: Immutable directory named by manifest hash and containing
  only review-safe artifacts.
- **Icon Receipt**: Redacted S072 generation identity and ready/placeholder
  counts, never image bytes or local roots.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Valid invented Live, PTS, offline, and collector-capture requests
  complete on Windows and Linux with 100 percent of required artifacts verified.
- **SC-002**: Repeated equivalent runs produce identical candidate manifest
  SHA-256, catalog semantic SHA-256, source inventory, validation, and diff.
- **SC-003**: Every cross-channel, corrupt, oversized, aliased, redirected,
  mismatched-hash, and interrupted fixture fails before immutable publication.
- **SC-004**: No candidate or workflow artifact contains any source bytes,
  capture bytes, user-local icon bytes, personal identifiers, credentials, or
  absolute local paths.
- **SC-005**: All suspicious configured changes produce a stable blocking
  finding, while ordinary additions and same-channel changes remain reviewable.
- **SC-006**: Scheduled automation has zero repository or release write
  permissions and zero automatic acceptance or promotion paths.
- **SC-007**: A failed run changes zero bytes in every previously published
  candidate and accepted catalog.

## Assumptions

- S070 remains authoritative for catalog validation, deterministic SQLite
  construction, verification, and semantic diffing.
- S071 remains authoritative for collector lifecycle and restricted capture
  parsing.
- S072 remains authoritative for local icon transformation and placeholders.
- Current approved remote technical sources are immutable GitHub content paths;
  any new host or mutable endpoint requires a reviewed source-policy change.
- A later acceptance operation may copy a reviewed candidate into tracked or
  release state, but that authority is intentionally absent here.

## Out of Scope

- Application update UI, background startup downloads, active catalog install,
  selection, rollback, or migration.
- New game addon categories, encounter capture, or live game automation.
- General web scraping, archive extraction, mutable feeds, remote icon
  acquisition, or distribution of third-party image bytes.
- Automatic pull requests, commits, releases, PTS promotion, or catalog
  acceptance.
