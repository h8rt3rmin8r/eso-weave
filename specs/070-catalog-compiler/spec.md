# Feature Specification: Deterministic SQLite Catalog Compiler

**Feature Branch**: `codex/s070-catalog-compiler`

**Created**: 2026-09-09

**Status**: Specced

**Input**: Issue #114 and the S068 catalog source contract reconciled with the
S069 encounter join requirements.

## User Scenarios and Testing

### User Story 1 - Build a trustworthy catalog (Priority: P1)

A maintainer supplies a normalized, versioned input bundle and deliberately
runs a repository-owned command that creates a validated SQLite catalog plus a
machine-readable build report.

**Why this priority**: Every catalog consumer depends on a reproducible,
provenance-rich artifact whose coverage claims are no stronger than its inputs.

**Independent Test**: Build the same fixture twice, inspect both databases and
reports, and prove equal semantic checksums, integrity, foreign keys, schema
version, record counts, and deterministic bytes within the pinned toolchain.

**Acceptance Scenarios**:

1. **Given** a valid live input bundle, **When** the compiler builds it twice,
   **Then** both builds pass all invariants and have identical content hashes.
2. **Given** duplicate identities, broken relationships, invalid coverage, or a
   mismatched source hash, **When** compilation is attempted, **Then** no
   publishable database replaces the last known-good destination and the report
   identifies the rejected invariant.
3. **Given** an input with optional categories absent, **When** it is compiled,
   **Then** the catalog succeeds with explicit Unknown coverage rather than an
   exhaustive claim.

---

### User Story 2 - Review and publish a version safely (Priority: P1)

A maintainer can compare a candidate with the installed or previous catalog,
keep live and PTS outputs separate, and publish only after a complete verified
build.

**Why this priority**: A deterministic artifact is not useful if an interrupted
or channel-confused update can destroy the working catalog.

**Independent Test**: Build changed live and PTS fixtures into separate channel
destinations, inspect the generated diff, simulate a rejected build, and verify
the prior live database and rollback manifest remain valid.

**Acceptance Scenarios**:

1. **Given** an existing valid catalog, **When** a valid candidate is published,
   **Then** the destination changes atomically and the prior bytes are preserved
   as the rollback artifact named by the manifest.
2. **Given** a PTS input, **When** the requested destination or explicit channel
   does not match PTS, **Then** publication is rejected.
3. **Given** old and new catalogs, **When** a diff is requested, **Then** the
   report lists added, removed, and changed entities, text, relations, coverage,
   and icon references in stable order.

---

### User Story 3 - Consume the catalog without risking startup (Priority: P1)

ESO Weave opens a packaged catalog read-only through typed queries. A missing,
corrupt, checksum-invalid, or newer-schema database degrades to an empty catalog
and exposes a visible diagnostic without blocking unrelated features.

**Why this priority**: The artifact must have a safe application boundary before
later encounter, icon, or recommendation work depends on it.

**Independent Test**: Open valid, missing, corrupt, foreign-key-invalid,
checksum-invalid, and incompatible-schema fixtures and verify typed lookups or
the corresponding empty-catalog diagnostic.

**Acceptance Scenarios**:

1. **Given** a compatible packaged catalog, **When** the application opens it,
   **Then** typed release and entity queries succeed through a read-only handle.
2. **Given** an invalid or unsupported catalog, **When** startup probes it,
   **Then** the application continues with no catalog rows and surfaces the
   reason in its user-visible status and warning log.
3. **Given** a catalog is already open, **When** another version is published,
   **Then** the current handle is not replaced in place and a later controlled
   reopen observes the new version.

---

### User Story 4 - Package one predictable baseline (Priority: P2)

Windows and Linux releases carry the same minimal, rights-compatible catalog,
and the runtime locator checks only documented application-owned paths.

**Why this priority**: A compiler without a packaged baseline leaves release
behavior dependent on a developer checkout.

**Independent Test**: Verify the MSI definition, Debian asset list, AppImage
layout, and portable Linux archive all include `catalog/catalog.sqlite`, while
the database contains no third-party art bytes or user-collected localized
prose.

**Acceptance Scenarios**:

1. **Given** each supported package layout, **When** its manifest is inspected,
   **Then** the catalog resides at the documented application data path.
2. **Given** the repository baseline catalog, **When** its contents are audited,
   **Then** it contains only approved normalized facts, virtual icon path
   metadata, and project-created placeholder state.

### Edge Cases

- Empty but valid catalogs retain all required coverage categories with Unknown
  status and a reason.
- Stable numeric IDs do not change when a source iterator position changes.
- Unknown observed ability or effect IDs can exist without a known catalog row
  and can resolve after a later catalog is opened.
- UTF-8 localized text remains byte-valid and search normalization never changes
  durable identity.
- An input timestamp is data and must be explicit; the compiler never reads the
  wall clock into deterministic content.
- Cross-device destinations, read-only directories, full disks, interrupted
  writes, and locked destination files leave the prior catalog usable.
- PTS inputs cannot be relabeled or written over a live artifact.
- Downgrades and schema versions newer than the application supports are
  rejected with a diagnostic.

## Requirements

### Functional Requirements

- **FR-001**: S070 MUST provide a deliberate catalog compiler command outside
  `build.rs`, application startup, and ordinary source discovery.
- **FR-002**: The compiler MUST accept a bounded normalized JSON bundle with an
  exact schema version, release metadata, immutable source snapshots, source
  records, coverage, entities, relations, aliases, localized text, attributes,
  and icon metadata.
- **FR-003**: Input MUST be treated as data only. The compiler MUST NOT execute
  Lua, scripts, expressions, hooks, network requests, or source-provided paths.
- **FR-004**: Every compiled entity or fact MUST trace to at least one validated
  source record and immutable source snapshot.
- **FR-005**: Durable entity identity MUST use a stable source ID plus entity
  kind, channel, and API version. Iterator positions MAY exist only as
  version-scoped attributes.
- **FR-006**: The schema MUST represent the approved S068 categories and the
  S069 ability, effect, item, item-set, class, race, consumable, trait,
  enchantment, champion, Mundus, alias, and relationship join requirements.
- **FR-007**: Coverage MUST use Exhaustive, Bounded, Opportunistic, or Unknown,
  declare limits, and reject unsupported upgrades in completeness.
- **FR-008**: Live and PTS builds MUST remain distinct, and any requested output
  channel MUST match the input channel.
- **FR-009**: Compilation MUST use one SQLite transaction and enforce foreign
  keys, uniqueness, enum, range, and relationship constraints.
- **FR-010**: Every candidate MUST pass `integrity_check`, `foreign_key_check`,
  schema compatibility, semantic checksum verification, and a read-only reopen
  before publication.
- **FR-011**: `user_version` MUST identify the schema, migrations MUST move only
  forward, and unsupported upgrade or downgrade paths MUST fail closed.
- **FR-012**: Identical normalized inputs MUST produce identical semantic hashes
  on all supported platforms and byte-identical databases within the pinned
  SQLite toolchain.
- **FR-013**: The compiler MUST emit a stable JSON report with input hash,
  source-set hash, semantic content hash, artifact hash, validation results,
  counts, min/max stable IDs, and a categorized diff from the prior catalog.
- **FR-014**: Candidate construction MUST use a sibling temporary file, sync and
  close it before verification, preserve the prior valid catalog and rollback
  manifest, and atomically persist the verified candidate.
- **FR-015**: Failed validation or publication MUST leave the last known-good
  destination unchanged and retain an actionable failure report.
- **FR-016**: Localized text MUST record locale and source version. Missing text
  MUST NOT change identity, and distributable fixtures MUST exclude
  user-generated or unlicensed prose.
- **FR-017**: Icon records MUST distinguish virtual references, local assets,
  availability, origin, transformation, attribution, and metadata. No ZeniMax or
  unlicensed third-party image bytes may enter the repository or packages.
- **FR-018**: The application MUST open catalogs read-only, verify compatibility
  and semantic checksum, expose typed release/entity query seams, and make no
  SQL query strings part of UI code.
- **FR-019**: Missing or invalid catalogs MUST return an empty typed service with
  a visible diagnostic and MUST NOT prevent application startup.
- **FR-020**: A live database handle MUST remain bound to its opened file version
  until a controlled reopen. The compiler MUST NOT mutate an open database.
- **FR-021**: Supported Windows and Linux package layouts MUST carry the same
  baseline catalog at predictable application-owned paths.
- **FR-022**: Tests MUST cover deterministic rebuilds, constraint failures,
  optional categories, changed indexes, live/PTS separation, locale changes,
  unknown entities, schema incompatibility, rollback, corrupt input, UTF-8, and
  read-only access.
- **FR-023**: Maintainer documentation MUST explain build, inspect, diff,
  approval, packaging, rollback, and checksum verification workflows.

### Key Entities

- **CatalogRelease**: One immutable catalog identity and its schema, channel,
  game/API versions, locale set, tool version, source-set hash, and semantic
  checksum.
- **SourceSnapshot**: Immutable source revision, hash, provenance, licensing
  scope, channel, API version, locale, acquisition method, and capture time.
- **SourceRecord**: A content-hashed normalized record within one snapshot.
- **Entity**: A stable catalog identity with kind, source numeric ID, channel,
  API version, and observed-only state.
- **EntityAttribute**: A versioned typed fact attached to an entity and source
  record.
- **EntityRelation**: A typed, provenance-backed relation between stable
  entities.
- **EntityAlias**: A traceable alias or supersession between entity versions.
- **LocalizedText**: Locale-specific user-owned or redistributable text with
  source version and optional search normalization.
- **IconReference**: Redistributable virtual path metadata connected to an
  entity.
- **IconAsset**: Local-only or project-created asset metadata without implying
  redistributable third-party bytes.
- **CoverageClaim**: Category, source, channel, locale, class, and declared
  limits for one completeness statement.
- **BuildReport**: Stable evidence for validation, hashes, diagnostics, counts,
  and changes from a prior catalog.
- **CatalogAccess**: Typed read-only application service that is either available
  or empty with a diagnostic.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Three consecutive builds of the valid fixture have equal semantic
  hashes, equal artifact hashes, and byte-identical databases on CI.
- **SC-002**: Every declared invalid fixture fails before publication, and a
  seeded prior destination retains its exact hash.
- **SC-003**: Integrity and foreign-key checks return `ok` and zero violations
  for every published candidate.
- **SC-004**: Every stored fact resolves through a source record to one immutable
  source snapshot, with no orphan rows.
- **SC-005**: Live and PTS fixture builds use different destinations and a
  cross-channel publication attempt fails in 100 percent of tests.
- **SC-006**: Valid, missing, corrupt, checksum-invalid, and newer-schema
  catalogs all produce deterministic typed access outcomes without a startup
  panic.
- **SC-007**: Package and release-policy tests find the baseline catalog in all
  four supported distribution layouts: MSI, Debian, AppImage, and Linux tarball.
- **SC-008**: Full formatting, lint, locked tests, documentation policy, package
  policy, UTF-8, line-ending, punctuation, and mojibake gates pass.

## Clarifications and Assumptions

- S070 uses the existing single Cargo package with a dedicated compiler binary.
  A workspace-level `xtask` would expand architecture without giving this one
  tool a meaningful isolation benefit.
- The schema deliberately uses a normalized typed-entity core rather than one
  mostly empty table per future game concept. Entity kind, typed attributes, and
  relations cover the approved concepts while preserving provenance and stable
  joins. Specialized tables can be added by forward migration when real source
  shapes justify them.
- Bundled SQLite is preferred over a system binding because Windows and Linux
  must share schema behavior and release packaging without an undeclared system
  dependency. The application remains read-only even though the compiler uses
  write support.
- Semantic content hashing is the cross-platform authority. Byte identity is an
  additional tested property within the pinned bundled SQLite version because
  SQLite file bytes are an implementation artifact.
- The baseline is intentionally minimal and rights-compatible. It proves the
  compiler and packaging path without claiming comprehensive ESO data.
- Issue #129 may refine field coverage and limits later but does not weaken the
  S068 contract or block this compiler foundation.
