# S128 Candidate Evidence

Date: 2026-10-13. Baseline: protected main 05c97041b9d12f5e5c10fae14ba1b80511c5974d.
Issue: #279. Branch: codex/s128-v0180-release. Preparation only.

## Specification

Specify, clarify, both checklists, read-only research, plan, tasks and blocking analysis completed in order. Checklists pass 12/12 each; [analysis.md](analysis.md) records 13/13 covered requirements/criteria across 16 tasks and no findings. Optional agent-context update ran after plan using the explicit S128 path; its after-specify run was deferred until that plan existed. No other hooks are installed for these phases.

## Baseline

Cargo-release 1.1.2 queried successfully. Existing preview rejects missing Highlights with exit 4 (target/s128-notes-red.log). v0.18.0 target remote tag absent at kickoff; no prior matching tracking issue. Existing release/config/workflow authority inspected. No runtime or pinned process change is planned.

## Candidate Checks

Clean preparation commit: 0750772b062947c43b06fa90cedf0188c9b37e1e.
Inspected dry run and effective cargo-release configuration (verify true;
tag/push/publish false), then executed the existing local rollover. Identity
commit b5a2ea4bde86e7fb82da4f4866456429b2dbe912 changes exactly six files.
Cargo.lock changes only the root package version. v0.18.0 and 2026-10-13 are
consistent across package, badge, bundled snapshot and capture fixture. S125-S127
records are preserved verbatim and earlier release sections are byte-identical.
Fresh Unreleased remains. Three Highlights pass the existing count/word gate.

| Gate | Result |
| --- | --- |
| Format | Pass |
| All-target/all-feature Clippy, warnings denied | Pass |
| cargo test --all --locked (default harness) | 1117 passed |
| Documentation/render/brand/trust policy fixtures | 181 passed |
| Release-note and Debian validator fixtures | Pass |
| Spelling, trust and exact brand adoption | Pass |
| mdBook examples, build/link checks and site policy | Pass |
| Complete headless rendering and 200 percent zoom | Pass |
| Optimized release build with bundled documentation | Pass |
| Identity/history/lockfile and text hygiene | Pass |

Local render discovery required an explicit installed Chromium path. The first
browser attempt exposed an empty page-target startup race; the complete existing
matrix passed with installed Chromium1234 and bounded local page readiness. No
repository script or assertion changed. This is build/render evidence, not
installed application or real-game behavior.

## Integration

Official PR #278 opened on 3158b96cef46c7f003da766ecdacededa09d44d2.
Automatic review was round one. Both authorized review rounds completed on the corrected implementation.
The candidate is unmerged and untagged; no package publication or field check occurred.

## First Review Correction

Automatic opening review completed on 3158b96 at 2026-10-13 11:30 UTC. Finding
4258054763 identified premature closure of the only release tracker. Parent
#277 now tracks publication and remains open after merge; child #279 owns
S128 candidate preparation and is the PR closing issue. Specification, contract,
plan, ledger, indexes and changelog agree. Publication authority is unchanged.
Correction17769999351ca7fdbea420e5cc1c1df870cad895 was pushed. Reply4258083813
answered the finding and threadPRRT_kwDOTU-tqc6r1g8X is resolved.

## Review Closure and Final Handoff

Exactly two review rounds were used: automatic opening review on3158b96 and
one security-scoped manual request6144608594 on1776999. The final request
completed at 2026-10-13 11:35:08 UTC with no major issues, comment6144635009
and a bot thumbs-up. No third round is requested. The only finding4258054763
is fixed, answered and resolved. Zero unresolved threads and zero open PR-ref
CodeQL alerts were confirmed live.

Corrected implementation17769999351ca7fdbea420e5cc1c1df870cad895 passed all nine
distinct required checks: Windows/Linux checks, dependency review, build and
validate, analyze Rust, CodeQL, proposed policy, closing-issue linkage and
protected trust policy. Repeated issue-linkage jobs also passed; Pages deployment
is intentionally skipped on a PR. Windows CI included the production build.

All16 tasks are complete and the official owner handoff is prepared. This last
commit changes only spec/task/receipt text, with no externally reviewed package
identity or implementation change. Checks on its own final pushed head are awaited
before the owner ping and recorded in the PR body. Owner merge and separately
authorized tag/package publication remain later steps. Parent #277 remains open
through publication; the candidate PR closes only #279.

## Owner Merge and Subsequent Publication

On 2026-10-13 the owner squash-merged PR #278 at
d88151c3ba1b5523fc29a22f7fcd8b9cc8491289, closing #279. The merged tree
exactly matches reviewed candidate 084f67d3ad716957de0ba0a262d2f28c05eacdb6.

After separate owner publication authorization, CI 38395051345, docs 38395051279
and CodeQL 38395051199 passed on that exact main commit. Annotated tag object
398ca1a386774a3de3758c155fd29157a893d71a points to it.
[Release 38396415318](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38396415318)
passed; v0.18.0 published at 2026-10-13 12:31:53 UTC as latest stable release.
Windows MSI, Linux Debian/AppImage/tarball and SHA256SUMS are available. All five
downloaded sizes and SHA-256 values matched published metadata; each package
also matched SHA256SUMS. Parent #277 is closed and Done.

This subsequent publication receipt does not enlarge the original S128
preparation authority. Post-release housekeeping archives Plan054 and leaves
no active plan. Published tag/assets are immutable. No package installation or
real-game behavior was tested or claimed.
