# Plan 057: Native v15 Format Coverage

Status: In Progress, Active

## S131 (2026-10-15)

The owner authorized [S131](../../../specs/131-native-v15-format-coverage/spec.md) for #296 and #297, with grammar established first, automatic push and official PR publication, at most two Codex review rounds, and owner final review and merge.

1. Create the exact S131 branch/specification, record clarifications and validate requirements checklists.
2. Archive completed Plan056 after owner-merged PR #298 and synchronize plan indexes and machine/prose ledgers.
3. Establish pinned producer grammar or representative evidence for normal/inline, verbose/separate and verbose/inline v15. Record exact supported shapes and distinguish observed evidence from synthetic tests. This is a prerequisite for design completion and implementation.
4. Complete research, data model, grammar/compatibility contracts, validation guide, tasks and blocking analysis. Resolve findings before product implementation.
5. Write failing parser, byte-partition, checkpoint, compatibility, automatic-runtime and consumer tests, then implement supported profiles through shared typed validation while retaining exact source evidence.
6. Reconcile source capabilities and current guide/API/MCP/contracts without changing game settings, adding player workflows or inventing unavailable measurements.
7. Run applicable Rust, storage/runtime/API and documentation gates, commit, push and publish one official PR. Resolve all findings within two Codex review rounds and verify final-head hosted checks.
8. Stop for owner final review and merge. No automatic merge, release/tag or owner field-testing task is included.

## Delivery evidence (2026-10-15)

S131 is implemented in [PR #299](https://github.com/h8rt3rmin8r/eso-weave/pull/299).
Pinned static v15 producer writers established grammar before implementation.
All 1207 local Rust tests, documentation/browser and release gates passed.
Codex round 1 completed without findings; no second round was needed. Windows,
Linux, documentation, trust, dependency and CodeQL gates passed on implementation
head `376b0e6`. The PR conversation records final delivery-head checks. The plan
remains active pending owner final review and merge.
