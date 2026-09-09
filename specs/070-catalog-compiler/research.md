# Research: Deterministic SQLite Catalog Compiler

## Decision 1: Rust SQLite binding

**Decision**: Use `rusqlite` 0.40.2 with `default-features = false` and the
`bundled` feature.

**Rationale**: ESO Weave controls the database and supports Windows and Linux.
Bundling avoids missing or old system SQLite libraries and gives both compiler
and runtime one reviewed implementation. Cargo.lock pins the exact transitive
SQLite source used in CI and releases.

**Alternatives considered**:

- System SQLite reduces binary size but makes supported behavior and security
  updates depend on the host package manager.
- A pure Rust SQLite implementation is less mature for the integrity, foreign
  key, read-only, and packaging requirements in this slice.
- SQLCipher adds unrelated key management and native crypto dependencies.

**Evidence**: The rusqlite project recommends `bundled` for applications that
control their own databases and documents that it avoids a missing or stale
system library: <https://github.com/rusqlite/rusqlite>. Version 0.40.2 is the
current crates.io release selected on 2026-09-09.

## Decision 2: Tool placement

**Decision**: Add a dedicated `catalog-compiler` binary target to the existing
Cargo package, with its implementation in the library catalog module.

**Rationale**: The command is explicit and never runs from `build.rs` or
application startup. A workspace-level `xtask` would force a repository
architecture promotion for one tool and duplicate package governance.

**Alternatives considered**:

- `build.rs` was rejected because catalog construction is a reviewed release
  operation, not a side effect of every application build.
- A new workspace member was rejected as disproportionate.
- An external script was rejected because schema and runtime compatibility are
  Rust contracts and should share tested types.

## Decision 3: Schema shape

**Decision**: Use constrained `entity`, `entity_attribute`, `entity_relation`,
and `entity_alias` tables plus dedicated provenance, coverage, localization, and
icon metadata tables.

**Rationale**: S068 proves uneven source visibility and several optional or
observed-only categories. Creating separate empty tables for every proposed
concept would freeze guessed shapes. The entity kind vocabulary still names
abilities, progressions, ranks, morphs, crafted abilities, scripts, effects,
items, variants, equipment/armor/weapon types, qualities, traits, enchantments,
sets, bonuses, combat stats, champion skills, Mundus effects, consumables,
classes, races, and companions.

**Alternatives considered**:

- Dozens of specialized tables provide stricter columns only after actual
  source shapes exist. Before then they encode speculation.
- One unvalidated JSON blob would be flexible but would discard relational and
  provenance guarantees.

## Decision 4: Determinism and checksums

**Decision**: Canonically sort validated inputs, insert in key order, configure
fixed SQLite pragmas, and hash a type-tagged canonical projection of every
semantic table. Also hash final file bytes.

**Rationale**: The semantic projection is independent of SQLite header and page
layout details. The artifact SHA-256 proves the exact packaged bytes. Repeated
CI builds additionally prove byte identity for the locked bundled SQLite
version.

**Alternatives considered**:

- File SHA-256 alone conflates semantics with SQLite implementation details.
- A hash stored outside the database cannot let the runtime verify semantic
  content.
- Hashing a source JSON alone would not detect a database row mutation.

## Decision 5: Atomic publication and rollback

**Decision**: Build a sibling temporary database, commit once, sync and close,
reopen read-only for complete verification, preserve the old bytes under a
content-addressed generation, atomically update its manifest, then use tempfile
atomic persistence for the candidate.

**Rationale**: Candidate work never touches the current destination. SQLite's
rollback journal and full synchronization protect the construction transaction;
filesystem replacement prevents readers from seeing a partial candidate.
Content-addressed rollback generations keep prior recovery evidence valid when
a later copy, verification, or manifest update fails.

**Alternatives considered**:

- Editing the destination in place conflicts with open handles and makes
  interruption recovery depend on a hot journal.
- Delete then rename creates a missing-file window.

**Evidence**: SQLite documents rollback journal flushing and recovery as the
foundation of atomic commit: <https://www.sqlite.org/atomiccommit.html>.

## Decision 6: Runtime and packages

**Decision**: Open with SQLite read-only flags, hold a version-bound connection,
and return an empty typed service with a diagnostic on any failure. Package the
baseline under `catalog/catalog.sqlite` relative to the application root.

**Rationale**: UI code receives typed status and query results, never SQL. A
controlled reopen is the only version swap. The same relative layout works in
the MSI and portable archive; Debian and AppImage expose it at their standard
shared-data root.

## Decision 7: Baseline rights

**Decision**: Generate the repository baseline only from project-authored
synthetic records, Unknown coverage declarations, and placeholder availability
metadata.

**Rationale**: It proves compiler, runtime, and package behavior without
redistributing game graphics, community datasets, user-collected prose, or
claiming unverified completeness.
