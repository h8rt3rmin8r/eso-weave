# Implementation Plan: Encounter SavedVariables Import

**Branch**: `codex/s076-encounter-import` | **Date**: 2026-09-10 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/076-encounter-import/spec.md`

## Summary

Implement issue #133 as the hostile-data desktop boundary for S075 encounter
captures. A reusable Rust module performs a stable bounded read, parses only a
restricted SavedVariables table grammar without Lua execution, validates every
terminal invariant and payload shape, produces deterministic canonical JSON and
SHA-256 hashes, and transactionally appends immutable raw bytes to a dedicated
user-selected SQLite store. Explicit list, backup, and deletion operations make
local ownership testable. Non-interactive CLI commands expose the same library
surface while encounter metrics and desktop history remain later slices.

## Technical Context

**Language/Version**: Rust 1.96, edition 2021; Markdown, JSON, SQL contracts

**Primary Dependencies**: Serde and serde_json; rusqlite 0.40.2 with bundled and
backup features; sha2; tempfile; existing bounded and atomic file primitives

**Storage**: Dedicated schema-versioned `encounters.sqlite` selected by the
caller; immutable canonical JSON BLOBs; consistent SQLite snapshot backups

**Testing**: Rust unit and integration tests, hostile fixture matrix, SQLite
integrity checks, full Cargo merge gate, mdBook and repository hygiene gates

**Target Platform**: Windows 10 and 11 x64 and Linux x64

**Project Type**: Single-crate desktop application, reusable library, and
maintainer CLI

**Performance Goals**: Accept the current 100,000-event, 64 MiB import ceiling
within documented production verification budgets; indexed list and identity
checks independent of event count

**Constraints**: Local and explicit only; no Lua execution; no names, upload,
telemetry, catalog mutation, automatic discovery, metric derivation, hidden
retention, or partial store replacement

**Scale/Scope**: S075 schema v1, fourteen event kinds, encounter store schema
v1, four CLI lifecycle commands, one shared restricted-table parser

## Constitution Check

*GATE: Passed before Phase 0 research and rechecked after Phase 1 design.*

- **Spec-first traceability**: PASS. Issue #133 maps to S076 and the S069/S075
  contract authority; all spec-kit artifacts precede implementation.
- **Safety invariants**: PASS. Encounter input remains local, read-only with
  respect to the game, non-executing, bounded, and independent of PixelBeacon,
  collector promotion, catalog mutation, input, and automation.
- **Test-first delivery**: PASS planned. Hostile parsing, invariant, atomicity,
  identity, lifecycle, and regression tests fail before implementation.
- **CI parity**: PASS planned. Rust and Cargo metadata change, so formatting,
  strict Clippy, and the complete locked test suite are commit gates.
- **Bounded scope**: PASS. This slice imports terminal handoffs only; summaries,
  metrics, recommendations, path discovery, and desktop history remain out.
- **Configuration discipline**: PASS. Raw capture facts use a dedicated store,
  never the settings file or catalog database.
- **Text hygiene**: PASS by design with automated final scans.

No constitution amendment or exception is required.

## Project Structure

```text
specs/076-encounter-import/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── checklists/
│   ├── requirements.md
│   └── import-safety.md
├── contracts/
│   └── encounter-import-contract.md
└── tasks.md
```

```text
src/
├── saved_variables.rs
├── encounter/
│   ├── mod.rs
│   ├── model.rs
│   ├── validate.rs
│   └── store.rs
├── collector/
│   └── parser.rs
├── bin/
│   └── catalog-compiler.rs
└── lib.rs
tests/
├── encounter_import.rs
└── fixtures/encounter/
docs/src/reference/encounter-data-and-metrics.md
docs/project/build-plans/{README.md,plan-038.md}
docs/project/{migration-ledger.json,migration-ledger.md}
Cargo.toml
Cargo.lock
CHANGELOG.md
```

**Structure Decision**: Add one focused encounter library module and reuse the
existing binary as the repository's non-interactive data-maintenance surface.
Extract the collector's sound restricted-table grammar into a crate-private
shared module instead of copying approximately 500 lines of security-sensitive
parser logic. Collector behavior remains behind its existing adapter and
regression tests.

## Delivery Sequence

1. Complete specify, clarify decisions, requirements and import-safety
   checklists, research, data model, contract, quickstart, tasks, and analysis.
2. Move issue #133 to In progress with Slice S076.
3. Add failing shared-parser regression and hostile encounter import tests.
4. Extract the shared restricted SavedVariables parser without changing the
   collector's accepted grammar or errors.
5. Add failing terminal invariant, canonicalization, identity, atomicity,
   corruption, backup, deletion, and production-budget tests.
6. Implement typed encounter parsing, validation, canonicalization, and hashing.
7. Implement store schema v1, immutable append, deterministic listing, explicit
   deletion, snapshot backup, and typed receipts.
8. Add explicit encounter import, list, backup, and delete CLI commands.
9. Update canonical documentation, active build plan, migration ledgers, and
   changelog.
10. Run post-implementation analysis and every local merge gate.
11. Commit, push the authorized branch, open the official PR, and move issue
    #133 to PR review.
12. Resolve every CI and external review finding, request at most the authorized
    second Codex round, and stop for the operator's merge ritual.

## Design Decisions

1. Store normalized raw facts, not the original Lua serialization. This retains
   accepted observations while removing syntax variation and executable text.
2. Hash both stable source bytes and deterministic canonical bytes. Source hash
   supports audit receipts; canonical hash owns identity and idempotency.
3. Use `(session_id, encounter_id)` as the semantic unique key and canonical
   SHA-256 as the content key. Same facts are idempotent; changed facts under the
   same identity are rejected rather than overwritten.
4. Use a dedicated SQLite database with an update-blocking trigger. SQLite
   transactions supply atomic append and deletion, while its backup API supplies
   a consistent snapshot even when the source database is open.
5. Accept truthful partial captures. Rejecting all partial data would discard
   explicitly modeled evidence and contradict the S075 handoff contract.
6. Require an expected channel at the call boundary. This prevents accidental
   Live or PTS mixing without changing or promoting provenance.
7. Enforce kind-specific payload keys and scalar types in addition to the broad
   JSON schema. The emitter's privacy boundary depends on rejecting unexpected
   strings, even when a generic schema would permit them.
8. Keep raw captures as canonical BLOBs rather than normalizing event rows in
   S076. Issue #134 owns rebuildable derived projections and can parse the
   immutable format without prematurely coupling raw storage to metric queries.
9. Expose lifecycle commands through the existing maintainer binary. A second
   binary would add packaging and release surface without user value before the
   desktop UI slice.
10. Do not silently repair corrupt or future-version stores. Preservation and a
    typed error leave recovery authority with the user.

## Complexity Tracking

| Design pressure | Why required | Simpler alternative rejected |
| --- | --- | --- |
| Shared restricted parser | Encounter and collector inputs share the same hostile SavedVariables grammar | Copying the parser would duplicate security-sensitive bounds and allow the two paths to drift |
| Dedicated SQLite store | Raw capture immutability, atomic append/delete, indexed identity, and consistent backup are acceptance requirements | Storing files beside configuration or in `catalog.sqlite` violates storage-plane ownership; an ad hoc manifest adds crash-recovery states |
| Strict payload validation beyond JSON schema | Generic scalar maps can smuggle names or unbounded tokens | Trusting the emitter or schema alone weakens the hostile-input boundary |

No constitution violation remains.
