# S124 Verification

## Baseline (2026-10-07)

Main e65c00c4cf8b712ca8b6a4d4877dde63bb7a1b59 resolves rustls 0.23.41,
which falls within GHSA-2mjx-qc3c-rqvc's affected range (patched in 0.23.45).
The existing focused suites passed: collector_addon 4, data_addon 15,
database_query 9, encounter_addon 38 and local_service 17 (83 total).
Logs: target/s124-baseline-focused.log.

Coverage gaps were explicit initialized MCP protocol compatibility and canonical
base64 padding/trailing-bit behavior. Added one focused contract test for each,
before dependency updates. Both expanded suites passed on the baseline:
database_query 10 and local_service 18. Existing default-client coverage remains.
Logs: target/s124-baseline-regressions.log. No production changes were required.

## Scoped updates

All ten targets resolve exactly as specified in research.md. Coordinated
transitive upgrades are rustls-webpki 0.103.15, thiserror-impl 2.0.21 and
mlua-sys 0.13.0; tokio-util adds its declared libc edge. Older base64/getrandom
versions remain for legitimate transitive consumers. The unrelated tempfile
getrandom edge remains 0.4.3, and locked Cargo tests accept that selection.
No other package versions changed. No advisory-affected rustls version remains.

All five upgraded focused suites passed (85 tests), including both new contracts
and exact Lua scalar import tests. No production compatibility changes needed.
Log: target/s124-upgraded-focused.log.

Both CodeQL pins resolve to official v4.38.2 commit
2892aa5e19bbd11bc0cff5427e3b750a04d9e3c2, verified through the annotated
upstream tag. Triggers, permissions and credential settings are unchanged.
Plan049 is archived with chronological merge/publication/repair evidence;
Plan050, both indexes, ledger and Plan048 reference agree.

## Preparation gates

Formatting, Clippy (all targets/features, warnings denied), all locked Rust tests
(1,041 passed), and the release-profile binary with bundled documentation passed.
The combined documentation/render/brand/trust fixture suite passed 180 tests.
Trust scan passed for seven workflows and 54 local skills; brand and generated
site policy, book examples, spelling and rendered-documentation checks passed.
Release-note generator tests, Debian validator tests and bounded Unreleased
candidate notes passed. Logs: target/s124-gate-*.log.

The first docs fixture run rejected the plural issue-reference wording in the
active ledger entry. Corrected the evidence to explicit singular issue references;
all fixtures and actual site policy then passed without changing policy code.
Touched files passed strict UTF-8 without BOM, LF and mojibake checks.
