# Implementation Plan: Interface and maintenance regressions

**Branch**: `codex/133-ui-maintenance-regressions` | **Date**: 2026-10-15 | **Spec**: [S133](spec.md)

## Summary

Implement #306-#310 in the existing single-crate Rust/egui application, with regression tests written before each repair.

## Technical Context

Rust pinned by rust-toolchain.toml; eframe/egui 0.36; serde_json UI preferences; tracing diagnostics; Windows 10/11 and Linux x64. Reuse existing headless egui, tempfile, package lifecycle and maintenance test seams. No new dependencies or release identity changes.

## Constitution Check

PASS before research and after design: issue-scoped full spec-kit artifacts; test-first work; exact ownership/transactional package protections unchanged; full fmt/Clippy/locked tests before commit; no game commands or private payload diagnostics; UTF-8 without BOM/LF; PR integration with owner merge only.

## Research and Decisions

- Remove unconditional toggle resizing; use measured growth only. Limit growth by usable viewport bounds and account for chrome/origin, retaining scroll accessibility when a display cannot fit intrinsic content. No automatic shrink.
- Store explicit visibility in existing UI preferences, defaulting missing values to closed. Persist on normal close and preserve live layout values when a Settings draft is applied.
- Overlay a real egui modal. Dismissal never emits UninstallBeacon. Preserve existing guarded package mutation and data-retention coverage.
- Inspect installed manifest support for current selected-environment evidence. Keep normal lifecycle text concise and omit the separate routine Game API row from presentation while retaining technical evidence/model diagnostics.
- Windows discovery includes nonexistent Live/PTS candidate roots; automatic migration currently rejects them before checking for absence. Treat missing roots as no-op in automatic reconciliation only. Real failures retain typed categories, per-root error identity, bounded retries and recovery transitions.

## Project Structure

`src/app/{ui.rs,mod.rs,settings_form.rs,strings.rs}`: geometry, modal, visibility, status.
`src/beacon/package.rs`: automatic migration no-op and installed manifest facts.
`src/data_maintenance/runtime.rs`: per-root attempt diagnostics and retries.
`tests/{app_ui_sizing,app_view_model,app_settings,s129_package}.rs` and runtime unit tests: regressions.
Canonical docs and historical sizing-contract annotations are updated alongside this packet.

## Execution

Write failing behavioral tests, repair each story, rerun focused tests, then run full required gates and relevant docs/policy checks. Commit, publish the authorized branch/PR, inspect exact-head CI and review threads, address review findings, and stop before merge.

## Complexity Tracking

No constitution exception, new service, new package, or dependency is needed. Native usable-area facts remain behind a small platform seam.
