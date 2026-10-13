# S126 Verification Receipt

Date: 2026-10-13
Branch: codex/s126-retained-recording-sessions
Issue: [#273](https://github.com/h8rt3rmin8r/eso-weave/issues/273)
Base: 7939729f302611f911c39216aea0c00f5dfe3ef1

## Specification and implementation

All spec-kit stages completed in order: specify, clarify, requirements/domain
checklists, plan with read-only research, tasks, blocking analyze and implement.
Requirements quality: 10/10; retention checklist: 12/12. All 13 requirements and
seven success outcomes are covered. Post-design archive-version and admission
refinements retain scope and constitution compliance.

The initial five retention regressions failed on S125 before implementation.
Additional red regressions reproduced malformed identifier disclosure and extra
legacy selection arguments; the implemented corrections pass.

## Local gates

| Gate | Result | Local log |
| --- | --- | --- |
| cargo fmt --all -- --check | PASS | target/s126-review-fmt.log |
| cargo clippy --all-targets --all-features -- -D warnings | PASS | target/s126-review-clippy.log |
| cargo test --all --locked | PASS, 1088 tests | target/s126-review-full-tests.log |
| cargo build --release --locked --bin eso-weave | PASS | target/s126-review-release-build.log |
| Policy fixture suite | PASS, 191 tests | target/s126-policy-tests.log |
| Documentation spelling, brand and trust policies | PASS | target/s126-spelling.log, s126-brand-policy.log, s126-trust-policy.log |
| mdBook test and build, including link checks | PASS | target/s126-mdbook-test.log, s126-mdbook-build.log |
| Complete documentation policy | PASS, including review records | target/s126-docs-policy.log, s126-review-docs-policy.log |
| Headless Chrome rendering sentinels | PASS | target/s126-docs-render.log |
| Release-note and Debian validator fixtures | PASS | target/s126-release-notes-fixtures.log, s126-validate-debian-package-fixtures.log |
| git diff --check | PASS | Direct repository check |

An initial full run stopped at one shared-addon emitted-version assertion; that
assertion now names controller2 and retains catalog isolation. Documentation
fixtures initially required restoring the explicit clear-all command at its
point of use; its deletion-only meaning remains. Both gates passed after repair.

## Publication and review

Official [PR #275](https://github.com/h8rt3rmin8r/eso-weave/pull/275) is published
and closes issue #273. Initial source head
4d72285046c2c22e2e70850b7518e12461ef1368 passed all nine remote checks:
Linux/Windows CI, documentation, CodeQL build and analysis, trust policy,
proposed/closing issue-link policy and dependency review. Documentation
deployment was correctly skipped for the pull request.

Round 1 completed automatically on 2026-10-13 with a thumbs-up and no findings.
The second and final request was explicitly security-scoped:
[request](https://github.com/h8rt3rmin8r/eso-weave/pull/275#issuecomment-6139506362).
The connector reported its result as Code Review and raised one P2:
[migration rollback](https://github.com/h8rt3rmin8r/eso-weave/pull/275#discussion_r4255360060).
The finding reproduced a schema4 upgrade surviving a rejected import.

Migration helpers now require the caller transaction. Schema1-4 migration,
record/snapshot preflight and append share one commit/rollback boundary.
No-op deletion also rolls back any migration. A red schema4 regression became
green for both record and snapshot collisions; existing schema1-3 migration
fixtures now verify byte-identical rejected imports before successful migration.
Focused storage suites passed 102 tests. Post-review full gates passed 1088
Rust tests, fmt, Clippy and release build.
Logs: target/s126-review-migration-red.log, s126-review-storage-tests.log and
the post-review gate logs above.

The correction was committed and pushed as
3e8ef8cbdf8954572a65cdeb9ae4d5bbe3a716a0.
The [review reply](https://github.com/h8rt3rmin8r/eso-weave/pull/275#discussion_r4255520338)
explains the correction and regressions. Direct thread inspection confirmed
the sole thread resolved and zero unresolved findings.

All nine distinct checks passed on the corrected implementation head:
[Linux/Windows CI](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38358357488),
[documentation](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38358357386),
[CodeQL](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38358357457),
trust policy, issue-link policy and dependency review. CodeQL annotations
were empty and documentation deployment was skipped for the PR.

Both authorized review rounds are complete; no third request was issued.
This receipt commit changes task/evidence records only. Its implementation
is identical to the corrected head above; final-head checks are reverified
live before owner handoff. Plan052 and issue #273 remain in PR review until
owner merge. Owner review and merge remain the final boundary.
No merge, tag, release, native-log implementation or owner field-verification
request is included. Installed/game behavior remains unverified.
