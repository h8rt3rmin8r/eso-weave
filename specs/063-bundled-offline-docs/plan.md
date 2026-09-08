# Implementation Plan: Bundled Offline Documentation

**Branch**: `codex/s063-bundled-offline-docs` | **Date**: 2026-09-07 | **Spec**: `specs/063-bundled-offline-docs/spec.md`

## Summary

Close #82 by compiling the validated mdBook corpus into production executables, serving only immutable files from one bounded loopback listener, and exposing a reusable Help > Documentation action with native browser opening on Windows and Linux.

## Technical Context

- **Language/Version**: Rust 2021, MSRV 1.96
- **Primary Dependencies**: Rust standard library, existing eframe/egui, existing `windows-sys`
- **Build Inputs**: `docs/book.toml`, `docs/src/`, `docs/theme/`, exact mdBook 0.5.4 and mdbook-linkcheck2 0.13.0
- **Storage**: Compile-time immutable bytes only; no runtime extraction
- **Testing**: Rust unit/integration tests, headless egui, Node documentation policy, mdBook, Windows/Linux release builds
- **Target Platform**: Windows and Linux desktop
- **Performance Goals**: Local first start within 250 ms; repeat opens reuse; one bounded request buffer
- **Constraints**: Loopback only; GET/HEAD only; no request-derived disk path; one-second shutdown; no Windows console; UTF-8 without BOM; LF
- **Scale/Scope**: One issue, 76 current generated files, about 4.5 MB uncompressed site input, two supported platforms

## Constitution Check

- PASS: Spec-kit artifacts and analysis precede source changes.
- PASS: Documentation has no generated-input or game-state authority.
- PASS: The listener is bounded, off the input-hook thread, and lifecycle-owned.
- PASS: Canonical documentation and release workflows stay reproducible.
- PASS: Pinned workflow and release guidance changes receive a dated decision.

## Project Structure

```text
build.rs                               strict release generation and manifest emission
assets/dev-docs/                       checked non-release fixture
src/documentation/                     lookup, HTTP service, browser launch
src/app/ui.rs                          Help > Documentation and notice
src/app/strings.rs                     stable menu copy
tests/                                 protocol, lifecycle, UI, policy regressions
.github/workflows/ci.yml               cross-platform release smoke
.github/workflows/release.yml          exact documentation prerequisites
docs/src/                              operator and contributor contracts
docs/project/                          release guidance and plan lifecycle
specs/063-bundled-offline-docs/
```

## Implementation Strategy

1. Add failing manifest, route, protocol, reuse, concurrency, shutdown, browser-seam, and menu-copy tests.
2. Add the fixture and build manifest generator; make release generation strict.
3. Implement lookup, bounded loopback GET/HEAD service, and security headers.
4. Implement platform openers, app lifecycle, Help action, and visible failures.
5. Install exact tools in Windows/Linux CI and release jobs and exercise release embedding.
6. Update access, architecture, testing, release, lifecycle, and dated decisions.
7. Record size evidence, complete analysis and full gates, then publish for two review rounds.

## Complexity Tracking

| Deviation | Why needed | Simpler alternative rejected |
| --- | --- | --- |
| Profile-specific input | Production must fail on missing docs while tests stay practical | mdBook everywhere burdens compilation; optional release input can omit help |
| Narrow HTTP parser | Same-origin routing makes search portable without extraction | A general server adds surface; `file://` is less reliable; extraction adds path authority |
| Generated Rust manifest | Hashed filenames change and every file must be embedded | Hand-maintained includes drift and cannot prove completeness |
