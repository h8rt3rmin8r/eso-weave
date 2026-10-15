# Implementation Plan: S131 Native v15 Format Coverage

**Branch**: `codex/s131-native-v15-format-coverage` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md)
**Status**: Complete, owner-merged PR #299 at b28d34e on 2026-10-15; Plan057 archived and both issues Done.

## Summary

Complete issue #296 normal/inline and issue #297 verbose/separate plus
verbose/inline using the existing incremental parser and automatic worker.
Pinned static v15 producer serialization establishes the grammar before code.
The official uploader was inspected first; its current parser requires a login
and its public CDN parser is historical. The readable installed producer gives
direct writer and call-site evidence, including its literal version 15 header.
No game execution, setting changes or owner field tests are involved.

## Technical Context

- Rust 1.96.0, edition 2021; current Windows/Linux application targets.
- Existing dependencies only. The inspected Mac binary is evidence about native
  serialization, not a new application platform or distributed dependency.
- Schema 1 batch parsing remains frozen. Schema 2 incremental evidence and
  checkpoint version 1 retain their shapes; SQLite schema 7 needs no migration.
- Preserve all existing bounds: 256 KiB feed, 64 KiB line, 4096 tokens, depth 8,
  8 MiB context, 32 MiB/200000-record fight, 64 MiB canonical evidence and
  128 MiB checkpoint. Account nested tokens before allocating semantic values.
- Producer format flags can change independently without markers. Interpret
  each record locally; never infer one profile from the run header.

## Constitution Check

Pre-research and post-design: PASS. Source files remain read-only. Raw evidence,
source ownership, package/data boundaries, independent maintenance preferences,
authenticated read-only consumers and existing storage transactions remain.
Full spec-kit analysis precedes product edits; tests precede implementation.
Automatic push/PR and at most two Codex rounds are authorized; owner merges.

## Design and Project Structure

1. `src/encounter/native/grammar.rs` provides a bounded semantic record decoder.
   It distinguishes normal comma and verbose TAB records, validates exact ordered
   labels (including repeated flat unit fields), and validates inline ability and
   effect tails before sharing the established positional semantic validators.
   Semantic string quoting is an internal representation only; raw lines never
   change. Unknown verbose payloads remain opaque after validated common fields.
2. `src/encounter/native.rs` selects the semantic decoder only for schema 2 at
   every validation, context, identity and projection seam. Schema 1 uses its
   unchanged lexer and validation. Existing normal/separate schema 2 lines use
   the established compatibility path, including accepted synthetic escapes.
   Added profiles use literal producer strings with no invented escape scheme.
3. `src/encounter/native/incremental.rs` uses that same view during feed and
   checkpoint validation. There is no format state to serialize. Existing
   malformed-known recovery requires a fresh valid BEGIN_LOG and remains bounded.
4. Preserve raw evidence, canonical JSON shape, logical IDs, old unknown counts
   (including trial records), collision behavior and suppression. Recognized
   verbose trial labels are validated, while trials keep historical unknown
   semantic classification. Do not create synthetic standalone metadata records.
5. Current scalar ID arrays remain arrays in all modes. Only event ability
   references expand, and standalone metadata may still coexist with inline.
   Effect nulls and exact optional/branch-specific producer labels are accepted.
6. Signed producer numeric fields retain their actual values. Shared projection
   arithmetic represents signed observations without casting to unsigned or
   zero filling; reduced interpretation qualifies derived rates. Old
   nonnegative values and canonical evidence remain identical.
7. Reuse `src/data_maintenance/native.rs`, `src/encounter/store.rs` and the
   read-only HTTP/MCP services. Tests prove the new profiles through those same
   transactions, restart/cancel boundaries and consumer contracts.
8. Update `docs/src/development/encounter-ingestion.md`, source capabilities,
   guide/API/MCP references and current contracts. Preserve distinctions between
   static producer evidence, synthetic tests, hosted checks and field
   verification. No installed-game verification claim or task is added.

## Evidence and Contracts

- [research.md](research.md): decision index and chronology.
- [contracts/native-v15-grammar.md](contracts/native-v15-grammar.md): pinned
  ordered grammar, compatibility and evidence limits.
- [data-model.md](data-model.md): semantic view and unchanged persisted entities.
- [quickstart.md](quickstart.md): reproducible synthetic and integration checks.
- [tasks.md](tasks.md): test-first story tasks and acceptance mapping.

## Delivery Sequence

1. Finish grammar contract, data model, quickstart, tasks and blocking analysis.
2. Add failing producer-derived synthetic parser/profile tests, then implement
   the bounded semantic decoder and all shared call-site integration.
3. Add and run store/runtime/API compatibility and restart/failure tests.
4. Reconcile docs, pass full Rust/docs/release gates and text hygiene, commit.
5. Push, publish and attach PR, perform at most two Codex review rounds, resolve
   every finding, verify exact final-head hosted checks, stop for owner merge.

## Complexity Tracking

One private grammar module; no new service, dependency, manual workflow, store
migration, checkpoint version or source-setting control.
