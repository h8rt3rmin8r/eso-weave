# Implementation Plan: v0.19.1 Patch Release Candidate

**Branch**: `codex/s134-v0191-release` | **Date**: 2026-10-17 | **Spec**: [spec.md](spec.md)

**Input**: Candidate #315 under release coordinator #314 and Plan060.

## Summary

Prepare the existing S133 fixes as a reviewed v0.19.1 candidate. Use the current
cargo-release configuration rather than hand-editing version surfaces or creating
a parallel release mechanism. No new runtime behavior or dependency is needed.

## Technical Context

- Language/version: existing pinned Rust toolchain and single-crate Cargo package.
- Dependencies: existing cargo-release 1.1.2, Node policy scripts, Git Bash release
  contracts, mdBook 0.5.4/linkcheck2 0.13.0, typos and headless Chrome. No additions.
- Storage: committed release/planning documents and version-sensitive fixture;
  temporary validation evidence stays under ignored target/.
- Testing: existing Rust safety/regression suite, release-note and Debian contract
  scripts, documentation/render/brand/trust policy and production binary build.
- Platforms: Windows 10/11 x64 and Linux x64. Linux packaging contracts are exercised
  by hosted Linux CI; local Windows checks do not prove Linux package behavior.
- Constraints: clean tree for rollover; tag=false, push=false, publish=false;
  no owner merge, tag, publication, addon deployment or field-testing task.
- Scale/scope: five S133 outcomes, six governed identity files, one candidate PR.

## Constitution Check

PASS before research and after design. The full Spec-Kit packet precedes changes.
All safety-critical tests stay enabled. Existing tests provide the appropriate
coverage for identity-only changes; no runtime behavior or new test seam is added.
Rust parity runs before candidate handoff and before any buildable Rust change.
The initial documentation-only preparation commit uses the constitution's stated
exception. Pinned release scripts/config/workflows remain unchanged. Text is
UTF-8 without BOM, LF and standard hyphens. Operator control of merge/tag/release
is explicit; publication authority is not inferred from candidate preparation.

## Phases

1. Research the protected-main release authorities, exact version replacements,
   prior S132 receipt and current local/hosted gates. Dispatch read-only research
   through the installed planning command and consolidate in research.md.
2. Define candidate identity and lifecycle in data-model.md, review/release
   surfaces in contracts/release.md, and runnable scenarios in quickstart.md.
3. Generate dependency-ordered tasks and run the read-only analyze gate.
4. Exercise the existing missing-Highlights rejection; write concise Highlights,
   establish Plan060 and synchronize indexes/ledger/context. Validate and commit
   preparation so the rollover begins from a clean tree.
5. Inspect the dry run, execute the governed local v0.19.1 rollover, inspect its
   six-file identity commit and rerun the actual candidate validation/build gates.
6. Publish the official PR, read final-head checks and review threads, resolve
   findings and stop for owner review before merge/tag/publication.

## Project Structure

- specs/134-v0191-release/: specification, checklists, plan, research, data model,
  contracts, quickstart, tasks, analysis record and observed verification receipt.
- docs/project/build-plans/plan-060.md and README.md: current slice authority.
- docs/project/migration-ledger.json and .md: matching lifecycle records.
- CHANGELOG.md: Highlights plus retained full patch history and dated preparation decision.
- Rollover-owned files: Cargo.toml, Cargo.lock, CHANGELOG.md, README.md,
  docs/src/README.md, specs/073-reviewed-catalog-pipeline/fixtures/capture-request.json.
- CLAUDE.md and .specify/feature.json: current slice context.
- src/, tests/, addon contracts, dependencies, packaging and workflows: existing
  behavior and gates, with no planned edits.

## Decisions and Alternatives

Use local cargo-release instead of manual identity edits because its exact
replacements and verification are the authoritative release contract. Preserve
separate candidate #315 and coordinator #314 instead of closing publication on
identity merge. Retain existing release-note/policy/regression tests instead of
adding an assertion that only mirrors a constant. Use a dated candidate snapshot
without claiming v0.19.1 assets are published. No complexity exception is needed.
