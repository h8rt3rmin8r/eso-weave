# S127 Verification and Delivery

Date: 2026-10-13. Issue: #272. Plan053.
Branch: codex/s127-native-completed-import.
Base: 9bfe9e07644d4611cd29bec3293e33457e4c4d69.

## Local gates

| Gate | Result |
| --- | --- |
| cargo fmt --all -- --check | Pass |
| cargo clippy --all-targets --all-features -- -D warnings | Pass |
| cargo test --all --locked -- --test-threads=1 | 1117 passed (complete local recheck) |
| Policy fixture suites (docs/render/brand/trust) | 181 passed |
| Spelling / trust / brand | Pass |
| mdBook examples / build / links / generated-site policy | Pass |
| Complete headless render matrix and 200 percent zoom | Pass |
| Release-note / Debian validator fixtures | Pass |
| cargo build --release --locked --bin eso-weave | Pass |
| Isolated 43 MB private-file import/reimport | 205 new, then0 new/205 existing; source unchanged |
| Text/encoding and diff sanity | Pass (UTF-8 without BOM, no mojibake) |

Native fixtures cover exact retention, source/copy identity, multiple runs, actor
scope, separate hit/overflow, unknown context, UTF-8/BOM/CRLF/escapes, malformed
known layouts, unsupported versions, nesting/token/line limits, incomplete tails
and fights, stable source reads, inspected-source change, immutable store tampering,
schema1-5 migration, collision rollback, mixed order, deletion/reimport and backup.
Desktop journeys cover selection, inspection, import and factual details without a
catalog at420/760 pixels. Current facts are repository/private-file evidence.

## Publication and review

Automatic push and official PR are explicitly owner-authorized. Official [PR #276](https://github.com/h8rt3rmin8r/eso-weave/pull/276) was
published from ff7dd6c6bae5b7a5ae925399b05886d6e1a24bf6. Automatic initial
review was round1 and completed on ff7dd6c at09:48 UTC with one P2
duplicate-store-validation finding. The corrected source-kind dispatch is covered
by a mixed-source worker regression; reply/resolution and round2 security review
follow publication of the correction. One further @Codex round is authorized.
No third review request is permitted. Every finding must be addressed/replied to
and each review thread resolved. Exact final-head CI must be green before owner
final review and merge handoff. The agent does not merge, tag or release.

Initial ff7dd6c CI: all9 checks passed, including Windows/Linux default-harness
Rust checks and CodeQL; Pages deploy is intentionally skipped on a PR.
After corrections, two local default-harness reruns reached OS10053 host-aborted
loopback reads in unchanged documentation/local-service suites. Both focused
suites passed (9 and18 cases), then all1117 tests passed with one harness thread.
The CI configuration and assertions are unchanged. Corrected-head CI remains
pending publication.

Round2 request: issuecomment6143163204, security-scoped manual request on5043e23,
completed10:09 UTC with one P2 context-work finding4257335544. The correction
adds whole-file work reservations before snapshot/scanning and focused regressions.
Commit, reply/resolution and green corrected-head CI follow full local gates.
No more review rounds will be requested.

Final-round local gates pass: fmt, all-target/all-feature Clippy, all1117 cases
with one harness thread,181 policy fixtures, spelling, mdBook build/links/site,
complete headless render matrix and bundled release build. The43MB source still
imports205 and repeats as0 new/205 existing under the cumulative work bound;
source bytes remain unchanged and the isolated private store was removed.

## Review closure and owner handoff

Both authorized rounds are complete (automatic code review, then the single
security-scoped manual request6143163204). No third round was requested.

| Round | Finding | Correction | Reply | Thread |
| --- | --- | --- | --- | --- |
| 1 |4257130239, duplicate addon detail validation |5043e2373fa789efd495323e97aed0ec3f87ba1b, recorded-source dispatch |4257295279 |PRRT_kwDOTU-tqc6rzQUh, resolved |
| 2 |4257335544, cumulative context amplification |5672cae350af5e377e1b35a39784c3ae0b3d5950, pre-snapshot whole-file work bounds |4257535848 |PRRT_kwDOTU-tqc6rzwnX, resolved |

The final implementation head5672cae350af5e377e1b35a39784c3ae0b3d5950 passed
all9 distinct CI checks: CodeQL, Test proposed policy, Check closing issue, checks (windows-latest), dependency review, build and validate, analyze Rust, checks (ubuntu-latest), Enforce protected trust policy.
This includes default-harness Linux and Windows Rust validation and CodeQL.
Repeated issue-linkage jobs also passed; Pages deploy is intentionally skipped
for a PR. Both review threads were read live and confirmed resolved.

The remaining commit records this receipt only. Its exact pushed-head checks are
reverified live before owner handoff. No Rust or published documentation behavior
changes after the verified implementation head. Owner final review and merge are
the remaining authorized ritual; the agent does not merge, tag or release.
