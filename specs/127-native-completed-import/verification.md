# S127 Verification and Delivery

Date: 2026-10-13. Issue: #272. Plan053.
Branch: codex/s127-native-completed-import.
Base: 9bfe9e07644d4611cd29bec3293e33457e4c4d69.

## Local gates

| Gate | Result |
| --- | --- |
| cargo fmt --all -- --check | Pass |
| cargo clippy --all-targets --all-features -- -D warnings | Pass |
| cargo test --all --locked -- --test-threads=1 | 1114 passed (complete local recheck) |
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
suites passed (9 and18 cases), then all1114 tests passed with one harness thread.
The CI configuration and assertions are unchanged. Corrected-head CI remains
pending publication.
