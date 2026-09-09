# Catalog Integrity Checklist

**Purpose**: Guard the compiler, catalog, and runtime trust boundary

## Input and Provenance

- [x] Input schema and size limits are checked before database construction.
- [x] Input is data-only and causes no network, script, Lua, or path execution.
- [x] Every fact traces to an immutable source record and snapshot.
- [x] Stable IDs do not depend on iterator positions.
- [x] Coverage claims cannot silently strengthen source evidence.

## Database Construction

- [x] Schema constraints reject invalid channels, kinds, completeness, ranges,
  duplicates, and orphan relationships.
- [x] Construction uses one transaction with deterministic ordering.
- [x] `user_version`, `integrity_check`, and `foreign_key_check` are enforced.
- [x] Semantic and artifact hashes are reported and reproducible.
- [x] Live and PTS destinations cannot collide accidentally.

## Publication and Recovery

- [x] Candidate files are built, synced, closed, and reopened read-only before
  publication.
- [x] Atomic persistence never exposes a partial database.
- [x] The prior catalog and rollback manifest survive replacement.
- [x] Failure leaves the last known-good destination byte-identical.

## Runtime and Distribution

- [x] Runtime connections are read-only and schema-compatible.
- [x] Missing, corrupt, incompatible, or checksum-invalid catalogs degrade to an
  empty service with a visible diagnostic.
- [x] No open handle is replaced in place.
- [x] MSI, Debian, AppImage, and tarball layouts include the approved baseline.
- [x] No prohibited icon bytes or user-collected prose enter distributed files.
