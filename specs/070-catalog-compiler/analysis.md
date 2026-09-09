# Analyze Gate: Deterministic SQLite Catalog Compiler

## Pre-implementation result

Status: PASS

The specification, clarification decisions, requirements checklist, integrity
checklist, plan, research, data model, JSON contract, and quickstart agree on the
following boundaries:

1. The existing single Cargo package owns one explicit compiler binary and one
   typed runtime library seam.
2. Semantic content hashing is authoritative across platforms; locked SQLite
   byte identity is separately tested.
3. Every fact is provenance-backed and constrained; Unknown coverage is valid.
4. Candidate construction and publication never mutate the destination in
   place.
5. Runtime access is read-only and gracefully empty on any verification error.
6. Package changes include a dated pinned-artifact decision.
7. Collector, remote discovery, icon acquisition, encounter import, and user
   update orchestration remain assigned to later issues.

## Traceability

- User Story 1 maps to FR-001 through FR-013 and compiler tests.
- User Story 2 maps to FR-008 and FR-013 through FR-015 plus rollback tests.
- User Story 3 maps to FR-018 through FR-020 plus runtime tests.
- User Story 4 maps to FR-016, FR-017, FR-021, and package-policy tests.
- FR-022 and FR-023 are cross-cutting verification and documentation tasks.

## Findings resolved before implementation

- The issue's proposed per-concept table list was architectural guidance, not a
  requirement to freeze empty guessed schemas. The normalized entity core names
  every concept and permits forward specialized migrations.
- A checksum stored in the database cannot include itself. The canonical
  projection explicitly omits only the checksum column while including every
  other semantic field.
- Artifact SHA-256 belongs in the build report and rollback manifest because
  storing it inside the bytes being hashed is recursive.
- The application does not silently search arbitrary current directories. The
  locator uses documented package roots and a debug-only repository path.

No CRITICAL conflict, unresolved clarification, missing acceptance criterion, or
constitution violation remains. Implementation may begin under test-first
discipline.

## Post-implementation result

Status: PASS

The implementation satisfies all four user stories and every acceptance
criterion from issue #114:

1. The strict JSON boundary rejects oversized, executable, malformed,
   provenance-invalid, conflicting, transient-identity, channel-mismatched, and
   coverage-overclaiming inputs before publication.
2. Schema version 1 maps every approved S068 category through constrained
   entities, attributes, relations, aliases, localization, icon metadata,
   coverage, source snapshots, and source records without freezing speculative
   empty concept tables.
3. The compiler builds in one transaction, verifies a closed candidate
   read-only, reports integrity and hashes, preserves rollback evidence, and
   atomically publishes only after the report is durable. A live destination
   rejects a PTS candidate and vice versa.
4. Typed runtime access verifies schema and semantic content, remains read-only,
   stays bound to one opened version, and exposes a visible empty-service
   diagnostic for missing, corrupt, incompatible, or tampered catalogs.
5. MSI, Debian, AppImage, and tarball definitions carry the minimal
   rights-compatible baseline. The baseline contains no entities, localized
   text, or icon bytes and truthfully reports all 15 categories as Unknown.

Repeated builds from normalized inputs produced identical bytes with artifact
SHA-256 `be9015e2b3e1465fb59b279a22e53d5a6d2ecf1196fb9db478ade355b3f8eb46`
and semantic SHA-256
`abfe8da9f08cdd51068ce860e0b0993ed061ba3bf329b57ca4368b11932af4df`.
The locked full Rust suite, strict Clippy, formatting, optimized builds,
documentation tests and policy, mdBook test/build/linkcheck, spelling, package
assertions, JSON parsing, UTF-8 without BOM, LF, forbidden-dash, and mojibake
gates pass.

No CRITICAL conflict, unresolved clarification, acceptance gap, unrelated edit,
or constitution violation remains. The user-owned untracked project-management
document was preserved and excluded from this slice.
