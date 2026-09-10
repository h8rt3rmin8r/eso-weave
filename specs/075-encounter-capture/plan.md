# Implementation Plan: Privacy-Minimized Encounter Capture

**Branch**: `codex/s075-encounter-capture` | **Date**: 2026-09-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/075-encounter-capture/spec.md`

## Summary

Implement issue #132 as a third, narrowly authorized addon named
`EsoWeaveEncounter`. The user explicitly arms one Live or PTS encounter. The
addon records privacy-minimized numeric events in a bounded SavedVariables
envelope, assigns encounter-local actor IDs, preserves sequence and monotonic
duration, and declares loss before disarming. S075 ships the capture artifact,
its versioned contract, executable Lua 5.1 tests, canonical documentation, and
the governance amendment needed to authorize this new bridge. Desktop import
and storage remain issue #133.

## Technical Context

**Language/Version**: ESO Lua 5.1-compatible addon code; Rust 1.96 integration
tests; Markdown and JSON contracts

**Primary Dependencies**: Documented ESO addon APIs pinned by S069; `mlua` with
vendored Lua 5.1 as a test-only dependency; existing Serde JSON test support

**Storage**: One bounded `EsoWeaveEncounterSaved` SavedVariables table; no
desktop database or configuration mutation

**Testing**: Executed Lua state-machine tests with a deterministic ESO API
harness, static confinement checks, JSON schema and fixture checks, complete
Cargo merge gate, documentation and text hygiene gates

**Target Platform**: ESO Live API 101050 and PTS API 101051 on PC; repository
tests on Windows and Linux

**Project Type**: Desktop repository with a separately versioned ESO addon
artifact

**Performance Goals**: Constant bounded work per callback; 500 ms boss sampling;
1,000 ms performance sampling; no unbounded table growth

**Constraints**: Explicit arm; one encounter; at most 100,000 stored events,
4,096 encounter-local actors, and 32 MiB conservative estimated output; two
terminal event slots and 2 KiB terminal byte reserve; no names, upload, Pixel
Bus transport, protected calls, gameplay mutation, or automation coupling

**Scale/Scope**: Fourteen event kinds, one SavedVariables envelope version, one
Lua addon, one executed harness, and no desktop import surface

## Constitution Check

*GATE: Requires and includes a constitution amendment before implementation.*

- **Spec-first traceability**: PASS. Issue #132 maps to S075 and Plan 038.
- **Safety invariants**: PASS after amendment. The new addon has its own name,
  state root, confinement tests, and no PixelBeacon or input relationship.
- **Test-first delivery**: PASS planned. Lua 5.1 harness tests fail before the
  addon behavior is implemented.
- **CI parity**: PASS planned. Rust sources and Cargo metadata change, so the
  complete format, Clippy, and locked test gate is mandatory before commits.
- **Bounded scope**: PASS after amendment. The addon uses documented callbacks
  and local SavedVariables only.
- **Configuration discipline**: PASS. Capture state is game-owned
  SavedVariables, not desktop configuration or runtime state.
- **Text hygiene**: PASS by design with automated final audit.

The current constitution says the only allowed addon surfaces are PixelBeacon
and ESO Weave Collector. Quietly treating encounter capture as part of either
would violate ownership and make deletion and transport boundaries ambiguous.
S075 therefore amends the constitution from 2.1.0 to 2.2.0 to authorize a third
separate, read-only, user-armed encounter bridge. Existing prohibitions remain.

## Project Structure

```text
specs/075-encounter-capture/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── checklists/
│   ├── requirements.md
│   └── capture-safety.md
├── contracts/
│   ├── encounter-capture-contract.md
│   └── encounter-capture.schema.json
├── fixtures/
│   └── representative-capture.json
└── tasks.md
```

```text
addon/EsoWeaveEncounter/{EsoWeaveEncounter.txt,EsoWeaveEncounter.lua}
tests/encounter_addon.rs
docs/src/{SUMMARY.md,features/encounter-capture.md,
  reference/encounter-data-and-metrics.md,development/architecture.md,
  development/test-strategy.md,getting-started/responsible-use.md}
.specify/memory/constitution.md
CLAUDE.md
docs/project/{encounter-model.json,migration-ledger.json,migration-ledger.md,
  build-plans/README.md,build-plans/plan-038.md}
Cargo.toml
Cargo.lock
CHANGELOG.md
```

**Structure Decision**: Keep capture in a third addon directory instead of
expanding PixelBeacon or the catalog collector. Use a test-only Lua 5.1 runtime
to execute the actual addon state machine against a deterministic API harness.
This intentionally improves on source-string-only addon tests because consent,
overflow, cleanup, and privacy are behavioral safety properties.

## Delivery Sequence

1. Complete specify, clarify, requirements, safety checklist, research, data
   model, contracts, quickstart, tasks, and pre-implementation analysis.
2. Amend the constitution and agent guidance for the third narrow addon bridge.
3. Move issue #132 to In progress with Slice S075.
4. Add failing executed Lua tests for consent, complete capture, privacy,
   overflow, clock reset, interruption, commands, and teardown.
5. Implement the manifest and Lua state machine until the harness passes.
6. Add the representative contract fixture and invariants.
7. Update canonical documentation, active build plan, migration ledgers, and
   changelog.
8. Run post-implementation analysis and every local merge gate.
9. Commit, push the authorized feature branch, open the official PR, and move
   issue #132 to PR review.
10. Resolve every CI and external review finding, trigger no more than the
    authorized second Codex round, and stop for the merge ritual.

## Design Decisions

1. The addon is a third bridge, not a new mode of PixelBeacon or the discovery
   collector. Distinct identities prevent accidental lifecycle and data mixing.
2. Capture is armed for one encounter and never begins on addon load. Explicit
   channel selection and single-use authority minimize privacy and retention.
3. Callback names are ignored at the handler boundary. Persisted payloads use
   only numbers, booleans, stable reason strings, and local actor integers.
4. Source sequence advances for every observation, including omitted events.
   Capacity reserved up front allows finalization to append a discontinuity and
   terminal event without exceeding the budget.
5. Game-time deltas build a nondecreasing elapsed clock. A backward raw clock
   creates a declared discontinuity and starts a new raw-clock baseline.
6. The addon stores a conservative byte estimate rather than claiming ESO's
   exact serialized size. Exact bytes and retention guidance require live issue
   #131 evidence.
7. The desktop importer, not Lua, computes canonical SHA-256. This avoids a
   bespoke cryptographic implementation and keeps #133 responsible for hostile
   input validation.
8. The actual Lua source executes under vendored Lua 5.1 in tests. The new
   test-only dependency is proportional to the safety and privacy state machine
   and does not enter the shipped application.

## Complexity Tracking

| Governance pressure | Why required | Simpler alternative rejected |
| --- | --- | --- |
| Third addon bridge | Combat volume and privacy ownership differ from both existing addons | Expanding PixelBeacon would turn a safety signal into bulk transport; expanding the collector would mix out-of-combat catalog enumeration with combat capture |
| Lua 5.1 test runtime | Consent, loss, and callback teardown need behavioral evidence | Source-string assertions cannot prove state transitions or persisted privacy |

No other constitution exception remains after the 2.2.0 amendment.
