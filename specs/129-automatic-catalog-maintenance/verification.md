# S129 Verification and Handoff

Date: 2026-10-15. Branch: codex/s129-automatic-catalog-maintenance.
Baseline: 43d8c557c0b3e346fea0cbc9fc1a288af854234a.
Scope: #285, #286, #287, #288 and #291. The owner authorized push/PR and up to
two Codex review rounds; the owner retains final review and merge.

## Specification and acceptance

The specify, clarify, checklist, plan, tasks and blocking analyze sequence passed
before implementation. analysis.md records the blocking gate and the scoped
constitution amendment. Requirements and data-integrity checklists passed.

| Issue | Delivered behavior and acceptance evidence |
| --- | --- |
| #285 | One five-file PixelBeacon package and authoritative version/API declaration. Transactional automatic migration covers neither/legacy beacon/legacy Data/both/current/interrupted/conflicting roots. Exact inventory, regular confined reads, no-replace retirement, rollback and typed cleanup/recovery preserve unrelated files and SavedVariables. Legacy encounter source is a test fixture only. Whole actual beacon execution covers optional-module failures without changing supported signal values. See package-verification.md, tests/beacon.rs, tests/s129_package.rs and package unit tests. |
| #286 | Automatic readiness/refresh/defer/resume and bounded restart preserve the previous complete generation. All scan/finalization/checksum stages yield. Actual API results and exact client/API/locale/character identity feed twelve source groups; priority save remains a request. No player collection action, routine chat or native logger dependency. The current PixelBeacon.lua output and retained legacy EsoWeaveData.lua are read separately without changing either file. See lua-verification.md, coverage.md and seventeen collector_addon tests. |
| #287 | Real packaged baseline plus an independent complete capture produces queryable facts. Compatible accumulation retains disjoint/overlapping scopes, all parents, locales and supported earlier facts. Acquisition lineage does not imply coverage loss or semantic change. Genuine regressions/integrity failures still reject publication. Exact channel/API/game domains and retained historical lookup remain available. tests/catalog_automatic.rs, catalog compiler/pipeline/import/update suites and coverage.md map actual sources and gaps. |
| #288 | A bounded application-lifecycle coordinator and serialized worker discover actual roots and pre-existing/later complete inputs without a window or acknowledgement. Stable semantic identity, retry, periodic reconciliation and restart recovery handle missed notifications/partial writes. Verify/open precedes cancellation-fenced atomic selection. Already committed changes publish coherently even if stop arrives afterward; last-good and historical readers remain available. Runtime tests plus catalog_automatic and data_maintenance integration fixtures cover the effective database and identity. |
| #291 | Independent typed booleans default false and persist immediately through normal settings. Shared per-root eligibility separates compatible presence from package mutation ownership. Two accessible direct controls are inactive without recognized presence. Stops cancel independent generations; failed disable remains stopped and failed enable cannot claim durable activation. Exact startup-config-only transactions also support compatible manual packages without adopting their manifest/modules. API state distinguishes requested preferences, application persistence, on-disk startup match and unknown loaded Lua. Pure model, settings, package/Lua/runtime and headless UI fixtures cover these boundaries. |

## Test-first and audit evidence

- Package and preference regressions initially failed on missing combined-package
  and typed preference interfaces (target/s129-package-red.log and
  target/s129-preferences-red.log).
- The independent capture regression failed against the real packaged baseline
  on false coverage removal (target/s129-catalog-red-workspace-temp.log).
- The old manual collector failed six of seven automatic regressions
  (target/s129-lua-red.log). The completed collector passed all sixteen current
  tests (target/s129-ui-lua-api-final.log).
- A postcommit cancellation regression demonstrated that the new selection could
  remain unpublished to consumers. The fix passed the same regression
  (target/s129-runtime-postcommit-red.log and
  target/s129-runtime-postcommit-green.log).
- The first full run found obsolete separate-Data/reload fixtures and the API
  field inventory mismatch. These were corrected against current behavior, with
  meaningful geometry, ownership and field-inventory checks retained. Socket
  failures in the sandbox require the final full suite outside that restriction.

## Initial implementation gates

| Gate | Result and evidence |
| --- | --- |
| Rust format | PASS, target/s129-fmt-check-final.log |
| Clippy all targets/features, locked, warnings denied | PASS, target/s129-clippy-final.log |
| Complete Rust suite, locked | PASS: 1,171 passed, zero failures or ignored tests, target/s129-full-final.log. Required loopback tests ran outside the socket-restricted sandbox through the hidden launcher. |
| Documentation, render, brand and trust unit tests | PASS: 183/183, target/s129-policy-all.log |
| Book examples and spelling | PASS, target/s129-book-examples-final.log and target/s129-spelling-final.log |
| Book build/linkcheck and complete generated-site policy | PASS, target/s129-catalog-diagram-build.log and target/s129-catalog-diagram-generated.log |
| Actual headless browser | PASS: all five diagram/layout/syntax/figure/table sentinels, target/s129-catalog-diagram-browser-final.log |
| Repository trust, exact brand assets, release-note and Debian validator contracts | PASS, target/s129-trust-final.log, target/s129-brand-final.log, target/s129-release-notes-final.log and target/s129-debian-contract-final.log |
| Text integrity and patch hygiene | PASS: all 99 staged text files checked for UTF-8, BOM, CRLF and mojibake; staged diff --check clean, target/s129-text-integrity-final.log. |
| Release binary and embedded guide | PASS: release binary built with the pinned generated guide; release-profile documentation tests also pass, target/s129-release-build-final.log and target/s129-embedded-guide-final.log. |

The full Rust pass includes all thirteen automatic Catalog cases, all seven
runtime cases, the final dormant-token model corrections, sixteen collector
tests, package/configuration transactions and keyboard/absence UI paths.
The final API inventory includes the additive observation in both canonical
contract and published inventory; the earlier inventory-only failure is fixed.

Documentation authority changes preserve all twenty specification units, six
safety identifiers/destinations, historical baseline blobs and obligation IDs.
Only current combined-lifecycle excerpts, renamed lifecycle tests, two affected
ownership/log authorities and the Catalog diagram narrative changed. Baseline
projection checks proved the two corresponding pin updates exactly match those
scoped changes; validator enforcement and negative mutation tests remain active.
Implementation extension hooks were inspected: no after_implement hook exists.

## Scope and evidence limits

No installed-game or owner field behavior was exercised. Lua tests use public-API
doubles and real emitted envelopes; desktop tests use temporary filesystem roots,
the actual packaged baseline and synthetic independent inputs. They do not prove
ESO save latency, exhaustive API visibility, engine-owned operation latency or
installed package behavior. coverage.md names source gaps without reducing any
retained product capability or inventing metadata for unknown IDs.

The future native watcher (#289/#290), complete downstream transfer (#293), full
Encounter History viewer/menu retirement (#292) and final whole-program
documentation/assets audit (#294) remain outside S129. Existing stored history,
completed native-file import and authenticated local reads remain supported.
Native preference/eligibility state does not claim a running native watcher.
No merge, tag, release or game action is included.

## PR and owner handoff

[PR #295](https://github.com/h8rt3rmin8r/eso-weave/pull/295) is open and attached.
The five issue Project items have Slice S129 and Stage PR review.

Round 1 automatically reviewed d7f08e6bf887721f3eb7a94b3ab739fca55e1d03
and completed on 2026-10-15 with two P2 findings:

1. Old two-addon uninstall wording described narrower removal than the unified
   lifecycle performs. Both routes now share one confirmation and action with
   accurate complete-package removal and retained-data wording. The UI
   regression failed first, then passed for both routes, cancellation,
   coalescing and actual byte preservation (target/s129-review1-uninstall-ui-red.log
   and target/s129-review1-uninstall-ui-final-green.log).
2. Beacon install/removal left the compatibility lifecycle/API cache stale.
   Both operations now re-inspect it; activation also refreshes it after
   background migration. The model/API regression reproduced Not installed
   after successful install, then passed after correction
   (target/s129-review1-cache-red.log and target/s129-review1-cache-green.log).

Hosted first-head Linux CI and CodeQL passed. Documentation CI exposed a
DejaVu-specific Catalog diagram overflow despite the Windows browser pass.
Labels now wrap to at most eighteen characters per visible line with the same
24-unit font, canvas, owning nodes, padding and semantic enforcement. Local
build/policy/163-unit/browser gates pass again (target/s129-linux-font-*.log).
Hosted DejaVu validation passed on 7e3355e88c78becfaee36874583dff9c946b50c0;
Linux Rust checks also passed on that head.

The correction passes formatting, all-target/all-feature Clippy with warnings
denied, the full locked Rust suite (1,172 passed, zero failures/ignored tests),
and release compilation. Evidence: target/s129-review1-fmt-check.log,
target/s129-review1-clippy.log, target/s129-review1-full.log and
target/s129-review1-release.log. The nine release-guide tests pass in
target/s129-review1-embedded-guide-recheck.log. The first correction-profile run
had one unexpected POST status in the unchanged documentation-service test;
both its isolated rerun and the unchanged nine-test rerun passed. The initial
failure log is retained (target/s129-review1-embedded-guide.log); its cause was
not established, and no assertion, timeout or server code was changed to obtain
the pass. Both round-1 findings were answered with the correction commit and
their review threads were explicitly resolved.

Round 2, the final authorized round, reviewed
7e3355e88c78becfaee36874583dff9c946b50c0 and completed on 2026-10-15 with one
P1 finding: the saved global name does not preserve ESO's on-disk filename.
The current PixelBeacon manifest saves PixelBeacon.lua, while the desktop
previously watched only legacy EsoWeaveData.lua. The correction reads both
files automatically, legacy first and current second, with independent
stable-input and retry state. Existing files remain untouched; historical
encounter readers retain their original source. No new addon ingress or
game action is required. The current-only cold-start regression failed first
(target/s129-runtime-filename-red.log).

Windows CI on the same head exposed background journal recovery racing an
active package uninstall. Mutation and recovery now share a per-canonical-root
serialization boundary; deterministic concurrency coverage supplements the
original runtime regression, which remains intact. External ownership,
link and filesystem race checks remain required.

The current-only regression now passes. The ten runtime unit cases also verify
compatible dual-source union, fresh-service restart/no-op, partial input in
either source without blocking its peer, later natural-save metadata bypassing
only that source's backoff, independent periodic reconciliation and coherent
publication if cancellation follows the first commit. The original Windows
primary-removal runtime regression remains intact and passes. The deterministic
uninstall/recovery race failed before the lock and passes afterward. Evidence:
target/s129-review2-package-green.log,
target/s129-review2-maintenance-integration-green.log,
target/s129-review2-runtime-filename-green.log and
target/s129-review2-lifecycle-red.log. The manifest-stem Lua file model parses the
actual emitted envelope, preserves legacy bytes and reloads current data; this
is synthetic evidence, not an executed ESO save.

## Final correction gates

| Gate | Result and evidence |
| --- | --- |
| Rust format | PASS, target/s129-review2-fmt.log |
| Clippy all targets/features, locked, warnings denied | PASS, target/s129-review2-clippy.log |
| Complete Rust suite, locked | PASS: 1,182 passed, zero failures or ignored tests across 50 suites after the final package and documentation socket corrections, target/s129-review2-full.log. Required socket tests ran unrestricted through the hidden launcher. Earlier 1,180/1,181-test passes are retained in target/s129-review2-initial-full.log and target/s129-review2-inspection-full.log. |
| Documentation, render, brand and trust unit tests | PASS: 183/183, target/s129-final-policy.log |
| Brand assets and repository trust | PASS, target/s129-review2-brand.log and target/s129-review2-trust.log |
| Spelling and book examples | PASS, target/s129-review2-spelling.log and target/s129-review2-book-examples.log |
| Book build/linkcheck and generated-site policy | PASS, target/s129-review2-book-build.log and target/s129-final-generated.log |
| Actual headless browser | PASS: all five sentinels, target/s129-review2-browser-unrestricted.log. The sandbox denied Chrome's crashpad named pipe; the unchanged check passed unrestricted. |
| Release binary and embedded guide | PASS: target/s129-review2-release.log, target/s129-final-embedded-connection.log (one accepted-socket regression) and target/s129-review2-embedded-guide-full.log (all nine documentation integration tests). An earlier library-only filter selected zero tests and is not counted as verification. |
| Text integrity | PASS: all 102 changed/relocated text files, including three retained legacy fixtures, strict UTF-8 without BOM, LF and no mojibake, target/s129-review2-text-integrity.log; git diff --check is clean. |

Correction 6248a692db9b37f9a6fd6053a72d964e7beb9757 is published. The round-2
finding has a response naming the correction and its regression evidence; its
thread is explicitly resolved. Direct review/thread reads confirm exactly two
Codex review submissions and zero unresolved threads.

Hosted Windows CI on that correction passes the deterministic journal-race test
but still fails the original primary-removal scenario at uninstall, now with
PermissionDenied (OS error 5) rather than deleted-journal cleanup. This reveals
a separate inspection-versus-retirement contention path. A deterministic
regression held the actual production bounded manifest handle through the
public presence-inspection helper and reproduced the same OS error 5
(target/s129-review2-inspection-red.log). Public presence/status/geometry
inspections now share the per-root lease; internal mutation/configuration
readers reuse that lease without recursive acquisition. Stable-read sharing,
identity, ownership and external-race checks remain unchanged. The lease covers
bounded package I/O, never Catalog compilation. All 169 focused cases pass
(target/s129-review2-inspection-green.log), including the original runtime test
and both concurrency regressions. Its full-suite pass contained 1,181 tests.
Exact final-head CI remains required before handoff.

The next release-profile guide run again failed an unchanged HTTP assertion
(browser_failure_leaves_the_service_reusable, retained in
target/s129-review2-embedded-guide-second-failure.log). Investigation found
that accepted sockets can inherit the nonblocking listener mode on Windows,
while the handler assumes bounded blocking reads. The
[Winsock accept contract](https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept)
documents inherited socket properties; the [pinned Rust Windows accept implementation](https://github.com/rust-lang/rust/blob/1.96.0/library/std/src/sys/net/connection/socket/windows.rs#L197-L204)
returns that socket without resetting its mode. A new cross-platform regression forces
that accepted mode and sends valid request bytes after 50 ms. It failed before
correction with no valid HTTP response
(target/s129-documentation-nonblocking-response-red.log) and passes after the
handler explicitly restores blocking mode
(target/s129-documentation-nonblocking-green.log). Existing request/connection
deadlines and status assertions are unchanged; the two previously failing
assertions now report their status line for future diagnostics. The old runs
did not record their responses, so their exact status/cause is not retroactively
claimed. This bounded correction supports the required embedded-guide gate;
it adds no documentation endpoint or product workflow.

After both Windows package corrections and the documentation socket correction,
the final local gates above pass. The accepted-socket regression also passes in
release profile, followed by all nine original release-guide integration tests.
Hosted checks on the next published correction remain required.
The two authorized Codex rounds are complete. No third review, merge, tag or
release is authorized.
