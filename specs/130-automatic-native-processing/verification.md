# S130 verification and delivery

Date: 2026-10-15. Branch: `codex/s130-automatic-native-processing`.
Base: merged S129/main `9a2580d07238822a2952f488856a419cc63a8f14`.
Scope: issues #289, #290 and #292-#294 with the owner's explicit normal/separate
native v15 revision. Normal/inline is #296; verbose/separate and verbose/inline
are #297. The five issue bodies and project stages were reconciled to that scope.

## Implemented result

Pure incremental parsing yields each validated completed fight without END_LOG,
persists bounded pending state, and preserves schema1 evidence. Schema2 logical
aliases reconcile copied/extracted runs without replacing first accepted bytes.
Store schema7 commits completed evidence, projection and checkpoint revision
together, preserves supported schema1-6 stores and honors native deletion intent.

An independent native worker discovers actual supported roots, catches up and
tails read-only sources without UI or Catalog availability. Root tokens and final
commit admission serialize disable/removal/shutdown with durable writes. Catalog
and native cancellation remain independent. Background consumers persist qualified
results with actual Catalog/source/algorithm identity. Retained authenticated
HTTP/MCP reads keep their fixed database and read-only authority.

The complete Catalog Update, Encounter History and Data Details interfaces and
their workers/state/helpers are removed. One combined PixelBeacon lifecycle and
the two existing disable preferences remain. Current docs, help, contracts,
policies, diagrams, screenshots and release-embedded guide describe that result.

## Evidence

- `acceptance-evidence.md` maps all 83 revised criteria to concrete tests/inspection.
- `parser-verification.md`: 45 focused parser/native tests, actual long-stream
  bounds and independently reproduced pre-S130 schema1 canonical hash.
- `store-verification.md`: atomic ingest/migration/suppression/backup/derivation
  coverage, historical compatibility and obsolete Catalog worker retirement.
- `surface-audit.md` and `surface-verification.md`: complete dispositions,
  171 policy tests, 28 source-built capture variants, seven curated PNGs,
  pinned book/link/spelling/browser matrices and 10 release guide tests.

Primary integration also covers append-range witnesses, late-root inventory
fairness, root-specific cancellation, exact commit/disable serialization,
catchup/append/restart/copy/truncation, late Logs creation, independent Live/PTS,
peer removal followed by peer derivation, multi-Steam-root discovery and bounded
native shutdown without cancelling Catalog or preventing restart.

## Corrected integration failures

The native-only Drop initially cancelled the global maintenance generation,
including Catalog. A failing shutdown regression demonstrated the error; native
shutdown now cancels only native root jobs. Derivation tokens rebind after a root
disappears and opening/processing failures use bounded retry, preserving peers.

The first full compile exposed old view-model tests calling removed dialogs.
Meaningful ownership/idempotence/foreign-byte assertions now exercise the one
combined lifecycle; domain tests remain. Two schema4 fixture builders now remove
schema7 objects before claiming to be historical schema4 databases. Strict schema
validation remains unchanged. The focused corrected targets passed 85 model,
64 historical addon and 18 local service tests.

One initial full-suite local-service client read aborted with Windows10053. The
unchanged local-service suite passed its focused rerun. No assertion, endpoint or
product timeout was relaxed. A separate early linker failure was caused by a
missing S130 temporary directory in the verification launcher, corrected before
rerunning. Final full-suite results are recorded below.

## Final local gates

Post-publication inspection found permanently disappeared filenames could retain
candidate slots indefinitely. The new disappeared_sources_release regression
failed before the fix, then passed after NotFound retires only the in-memory
candidate while retaining its checkpoint. Full formatting/Clippy/locked tests
passed again: **1189 passed, zero failed/ignored**, including all local-service
tests (`target/s130-review-{fmt,clippy,full}.log`). Red evidence is
`target/s129-s130-disappearance-red.log`.
The review-stage release executable, 10 embedded guide tests and one connection
regression also passed (`target/s130-review-release.log`,
`target/s130-review-embedded-guide-full.log`,
`target/s130-review-embedded-connection.log`).

The aggregate Rust gates passed: `cargo fmt --all -- --check`,
`cargo clippy --all-targets --all-features --locked -- -D warnings`, and
`cargo test --all --locked --no-fail-fast`. Across 53 target/doc-test summaries,
**1188 tests passed, zero failed and zero ignored**. Evidence:
`target/s130-initial-{fmt,clippy,full}.log`. The release executable build passed
(`target/s130-initial-release.log`). Release-profile embedded guide tests passed
10/10 and the accepted nonblocking-connection regression passed 1/1
(`target/s130-initial-embedded-guide-full.log` and
`target/s130-initial-embedded-connection.log`). Trust policy (13 unit tests,
seven workflows and 54 local skills), exact brand policy, release-note generator
and Debian validator regressions passed under the hidden foreground runner.
The primary final text audit covered 104 changed/new text files and found no
UTF-8, BOM, CRLF or mojibake errors. `git diff --check` passed.

## Review and owner boundary

PR #298 was published from initial commit
`e6ce912c1161b6b0a6169dd0239e368ddf2b53c7` and attached to the owner chat.
All five delivery project items are in PR review. The manual Codex request is
issue comment 6193435680; no review submission was present before requesting.
Round accounting, final-head hosted checks and unresolved-thread state will be
recorded here after completion. The agent stops before merge; no tag or release
is authorized or created.

## Limits

Evidence is repository/synthetic Windows execution, generated browser rendering
and release-profile guide behavior. Installed-game production behavior, native
producer flush timing, physical disk exhaustion/host ACL changes and deferred
inline/verbose fidelity are not claimed. Linux platform compilation/testing is
verified separately by the final-head hosted check. Source-generation witnesses
sample physical identity and bounded prefix/trailing windows; they are not a
whole-file forensic guarantee against arbitrary same-identity middle rewrites.
The worker checkpoints bounded pending state each accepted turn, so large pending
fights can cause bounded write amplification. No owner field-testing task or
manual repair/import workflow is introduced.
