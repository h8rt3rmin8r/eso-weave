# Implementation Plan: Documentation Release Verification

**Branch**: `codex/s065-docs-release-verification` | **Date**: 2026-09-08 | **Spec**: `specs/065-documentation-release-verification/spec.md`

## Summary

Close #84 and parent epic #83 only after exercising the downloadable v0.15.0 Windows MSI, Linux deb, and Linux AppImage, validating their checksums and bundled documentation behavior, and committing a complete evidence receipt. The intended repository diff contains no runtime code.

## Technical Context

**Language/Version**: Markdown evidence, PowerShell 7 on Windows 11, Bash on Ubuntu 24.04 WSLg

**Primary Dependencies**: GitHub Release v0.15.0, Windows Installer, WSLg, dpkg, AppImage runtime, operating-system browser, loopback HTTP inspection

**Storage**: Isolated temporary artifact and installation directories plus one tracked evidence receipt

**Testing**: SHA-256 verification, package install and payload inspection, UI and browser interaction, HTTP and socket probes, documentation policy, repository contract tests

**Target Platform**: Windows 11 x64 and Ubuntu 24.04 x64 under WSLg

**Project Type**: Cross-platform desktop release verification

**Performance Goals**: Documentation opens interactively; repeated access reuses one endpoint; shutdown removes the listener promptly

**Constraints**: Use published artifacts only; no fabricated observation; no host-wide network disruption; no runtime or pinned-tooling change; UTF-8 without BOM and LF

**Scale/Scope**: Five release assets, three executed package forms, two platforms, one documentation service contract, issues #84 and #83

## Constitution Check

- Actionable issue and full spec-kit sequence precede evidence implementation: PASS
- Release publication was separately and explicitly authorized before S065 evidence: PASS
- Installed verification remains distinct from implementation and publication: PASS
- No safety-critical runtime surface is changed: PASS
- No Rust source change is planned, so cargo parity is supporting rather than a commit gate: PASS
- Pinned release artifacts remain unchanged: PASS
- Artifact-dependent UI verification occurs only after downloadable packages exist: PASS
- Any product defect creates separate implementation work and blocks closure: PASS

## Project Structure

```text
specs/065-documentation-release-verification/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/
│   └── verification-receipt.md
├── checklists/
│   ├── requirements.md
│   ├── release-evidence.md
│   └── delivery.md
└── tasks.md

docs/project/release-verification/
└── v0.15.0-documentation.md
```

**Structure Decision**: Keep temporary packages and machine output outside the repository. Commit only the normalized receipt and the spec-kit and lifecycle records needed to audit it.

## Execution Sequence

1. Complete specification, clarification decisions, checklists, research, model, receipt contract, tasks, and pre-evidence analysis.
2. Download and hash every v0.15.0 asset.
3. Install and exercise the Windows MSI, capturing UI, browser, process, HTTP, accessibility, and cleanup evidence.
4. Install and exercise the Linux deb and AppImage under WSLg with the same matrix.
5. Inspect package payloads, request origins, listener binding, reuse, safe failures, sidecar absence, and cleanup.
6. Produce the durable receipt and map every #84 criterion to evidence.
7. If any product failure occurs, file a separate issue and keep #84 and #83 open; otherwise prepare their closure through the PR.
8. Run documentation policy, text hygiene, cross-artifact analysis, and final scope audit.
9. Commit, push, open the official PR, complete up to two authorized Codex review rounds, and wait for green CI.

## Complexity Tracking

No constitution violation or additional architecture is introduced.
