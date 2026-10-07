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

## Governed candidate rollover

Clean preparation commit: 3779b91. Installed cargo-release 1.1.2 dry run passed;
reviewed replacements and executed `cargo release 0.17.4 --execute --no-confirm`.
Generated identity commit: d347031 (release: v0.17.4). Cargo.toml, Cargo.lock,
CHANGELOG.md, README.md, docs/src/README.md and the S073 capture fixture agree
on v0.17.4. Date-bearing changelog/docs fields agree on 2026-10-07.
Unchanged release.toml disables tag, push and publication. Logs:
target/s124-release-dry-run.log and target/s124-release-execute.log.
Candidate gates are rerun because crate identity, import fixture and embedded
public documentation changed through the governed command.

All candidate gates passed again on v0.17.4: 1,041 Rust tests, 180 policy
fixtures, release binary and generated documentation, all five rendered receipt
sentinels, spelling, examples, trust/brand/site policy and release-note/package
validator tests. Candidate notes meet the two-bullet bounded Highlights contract.
Remote/local v0.17.4 tags are absent; latest published release remains v0.17.3.

## External review

Official PR: https://github.com/h8rt3rmin8r/eso-weave/pull/264, published and
attached with closing references for #261, #262 and #263. Source update PRs
#239, #240, #247, #257 and #258 remain open for post-owner-merge supersession.

Exactly two Codex rounds reviewed candidate 6429f28389c110dc606854e9549695b5ea8f1e38:

1. Automatic PR-open review completed 2026-10-07 19:49:49 UTC, with no findings
   and a positive PR-body reaction.
2. Final authorized manual request used `@codex security review` and explicit
   dependency/authority/import security scope. The connector labeled the result
   Code Review and completed 19:52:57 UTC, reporting no major issues:
   https://github.com/h8rt3rmin8r/eso-weave/pull/264#issuecomment-6045656844.

No inline comments, review submissions or unresolved threads were present.
No additional round will be requested. Hosted CodeQL and dependency-review
checks provide separate scanner results; the connector's generic label is not
presented as an independently reported specialized security-review result.

## Hosted CI and completion receipt

The reviewed candidate 6429f28389c110dc606854e9549695b5ea8f1e38 passed all nine
non-deployment checks: both Linux/Windows format/lint/test/release-build jobs,
dependency review, CodeQL analysis/result, documentation build/render/policy,
protected trust boundary, proposed linkage policy and closing-issue enforcement.
Pages deployment is intentionally skipped for PRs. No CI corrections were needed.

CI run: https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37677255917
Docs run: https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37677255931
CodeQL run: https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37677255937

All fifteen tasks are complete. This final completion commit changes only the
S124 task/evidence records; implementation, dependency graph, workflow pins,
release identities and public documentation are identical to the reviewed head.
Final-head hosted checks remain the handoff gate and are rechecked after this
receipt is pushed. No third review request is authorized or sent. No before/after
implementation hook is registered. Owner final review/merge remains outstanding;
Plan050 stays Active until that merge. No tag or package publication occurred,
and original dependency PR closure remains post-merge housekeeping.

Automated repository and CI evidence is complete within this slice. Installed
and game-session behavior was not observed and is not claimed as proven.
