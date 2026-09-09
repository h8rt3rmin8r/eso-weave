# Feature Specification: Placeholder-First Local Icon Cache

**Feature Branch**: `codex/s072-placeholder-icon-cache`

**Created**: 2026-09-09

**Status**: Draft

**Input**: Issue #116, the S068 graphics and redistribution decision, and the
S070/S071 catalog reference contracts.

## Clarifications

Routine choices were resolved under the build-phase autopilot policy:

- S072 enables only project-created placeholders and a user-supplied local
  directory. Network, community mirror, prebuilt third-party pack, and installed
  archive extraction adapters remain disabled until separately approved.
- Inputs may be static PNG or DDS. Animated PNG is rejected. Both formats are
  decoded under byte, dimension, pixel, and frame limits and are
  deterministically re-encoded as PNG.
- Virtual paths are normalized case-insensitively for lookup while canonical
  spelling remains catalog provenance.
- An immutable cache manifest provides the explicit virtual-path-to-asset
  association missing from the S070 catalog tables. User-local bytes do not
  enter `catalog.sqlite` or a release artifact.
- Cache generation is synchronous library work with bounded inputs. Callers run
  it off the GUI thread; S072 exposes stable states but does not add a new UI.

## User Scenarios & Testing

### User Story 1 - Always receive a usable icon result (Priority: P1)

An application consumer asks for an entity icon and receives either a verified
local PNG or a stable project-created placeholder without losing the entity
name or blocking catalog use.

**Why this priority**: Missing or unlicensed artwork must never make catalog
features unusable.

**Independent Test**: Resolve absent, unsupported, corrupt, and valid local
inputs against a fixture catalog and prove every reference has one typed result
with the same deterministic placeholder for equivalent missing cases.

**Acceptance Scenarios**:

1. **Given** a catalog icon reference with no approved local file, **When** a
   generation is built, **Then** its manifest binds that reference to a
   project-created placeholder PNG and records why fallback was used.
2. **Given** a valid user-supplied image, **When** the referenced path is
   resolved, **Then** the manifest binds the canonical virtual path to a
   verified content-addressed PNG with source and transformed hashes.
3. **Given** a corrupt or unsupported image, **When** resolution runs, **Then**
   the generation succeeds with a placeholder and a non-sensitive failure
   classification.

---

### User Story 2 - Build a safe user-local cache (Priority: P1)

A user points ESO Weave at a directory they control. ESO Weave reads only files
needed by selected catalog references, transforms them within strict limits,
and publishes an immutable cache generation without changing the originals.

**Why this priority**: This delivers useful local graphics without ESO Weave
distributing or remotely acquiring third-party bytes.

**Independent Test**: Build repeated cache generations from synthetic PNG and
DDS files and prove byte stability, containment, deduplication, immutable
publication, path-alias rejection, and preservation of source files.

**Acceptance Scenarios**:

1. **Given** several references that resolve to identical bytes, **When** a
   generation is built, **Then** one transformed object is stored and every
   reference explicitly maps to it.
2. **Given** traversal, absolute, device-prefixed, symlinked, or case-ambiguous
   paths, **When** lookup is attempted, **Then** no file outside the approved
   root is read and the reference falls back safely.
3. **Given** an interrupted or failed generation, **When** an earlier immutable
   generation exists, **Then** that generation remains complete and readable.
4. **Given** identical catalog references and local inputs, **When** generation
   is repeated, **Then** object bytes, manifest bytes, and generation identity
   are identical.

---

### User Story 3 - Audit every asset decision (Priority: P2)

A maintainer can inspect a cache manifest and determine which canonical catalog
reference produced each local object, which transformation ran, why fallback
occurred, and whether bytes may be redistributed.

**Why this priority**: A local-only design still needs traceable source and
transformation evidence.

**Independent Test**: Parse the generated manifest and verify complete mapping,
hashes, dimensions, origin, transformation version, fallback reason, and
redistribution classification without personal paths.

**Acceptance Scenarios**:

1. **Given** a mixed generation, **When** its manifest is inspected, **Then**
   every requested virtual path appears once with an explicit object mapping.
2. **Given** a user-supplied asset, **When** its entry is serialized, **Then**
   it is classified `user-local-only` and contains no source root or account
   path.
3. **Given** a placeholder entry, **When** its entry is serialized, **Then** it
   names the project-owned origin and `allowed` redistribution class.

### Edge Cases

- The catalog contains duplicate paths with different canonical case.
- A source file changes between metadata inspection and read.
- A DDS advertises excessive dimensions, arrays, depth, or mip levels.
- Decoded dimensions overflow a pixel or allocation calculation.
- A PNG contains animation, unexpected color representation, or excessive
  metadata.
- The cache target aliases the source directory or is nested within it.
- A generation directory already exists with matching or mismatching bytes.
- Object or manifest writes fail after some candidate objects are written.
- A reference has no extension or an unsupported extension.

## Requirements

### Functional Requirements

- **FR-001**: The system MUST normalize virtual icon paths to one safe lookup
  key while preserving canonical spelling in provenance.
- **FR-002**: The system MUST reject absolute, traversal, empty-segment,
  device-prefixed, separator-ambiguous, and NUL-bearing virtual paths.
- **FR-003**: The only enabled acquisition adapter MUST read an explicitly
  supplied local directory and MUST request only selected catalog references.
- **FR-004**: Network access, remote mirrors, prebuilt third-party packs, full
  archive crawling, and installed-client extraction MUST remain absent.
- **FR-005**: Source paths MUST remain confined to the approved root after a
  stable no-follow file handle is opened. The handle's resolved path, type, link
  attributes, and bounded read MUST be verified so component swaps, symlink or
  reparse-point races, and concurrent growth cannot escape the root or limit.
- **FR-006**: Source files MUST be bounded to 8 MiB before reading.
- **FR-007**: Decoding MUST accept only static PNG and DDS inputs with one
  two-dimensional image, dimensions from 1 through 1024 on each axis, and at
  most 1,048,576 decoded pixels.
- **FR-008**: The DDS path MUST use a memory-safe decoder with encoding features
  disabled and MUST decode only the base image after header limits pass.
- **FR-009**: Every accepted source MUST be converted to RGBA8 and encoded as a
  deterministic PNG with a versioned transformation identifier.
- **FR-010**: The system MUST generate a deterministic project-owned generic
  placeholder without incorporating third-party bytes.
- **FR-011**: Missing, rejected, corrupt, unsupported, or permission-denied
  source files MUST bind to the placeholder rather than fail the generation.
- **FR-012**: I/O failures in the cache destination MUST fail publication and
  preserve every previously published immutable generation.
- **FR-013**: Objects MUST be addressed by transformed SHA-256 and deduplicated
  within and across identical generations.
- **FR-014**: A canonical manifest MUST explicitly bind every virtual path to
  its transformed object hash and transformation identifier.
- **FR-015**: Each manifest entry MUST record source hash when available,
  dimensions, media type, origin, availability, redistribution, and a typed
  fallback reason when applicable.
- **FR-016**: Manifests and receipts MUST NOT contain the user-supplied root,
  account names, character names, or other personal local paths.
- **FR-017**: Generation identity and serialized output MUST be byte-stable for
  equivalent normalized inputs.
- **FR-018**: Candidate generation content MUST be complete and verified before
  it is published under its immutable generation identity.
- **FR-019**: Existing source files and published generation objects MUST never
  be overwritten or deleted by generation building.
- **FR-020**: The resolver MUST expose `Ready`, `Placeholder`, `Missing`,
  `Unsupported`, and `Failed` states without performing network work.
- **FR-021**: Catalog compilation and runtime access MUST remain valid when all
  icon references are reference-only or placeholders.
- **FR-022**: The cache manifest, rather than filename inference or the unlinked
  S070 `icon_asset` inventory, MUST be authoritative for reference-to-object
  mapping.
- **FR-023**: Fixture assets MUST be project-created synthetic images and MUST
  contain no ZeniMax or community artwork.
- **FR-024**: Documentation MUST explain local ownership, supported formats,
  limits, cache clearing, fallback behavior, and the lack of upload or remote
  acquisition.

### Key Entities

- **Icon Reference**: Canonical virtual texture path obtained from the catalog.
- **Lookup Key**: Case-folded, separator-normalized relative path used only for
  safe matching beneath a user root.
- **Source Asset**: Bounded PNG or DDS file explicitly supplied by the user.
- **Transformed Object**: Deterministic RGBA PNG addressed by SHA-256.
- **Placeholder Object**: Project-created transformed object used for every
  unavailable reference.
- **Manifest Entry**: Explicit binding from one canonical reference to one
  transformed object plus audit metadata.
- **Cache Generation**: Immutable, complete set of objects and canonical
  manifest identified by its manifest hash.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Every valid catalog icon reference receives exactly one manifest
  entry and one resolvable object in a successful generation.
- **SC-002**: Repeated builds from identical synthetic inputs produce identical
  PNG bytes, manifest bytes, and generation identifiers on Windows and Linux.
- **SC-003**: All traversal, alias, symlink, oversize, corrupt, unsupported, and
  decompression-boundary fixtures read zero bytes outside the approved root and
  degrade to a placeholder or fail publication as specified.
- **SC-004**: No source or transformed image above 8 MiB, 1024 by 1024, or
  1,048,576 pixels is admitted.
- **SC-005**: A mixed generation with duplicate content stores exactly one
  object per unique transformed hash.
- **SC-006**: Repository and package scans find no third-party game icon bytes,
  source roots, account identifiers, network icon endpoint, or archive crawler.
- **SC-007**: Existing catalog, collector, input, PixelBeacon, documentation,
  packaging, and cross-platform tests remain green.

## Assumptions

- Users obtain or create source assets independently and choose the supplied
  directory explicitly.
- The initial resolver is a library and compiler-side contract; UI browsing and
  updater orchestration remain in later issues.
- PNG is the cache interchange format because the existing application already
  ships a PNG decoder and egui can consume its decoded RGBA data.
- Placeholder appearance is generic and conveys unavailable artwork rather
  than an ability, item, or status identity.

## Out of Scope

- Downloading icons, calling community mirrors, scraping, or bypassing access
  controls.
- Extracting installed ESO archives or invoking external extraction tools.
- Shipping ZeniMax or community image bytes, localized descriptions, or a
  prebuilt icon pack.
- Automatic garbage collection or deletion of user originals.
- A new application browser, catalog updater modal, or background network job.
- Proving representative live ESO formats; issue #129 retains field evidence.
