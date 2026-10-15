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
| #286 | Automatic readiness/refresh/defer/resume and bounded restart preserve the previous complete generation. All scan/finalization/checksum stages yield. Actual API results and exact client/API/locale/character identity feed twelve source groups; priority save remains a request. No player collection action, routine chat or native logger dependency. See lua-verification.md, coverage.md and sixteen collector_addon tests. |
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

## Final gates

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

Pending publication. Review-round count: zero. The final receipt must identify
the exact checked head, review results and unresolved-thread count before handoff.
