# Implementation Plan: Completed Native Encounter Log Import

**Branch**: codex/s127-native-completed-import | **Date**: 2026-10-13
**Spec**: [spec.md](spec.md) | **Issue**: #272 | **Base**: 9bfe9e07644d4611cd29bec3293e33457e4c4d69

## Summary

Import explicit completed native files into the existing local Encounter History,
with separate native canonical evidence and projections. Support the researched
normal/separate version15 profile, truthful unknown/incomplete reporting and
source-aware history, while preserving all addon formats and replay behavior.

## Technical Context

- Language/version: Rust 1.96.0, edition2021, one existing desktop crate.
- Dependencies: existing serde/serde_json, sha2, rusqlite0.40.2, tempfile and eframe0.36.
  No new dependency or configuration schema is required.
- Storage: schema6 adds immutable native run and encounter tables; schemas1-5
  remain readable and migrate inside each write transaction.
- Tests: Rust fixtures, real-derived sanitized native records, synthetic hostile
  input, mixed-source transactions, serialized workers, egui sizing and docs policy.
- Platforms: repository Windows/Linux backends and CI; installed ESO/Proton behavior is unverified.
- Performance: stable bounded reading on the existing serialized history worker;
  parse lines without collecting token trees for the whole file.
- Limits: file256MiB, 2000000 records, line64KiB, nesting8, 4096 fields/tokens per
  record, 100000 metadata entries, 10000 runs and completed combats, 200000 records and
  32MiB original lines per combat, 8MiB retained context per combat, 512MiB
  retained batch evidence. All sums and time arithmetic are checked.
- Scale: observed owner file43518907 bytes /298596 records /205 completed combats.
  It exceeds the addon32MiB source limit; native bounds are independent.
- Unknowns: native grammar, source identity/storage and UI integration researched
  by read-only agents and resolved in research.md; no unresolved clarification.

## Constitution Check

Pre-research and post-design gates pass:
1. S127/#272 and canonical native-ingestion docs establish scope; all spec-kit stages precede code.
2. Native import is a local read-only file operation; no addon activation, gameplay action,
   input transport, raw diagnostic disclosure or source mutation is introduced.
3. Red regressions precede parser/store/worker/UI behavior; old safety tests remain.
4. Full foreground fmt/Clippy/locked tests precede Rust commits.
5. Desktop-only native file handling adds no in-game surface or transport. Addon
   capture remains exactly single/continuous and its formats are unchanged.
6. UTF-8/no BOM/LF/no long dashes, chronological plans and dated decisions remain required.
7. Automatic push/official PR are authorized; two review rounds maximum; owner merge
   is the boundary. No tag, release or owner field-verification request.

## Design and decisions

- Separate native models rather than constructing fake EncounterCapture callback data.
- Raw native records preserve original lines and source ordinals; metadata context
  needed by each fight is retained in original order and scoped to its run.
- Header identity includes actual header values, run ordinal and explicitly selected
  environment, never path/mtime. Combat identity combines run identity and native
  combat ordinal. Canonical content hash remains separate from source-file hash.
  Identical copied files and later appended runs preserve existing identities.
- Native API/addon versions remain absent. Realm/language/game strings are retained
  locally but default summaries expose controlled format/source facts.
- Completed boundary pairs are eligible; nested/orphan or reversed boundaries reject.
  Unclosed fights/runs and unterminated tails are counted, never fabricated.
  Unknown records are retained when relevant, counted separately from out-of-combat
  observations, and reduce capability claims. Malformed recognized records reject
  the entire source before store mutation.
- Native metrics expose paired duration, ordered record/cast/effect counts and
  reviewed observed combat hit/overflow quantities with explicit actor scope.
  Missing actor relation, interval reconstruction, native semantic evidence or
  callback-only data yields an unavailable reason. No addon replay, catalog API
  binding, uptime, recommendation or callback parity is invented.
- History summaries add explicit source kind and optional addon capture mode;
  native detail dispatch occurs before catalog gates. Addon projection stays intact.
- Schema6 extends immutable session ordering to native runs. Native batch preflight,
  migration and insert share one immediate transaction. Deletes affect the intended
  source; Delete All includes both; backup validates both; ordering survives reimport.
- Native source choice and path are transient UI state using the existing path-field
  pattern, with Inspect/Import actions. Source keys include kind/path/environment
  and generation. Addon inspection in Data Details remains independent.
- Update approved native documentation/diagram/policy expectations with negative
  fixtures preserving source isolation, raw-data boundaries and explicit no-live-tail claims.
  Frozen historical content-coverage records are not rewritten.
- Archive Plan052 with PR275 squash-merge evidence and create active Plan053.

## Project Structure

- src/encounter/native.rs: models, bounded tokenizer, supported layout validation,
  context/grouping, canonical validation, source read and native projection.
- src/encounter/store.rs: schema6 native tables, atomic import, mixed listing/loading,
  migration, deletion, backup and immutable validation.
- src/encounter/model.rs, mod.rs, history.rs: source-aware summary/public entry points
  and read-only/native import service.
- src/app/encounter_history.rs, ui.rs: serialized native commands/events and source-aware presentation.
- src/bin/catalog-compiler.rs: narrow native inspection/import validation commands.
- tests/encounter_native.rs, encounter_history.rs, app_encounter_history.rs,
  app_ui_sizing.rs: source, storage, worker and rendered workflow regressions.
- specs/127-native-completed-import/: specification, research, model, contracts,
  quickstart, tasks, analysis, audit and verification.
- docs/src and docs/project: canonical native guidance, plan archive/index/ledger.
- .github/scripts/docs-policy*: native contract guards and removal fixtures.

## Verification and delivery

Foreground cargo fmt --all -- --check, cargo clippy --all-targets --all-features
-- -D warnings, cargo test --all --locked and release binary build. Run existing
policy fixture suite, spelling, trust/brand, mdBook test/build/links, site policy
and headless rendering, release-note/packaging fixtures, text hygiene and diff check.
Verify representative-size native input and a private temp-store import without
changing the owner file or application history. Public fixtures are sanitized.
Commit, automatically push and publish one official PR closing#272, resolve all
findings within at most two rounds, verify exact final-head CI, and ask owner to merge.

## Complexity Tracking

No constitution violations. Separate native evidence types and two native tables
are required to preserve addon canonical contracts and avoid fabricated observations.
