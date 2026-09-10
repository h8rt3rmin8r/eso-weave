# S076 Spec-kit Analysis

## Pre-implementation Gate

Result: PASS after the resolutions recorded below.

The specification, clarification decisions, requirements checklist, import
safety checklist, research, data model, contract, plan, quickstart, and tasks
form one bounded implementation slice for issue #133.

## Coverage Matrix

| Concern | Requirement authority | Planned evidence |
| --- | --- | --- |
| Explicit hostile import | FR-001 through FR-006 | Restricted parser and bounded-read tests |
| Terminal truth | FR-007 through FR-009 | Complete, partial, gap, count, time, and terminal fixture matrix |
| Provenance and canonical identity | FR-010 through FR-012 | Unknown-ID, ordering, canonical hash, and channel tests |
| Dedicated immutable store | FR-013 through FR-018 | Schema, transaction, duplicate, collision, trigger, corruption, and version tests |
| User lifecycle ownership | FR-019 through FR-022 | List, delete, snapshot, replacement, and receipt tests |
| Privacy and usable surface | FR-023, FR-024 | Payload allowlist, sensitive-string, and CLI tests |
| Scope boundary | FR-025 | Source inventory and documentation checks |

## Findings and Resolutions

1. The existing collector parser is security-sensitive and solves the same fixed
   SavedVariables grammar. The plan extracts it behind a configurable shared
   boundary and preserves collector behavior through adapter regression tests
   rather than duplicating the implementation.
2. An empty Lua table is ambiguous between an array and an object. The shared
   parser receives an explicit empty-table policy: collector capture uses its
   existing array behavior, while encounter capture uses object behavior for an
   empty warning map. Non-empty table shapes remain inferred and mixed or sparse
   keys remain invalid.
3. The S075 JSON schema permits arbitrary scalar payload maps, which is too broad
   for hostile input and privacy. The contract derives a finite key allowlist and
   stable string tokens from the production emitter and rejects additions. Fields
   emitted by the current addon but omitted from the normalized S075 fixture are
   optional within that allowlist.
4. The S069 model's provisional 500,000-event import ceiling predates S075's
   accepted 100,000-event schema-v1 ceiling. S076 follows the current producer
   contract and records the distinction instead of accepting impossible current
   captures.
5. "Recover from partial input" could incorrectly imply accepting truncated
   files. The specification distinguishes a truthful terminal partial capture,
   which is valid evidence, from syntactically or semantically partial input,
   which is rejected without mutation.
6. A source-byte hash cannot provide semantic idempotency because SavedVariables
   whitespace and table order may vary. The design uses canonical content hash
   for raw identity and retains a separate source hash for the import attempt.
7. Normalizing every event into SQL rows now would couple raw storage to the next
   slice's metrics. The immutable canonical BLOB is the raw authority; #134 can
   create rebuildable derived projections later.
8. Copying an open SQLite file is not a consistent backup. The existing rusqlite
   dependency will enable its backup feature, create a temporary snapshot, close
   and verify it, then use the existing atomic publication primitive.
9. Store corruption fallback semantics differ from configuration fallback.
   User-owned raw data must remain untouched, so corrupt and unsupported stores
   return typed errors and are never renamed, recreated, or downgraded.
10. The repository has no public encounter UI yet. Reusable library operations
    plus explicit maintainer CLI commands make the slice verifiable without
    pulling #135 into scope.

## Constitution Gate

- Full spec-kit artifacts exist before implementation.
- The encounter bridge remains local, read-only, explicit, bounded, and separate
  from input, automation, PixelBeacon, and collector promotion.
- Raw observations are stored outside configuration and the catalog.
- Test-first tasks precede parser, validation, store, and CLI implementation.
- A full Cargo merge gate is mandatory because Rust and Cargo metadata change.
- No pinned workflow, script, packaging, release, ignore, license, or toolchain
  artifact is planned.
- Text and encoding requirements are explicit verification tasks.

No unresolved critical conflict, ambiguous authority, hidden network behavior,
unsafe recovery promise, impossible durability claim, privacy gap, or template
token remains. Implementation may begin.

## Post-implementation Gate

Result: PASS.

The shared parser preserves the collector's existing capture behavior while the
encounter adapter applies its own root, empty-warning-map interpretation, and
larger bounded work budget. Typed validation accepts the normalized S075
complete and truthful partial contracts, including all fourteen event families
and unknown numeric IDs, while rejecting executable syntax, schema additions,
unsafe strings, channel mismatch, ordering errors, undeclared or malformed loss,
and inconsistent terminal facts.

Encounter store schema v1 keeps canonical raw bytes outside configuration and
`catalog.sqlite`. Tests establish deterministic canonicalization, source and
content hashes, exact duplicate idempotency, semantic collision preservation,
transactional rollback, update rejection, deterministic metadata listing,
explicit delete-one and delete-all, consistent backup replacement, backup hash
verification, raw hash mismatch rejection, and preservation of corrupt,
unrelated, and future-version databases. File-boundary tests cover byte limits,
path aliases, link-like input and store paths where the platform permits them,
and no store creation for rejected input. The production test canonicalizes the
full 100,000-event S075 ceiling.

The library and non-interactive maintainer CLI expose import, list, backup, and
deletion without payload output, scanning, upload, telemetry, catalog mutation,
metric derivation, or gameplay authority. The final formatting, strict Clippy,
complete locked Rust suite, optimized binary builds, mdBook tests and build,
link checking, documentation policy, spelling, JSON, whitespace, UTF-8 without
BOM, forbidden-dash, and mojibake gates pass. An initial run observed one
pre-existing Windows documentation-server timing flake; its isolated rerun and
the required subsequent complete locked suite both passed.

No unresolved critical conflict, privacy leak, store-plane violation,
unbounded parser path, unsafe recovery behavior, unsupported completion claim,
or stale template token remains.
