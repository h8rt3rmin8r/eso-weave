# Implementation Plan: S129 Automatic Catalog Maintenance

**Branch**: `codex/s129-automatic-catalog-maintenance` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md)
**Input**: #285-#288/#291, owner-authorized push/PR, at most two reviews, owner merge.

## Summary

Deliver the five complete issues as one coherent Catalog path: transactional combined package, bounded automatic collection, deterministic accumulation, automatic immutable activation/recovery, and independent durable optouts. Follow the installed speckit specify/clarify/checklist/plan/tasks/analyze/implement sequence.

## Technical Context

Rust 1.96 single crate, existing serde/rusqlite/tempfile/egui and Lua harness; no new dependencies. Windows x64/Linux x64. Existing config JSON, game-owned SavedVariables and immutable SQLite generations. Pure models, injected filesystem/lifecycle failures, Lua doubles, real packaged baseline, service and headless UI integration. Console children use CreateNoWindow/redirected noninteractive I/O. No game action, live inbound transport or field test.

## Constitution Check

Pre-research: owner explicitly selected consolidation/automatic collection; old separate-package/manual-Catalog clauses require the scoped amendment below before code. Preserve exact inventory/ownership, raw data isolation, input safety, historical readers, opt-in authenticated local reads and all required tests.

Post-design: PASS after scoped topology amendment. Catalog and native preference contracts precede future native workers. Immutable verified activation and per-operation reader generation preserve data integrity. No unapproved product restriction or new transport. All complete acceptance criteria remain required; later UI/native integration issues are not claimed complete.

## Implementation Sequence

1. Freeze owner scope/spec/clarifications/checklists, research/contracts/model/quickstart and blocking analysis. Amend obsolete topology policy with dated decision.
2. Write red migration/race/package, automatic-Lua, independent-baseline/union, settings/eligibility/cancellation regressions.
3. Implement package transaction and startup configuration (US1), automatic collector/finalization (US2), accumulation/build/activation (US3), preference/model/UI and background runtime integration (US4).
4. Integrate shared contracts: recognized presence versus mutation ownership, actual-root channel, stable completed generation, semantic fingerprint, automatic outcome and coherent activation event.
5. Update nearby active docs, policy fixtures, plan/ledger/changelog. Run complete Rust, docs/render/embedded/repository gates.
6. Commit/push official PR closing the five issues, at most two Codex rounds, address every finding, verify exact final-head CI and zero unresolved threads, owner merge handoff.

## Project Structure

Existing src/beacon/package.rs and src/data_addon.rs own migration/facades; addon/PixelBeacon owns shipped modules; tests/fixtures/legacy-addon preserves retired source evidence; src/catalog/accumulate.rs, collector/import.rs, catalog_pipeline/mod.rs and catalog_update/mod.rs own facts/build/activation. src/data_maintenance.rs owns prefs/eligibility/reconciliation; config/mod.rs, main.rs and app UI bind runtime/settings. tests retain domain safety suites and add meaningful automatic/migration cases. specs/129-automatic-catalog-maintenance contains all evidence; docs/project/build-plans/plan-055.md directs this slice.

## Decisions

- Ship/load only beacon/bootstrap/Catalog/config, never legacy Encounter.lua auto-resume. Keep historical parser/source evidence.
- Preserve the EsoWeaveDataSaved table contract. ESO derives the saved-data filename from the manifest identity, so the unified package saves PixelBeacon.lua. Automatically read legacy EsoWeaveData.lua before current PixelBeacon.lua with independent stable-input/retry state and compatible accumulation; never rewrite, copy or delete either game-owned file. Keep module state independent.
- Serialize package and configuration mutation/recovery per canonical AddOns root so the background reconciler cannot retire a journal while an install, uninstall or configuration transaction owns it. Preserve the existing external filesystem race checks.
- Generate channel from actual installation context, never current Settings selection or guessed client patch/API.
- Distinguish compatible presence/capabilities from ownership allowing mutation.
- Persist the Catalog startup preference through a bounded config-only transaction even for compatible manually maintained packages. Exact literal grammar, stable confined reads, no-replace retirement/commit and recovery protect that single configuration seam; package replacement/deletion authority remains separate.
- Desktop stop is immediate; loaded Lua optout applies next natural addon load.
- Reuse verified in-process compiler and SQLite facts, never shell compiler or source execution.
- Merge only compatible identities; preserve earlier immutable artifact for historical/version access; semantic no-op excludes acquisition lineage.
- Whole viewer removal and full consumer/final-asset sweep remain #292/#293/#294 after native work; update changed surfaces now.

## Complexity Tracking

No workspace expansion or dependency additions. Stronger migration replaces unsafe in-place two-file operations; no duplicate compiler/ingest authorities.
