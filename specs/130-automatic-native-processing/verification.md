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

Initial aggregate verification passed 1188 tests across 53 target/doc-test
summaries with zero failures/ignored (`target/s130-initial-full.log`). Formatting,
all-target/all-feature Clippy and release binary/documentation checks passed.
The release guide passed 10 tests and the connection regression passed 1 test.

Post-publication inspection found permanently disappeared filenames could retain
candidate slots indefinitely. The disappeared_sources_release regression failed
before the fix, then passed after NotFound retired only the in-memory candidate
while retaining its checkpoint. Full gates passed again with 1189 tests
(`target/s130-review-{fmt,clippy,full}.log`), followed by release executable,
10 guide tests and one connection regression. Red evidence is
`target/s129-s130-disappearance-red.log`.

Codex review round 1 found compatibility import could duplicate automatic native
evidence or resurrect deleted native history. Both cases reproduced before the
fix. Shared logical disposition now runs before any insertion, returns the first
retained identity for a copy and rolls back a compatibility batch containing
suppressed evidence. Same-batch duplicate/divergent and mixed new/deleted cases
are covered. Focused tests passed 75/75; final full formatting, all-target/all-feature
locked Clippy and locked tests passed **1192 tests, zero failed/ignored**
(`target/s130-round1-fix-{fmt,clippy,full}.log`). The retained legacy deletion
expectation now verifies deletion intent, unchanged database and source bytes.

After clarifying that contributor contract, 184 combined documentation/render/
brand/trust unit tests passed, as did exact brand policy, trust policy (seven
workflows/54 skills), pinned spelling, book examples/build/link checks, generated
site and all five browser pass sentinels (`target/s130-review-*.log`). Earlier
release-note generator and Debian-validator regressions also passed. Final
review-fix release executable, 10 embedded guide tests and one accepted-connection
test passed (`target/s130-round1-fix-{release,embedded-guide-full,embedded-connection}.log`).

Round 2 reproduced invalid history after an empty source created a public
uninitialized SQLite file. A complete empty schema7 database is now prepared,
closed and synced privately, then published without replacement under the same
root/token commit gate. Omitted/rejected admission leaves the public path absent;
preexisting zero/foreign schemas and publication races preserve existing files.
The real runtime empty-source, disable/absence/read/append journey passed, as did
94 focused tests and final aggregate formatting, all-feature Clippy and
**1195 locked Rust tests, zero failed/ignored**
(`target/s130-final-{fmt,clippy,full}.log`). Final pinned documentation, generated
site and all five browser sentinels passed again. The final release executable,
10 embedded guide tests and one connection regression passed as well
(`target/s130-final-*.log`).

The primary pre-publication text audit covered 104 changed/new text files with
zero UTF-8, BOM, CRLF or mojibake errors. Final modified files use the same strict
encoding and `git diff --check` passes. No timeout or assertion was weakened.

## Review and owner boundary

PR #298 was published from initial commit
`e6ce912c1161b6b0a6169dd0239e368ddf2b53c7` and attached to the owner chat.
All five delivery project items are in PR review. The manual Codex request is
issue comment 6193435680; no review submission was present before requesting.
Round accounting, final-head hosted checks and unresolved-thread state will be
recorded here after completion. The agent stops before merge; no tag or release
is authorized or created.

Round 1 completed at 2026-10-15T13:43:28Z on `e6ce912` with one P1 finding,
review `PRR_kwDOTU-tqc8AAAABSdt_HQ`, thread `PRRT_kwDOTU-tqc6szFLY`.
The compatibility fix and its failing-first/aggregate evidence are recorded above.
Round 2 completed at 2026-10-15T13:56:59Z on `4f354d1`, review
`PRR_kwDOTU-tqc8AAAABSd7vSA`, with one P2 finding in thread
`PRRT_kwDOTU-tqc6szeES`. The empty-store publication correction and failing-first
evidence are recorded above. Both authorized rounds are complete; no third round
is requested. Final thread closure and hosted head checks follow publication of
that verified correction.

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
