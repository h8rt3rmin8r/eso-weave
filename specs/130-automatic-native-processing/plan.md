# Implementation Plan: S130 Automatic Native Processing

**Branch**: `codex/s130-automatic-native-processing` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md)

**Status**: Implementation complete for owner-approved normal/separate v15 scope after task generation and passing blocking analysis. Final verification and authorized PR/review delivery are recorded in verification.md.

## Summary

Complete #289, #290 and #292-#294 as one dependency-ordered implementation. Reuse S129 application-owned maintenance, preferences and Catalog activation; add incremental native parsing/ingestion and transfer required consumers before retiring all manual presentation. Preserve old canonical bytes and read-only local access.

## Technical Context

- Language: Rust 1.96.0, edition 2021, existing single crate.
- Dependencies: Existing serde, rusqlite, tracing, eframe/egui, authenticated HTTP/MCP services. No new runtime dependency selected.
- Storage: Existing SQLite encounter schemas 1-6. Proposed next schema adds checkpoint/revision, native suppression and rebuildable derived processing. All migrations must share a successful write transaction; no reset of unsupported stores.
- Testing: Rust unit/integration suites, Lua harness, egui/accessibility harness, deterministic capture, Node documentation/trust/brand policies and release-profile embedded guide.
- Platforms: Windows 10/11 x64 and supported Linux x64 Steam/Proton environments.
- Work budgets: Proposed 16 roots, round-robin sources, 256 KiB read turns, bounded parser state/queues and error backoff capped at 60 seconds. Final implementation must declare exact per-record/context/fight/checkpoint/candidate bounds.
- Required producer profile: normal/separate native log version 15. Normal/inline #296 and verbose #297 follow separately.
- Grammar decision: retain reviewed positional v15 vocabulary, including the substantiated optional scribing strings. Unsupported serialization is controlled and never changes game settings. Owner scope approval resolves the prior research dependency.

## Constitution Check

Before research: scope is authorized, source-preserving, outside gameplay ingress, and consistent with one combined addon, independent preferences and retained local reads. Full spec-kit, test-first work, strict text hygiene and exact-head review gates remain mandatory.

Post-design check: PASS. The direct owner revision and linked follow-ups resolve the prior format blocker. Scope preserves source evidence, strict input/package guarantees and independent retained reads. Background ownership precedes UI removal. The blocking analyze gate still precedes implementation. No constitution exception is required.

## Project Structure

- `src/encounter/native.rs` and a focused incremental module: pure feed/checkpoint/source-specific model, tokenizer and validated completion.
- `src/encounter/store.rs`: shared append preflight/insertion, transactional checkpoint and derivation state, compatible old schemas and deletion suppression.
- `src/encounter/history.rs`: retained domain readers and background calculation authority.
- `src/data_maintenance.rs` and `src/data_maintenance/runtime.rs`: bounded independent native scheduling, real source generations and synchronized commit/cancellation admission.
- `src/beacon/windows.rs`, `src/beacon/linux.rs`: deterministic supported-root discovery seams; enumerate supported Linux roots rather than first-match-only behavior.
- `src/main.rs`, `src/app/{mod,ui,strings}.rs`: application service ownership and full presentation retirement after integration.
- `src/player_state.rs`, `src/database_query.rs`, local HTTP/MCP tests: truthful additive service observations, retained fixed databases and authenticated read-only contract.
- `docs/src`, `docs/project`, `.github/scripts/docs-policy*`, capture harness and embedded-guide tests: final current-surface inventory, policy/visual/generated reconciliation.
- `specs/130-automatic-native-processing`: specification, issue acceptance inventory, requirement checklists and research. Design contracts are complete; tasks/analysis execute against the revised scope.

## Proposed Decisions

1. Use bounded yielding polling for appended prefixes, independent from the Catalog compiler, rather than repeatedly importing stable whole files or binding lifetime to UI paint.
2. Reuse actual root identity and immutable environment attribution. Keep missing/quiet files as ordinary waiting states and logging activity unknown without a verified current observation.
3. Keep schema-1 native canonical evidence immutable. A logical identity/alias index must reconcile normalized run-relative evidence without overwriting original identities or hashes; collisions with divergent evidence are explicit conflicts.
4. Persist completed fights and parser progress together with revision compare-and-swap. Acquire a write transaction before a small shared commit/cancellation gate; admission under the gate prevents late writes after a completed stop.
5. Avoid history-wide validation per appended chunk. Keep a validated worker-owned store connection with short cancellable busy retries and per-batch checks; preserve strict compatibility/corrupt-store handling.
6. Keep raw data, immutable Catalog generations and rebuildable derived results separate. Automatically process missing/stale derivation keys in bounded pages, preserving actual source/Catalog/algorithm identity.
7. Transfer required calculations/recommendations out of `src/app/encounter_history.rs` before deleting the viewer module and presentation workers.
8. At implementation setup, archive completed Plan055 with PR #295 merge evidence, establish Plan056, and synchronize indexes/ledgers in the same reviewed slice.

## Delivery Sequence After Research

1. Finish reviewed grammar/identity research and Phase 1 contracts/data model/quickstart.
2. Generate dependency-ordered tasks and acceptance mapping, then run blocking `/speckit-analyze`.
3. Write failing parser/profile/chunk/checkpoint/resource tests, implement parser, then storage/stop/source-generation regressions and ingestion.
4. Integrate background calculations and retained HTTP/MCP reads, then remove manual interfaces and obsolete commands.
5. Complete the whole-corpus current-surface audit, regenerate final captures and validate generated/release-embedded output.
6. Run formatting, all-target/all-feature Clippy, locked full Rust tests and all applicable repository gates before committing buildable changes.
7. Push and publish one official PR, close the five issues only when their full acceptance inventory is satisfied, resolve findings within at most two Codex rounds and verify final-head hosted checks.
8. Stop for owner final review and merge. No release/tag or field-testing task.

## Complexity Tracking

No constitution exception requested. Normal/separate v15 is explicitly owner-approved; broader format claims remain deferred to #296/#297.
