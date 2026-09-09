# Collector Safety Checklist: S071

**Purpose**: Gate the game-addon boundary, hostile import, and data provenance

**Created**: 2026-09-09

## Addon Isolation

- [x] Collector folder, manifest, saved variable, version, and marker are unique
- [x] PixelBeacon source, files, protocol, settings, and lifecycle are excluded
- [x] No synthesized input, equipment mutation, item consumption, process
  memory access, packet interception, or network upload is permitted
- [x] Collection requires an explicit command and cannot run in combat

## Bounded Collection

- [x] Approved iterator categories and coverage claims are enumerated
- [x] Per-tick record and elapsed-time budgets are required
- [x] Snapshot, record, string, chunk, and checkpoint limits are required
- [x] Pause, resume, cancellation, truncation, and API errors remain explicit
- [x] Partial exports cannot be accepted as complete captures

## Hostile Input

- [x] Lua evaluation is forbidden
- [x] The accepted grammar, root, scalar types, table shapes, and nesting are fixed
- [x] Functions, expressions, references, metatables, duplicate keys, and extra
  trailing values are rejected
- [x] All allocation-relevant limits are enforced during parsing
- [x] Chunk sequence, counts, bytes, checksum, and completion are revalidated

## Publication and Privacy

- [x] Import stops at deterministic compiler staging
- [x] Existing staging output survives every validation or write failure
- [x] Live and PTS remain distinct
- [x] Localized text and capture scope remain user-local
- [x] Receipts redact personal names and sensitive local paths
- [x] No third-party art bytes enter source control, fixtures, packages, or output

## Verification

- [x] Live, PTS, multi-class, incomplete, corrupt, executable, and boundary
  fixtures are required
- [x] Staged bundles must pass the S070 validator and deterministic compiler
- [x] Lifecycle tests must prove marker gating, confinement, and PixelBeacon
  byte preservation
- [x] Documentation must explain save boundaries and removal
