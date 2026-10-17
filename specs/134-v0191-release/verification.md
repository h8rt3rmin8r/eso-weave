# S134 Candidate Evidence

Date: 2026-10-17. Baseline main: b7e264cbaeffde9039fe8f5655c84f1de60e6142.
Candidate #315 under release coordinator #314. Branch: codex/s134-v0191-release.

## Housekeeping and specification

The owner merged PR #313 at 05:07:40 UTC as b7e264c. Its tree exactly matches
reviewed head 65501f15e3dda640b1cbf74658bb299ee9c17384. Main was synchronized,
the already-deleted remote branch was pruned and the matching local branch was
removed. Issue #312 is closed and its Project Stage is reconciled to Done.

The installed specify, clarify, release checklist, plan, tasks and read-only
blocking analyze workflow completed before implementation. All 15 written-quality
items pass; all eight requirements map to 15 tasks with no findings. The optional
context hooks are satisfied by the required plan-reference update in CLAUDE.md;
no mandatory hook applies. A read-only release audit found no blocker.

## Initial preparation evidence

The existing release-note generator rejected missing Unreleased Highlights with
exit 4 before notes were added. The command used a hidden noninteractive launcher.
The subsequent preparation and candidate check results will be recorded after
execution; no hosted or installed behavior is predeclared.

## Preparation checks

The four Highlights bullets total 46 words and cover all five S133 fixes. The
Unreleased preview and release-note contract suite pass. Debian validator contracts,
all 184 documentation/render/brand/trust policy tests, exact brand/trust scans,
spelling, mdBook examples/build and generated-site policy pass. The documentation
link checker does not resolve Markdown fragments; generated-site policy supplies
that additional coverage. These checks ran before the preparation commit.

All 18 changed text files pass strict UTF-8 without BOM, LF, corruption and
punctuation checks. Target v0.19.1 is absent locally and remotely. Issue #315 is
natively linked under #314, both appear once in the delivery Project, and #312
was read back Closed/Done. No candidate or hosted outcome is claimed yet.

## Candidate checks

Preparation commit 64a5233 passed the clean-tree dry run. The governed execute
created exactly six identity-file changes; local identity commit c737f53 includes
the standard agent attribution trailer. No tag, push or publication occurred
inside the rollover; configuration remains unchanged with all three disabled.

Observed candidate validation passed:

- Formatting and locked all-target/all-feature Clippy with warnings denied.
- All 1,219 Rust tests across 54 summaries, zero failed or ignored, including all
  13 named S133 regression cases. Required input, ownership/race, transactional
  recovery and retained-data tests remain enabled.
- All ten release-profile documentation contracts and the production Windows
  binary with freshly generated bundled documentation.
- All 184 documentation/render/brand/trust policy tests, exact brand and trust
  scans, spelling, mdBook examples/build and generated-site policy.
- Headless diagram/layout/syntax/figure/table smoke checks, with all five pass
  sentinels and no failures.
- Release-note generator contracts, the four-bullet/46-word candidate excerpt,
  and Debian validator contracts. Linux package-specific evidence belongs to
  hosted Linux CI; local Windows results do not establish Linux installation.

The identity audit confirms 0.19.1 / 2026-10-17 across all six governed surfaces.
Cargo.lock changes only the root application package version; dependency versions
are unchanged. Historical changelog from v0.19.0 onward is byte-identical, and
runtime/tests/assets/addon contracts, pinned scripts/config/workflows and package
matrix are unchanged. All 23 changed text files pass strict encoding/LF/corruption
checks and Git whitespace validation. Logs/audit data stay ignored under target/.

The published release remains v0.19.0; local and remote v0.19.1 tags were absent
at intake. Hosted final-head CI and review results remain pending publication of
the source PR and are not predeclared by these local results.

## Source PR

[PR #316](https://github.com/h8rt3rmin8r/eso-weave/pull/316) opened at reviewed
candidate head 659c78be88fc4e35f174299c9d9efb02bd6e6267. It closes #315 only;
#314 retains later publication. Both issue/Project records reflect this split,
with the candidate in PR review and Slice S134. Hosted completion is recorded
below only after observation; no additional review round was requested.

## Hosted review and candidate handoff

The automatic [Codex code review](https://github.com/h8rt3rmin8r/eso-weave/pull/316#issuecomment-6230122432)
completed on candidate 659c78be88fc4e35f174299c9d9efb02bd6e6267 with no findings
or unresolved inline threads. One automatic review occurred on PR opening; no
additional round was requested.

All nine checks pass on that reviewed candidate: [Windows/Linux and dependency
review](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/39005981432),
[documentation/browser validation](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/39005981493),
[CodeQL analysis](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/39005981586),
[protected trust policy](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/39005981474),
and [issue linkage/proposed policy](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/39005981420).
PR deployment is correctly skipped. The PR-ref code-scanning query has no open alerts.
The full 16-item specification-quality checklist and eight-item release checklist
now both pass; their initial grouped form is preserved in the analysis history.

The final documentary commit updates only S134 status, checklist, task, plan and
evidence records. Application identity, code, tests, dependencies, notes and
published-guide source remain identical to the reviewed candidate. Exact final-head
checks and the final handoff are recorded in the PR conversation before the owner
receives the PR. This record does not predeclare the follow-up check outcomes.

At handoff the PR remains open and unmerged, auto-merge is disabled, the target
v0.19.1 tag is absent locally/remotely, and the latest published release remains
v0.19.0. Candidate #315 remains open in PR review; coordinator #314 and milestone
v0.19.1 remain open for later publication. No installed/game evidence is claimed.

## Original candidate boundary

Owner merge, v0.19.1 tag and publication remain outside this run. #315 closes only
on owner merge; #314 remains open for later publication and asset audit. No package
installation, addon deployment or game behavior is tested or claimed.

## Owner Merge and Separately Authorized Publication (2026-10-17)

The earlier boundary describes the candidate run. The owner subsequently merged
PR #316 at 05:57:27 UTC as `6a3c95220120978a6decc87bc8e1ce7a63de5742` and
explicitly authorized tagging/publication, asset verification and closure of #314
after completion. The merge tree exactly matches reviewed final head
`760d41ee1cde9c1fb15c4b7f94187b94578552a4`. Candidate #315 is Closed/Done;
main is synchronized, with the merged branch removed and stale refs pruned.

Exact-main [CI 39008127095](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/39008127095),
[documentation 39008127068](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/39008127068)
and [CodeQL 39008127097](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/39008127097)
passed before the tag. Annotated tag object `4cf66b776d9dca01482207267dfa8210ed6d05b2`
was created at 06:08:47 UTC and pushed, targeting that exact reviewed main commit.
[Release workflow 39008830108](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/39008830108)
passed its verify, Windows and Linux packaging, and publication jobs.
[v0.19.1](https://github.com/h8rt3rmin8r/eso-weave/releases/tag/v0.19.1) published
as latest stable at 06:25:02 UTC. Fresh release/tag/run reads confirm the annotated
tag still targets the reviewed source and the successful tag-push run built it.

## Published Asset Audit (2026-10-17)

All five assets were downloaded from the published release into ignored local
audit storage. Their byte lengths and streamed SHA-256 digests match GitHub asset
metadata. All four package hashes match the published SHA256SUMS file. The release
body matches the governed four-bullet/46-word excerpt and tagged changelog link.

| Asset | Bytes | SHA-256 |
|---|---:|---|
| `SHA256SUMS` | 399 | `989b85521b3e1c4983e542329807560aedbf08de6e88b2985205dd7a2b55b7c2` |
| `eso-weave-0.19.1-x86_64-linux.tar.gz` | 14600919 | `f861346716de5c06a1f3cc1393cac5f1efc33cddbb2f81dddaa2d50eeb3132a8` |
| `eso-weave-0.19.1-x86_64-windows.msi` | 13144064 | `544054a19066263b007453755f8cb722091a5bb568f782d0f4543f1b0099bcac` |
| `eso-weave-0.19.1-x86_64.AppImage` | 14371320 | `d825b730eef9166a5bd552983109e9a44622289b4584db79bc145960fb32a6f5` |
| `eso-weave-0.19.1-x86_64.deb` | 9046220 | `b981733a816b00878e6c55c3458592f329e81fc786ebcea053f9ebe91ab4a6fb` |

Read-only package inspection passed:

- Windows Installer opened the MSI database read-only: ProductVersion 0.19.1,
  ProductName ESO Weave, Manufacturer ESO Weave. No installer was run.
- Debian ar/control archives parse correctly with all required fields present:
  Package eso-weave, Version 0.19.1-1, Architecture amd64, plus non-empty
  Maintainer and Description. The Debian revision follows existing packaging.
- AppImage and the executable tarball binary identify 64-bit little-endian
  x86_64 ELF. Neither was executed.
- The tarball has exactly its versioned root, executable, catalog directory and
  catalog.sqlite, README.md and LICENSE. Members are regular files/directories
  with no duplicate or escaping paths. Catalog and text bytes match tagged source,
  which remains unchanged on the archive branch. No tar member was extracted.

Coordinator #314 is read back Closed/Done; child #315 is Closed/Done, still
linked to #314. Milestone v0.19.1 closed after fresh issue/PR queries found zero
open assigned items. #317 owns the separate documentary archive and remains
open until its owner-reviewed PR merges. Plan060 is archived; both indexes and
ledgers record completion with no active build plan.

The archive changes only maintainer documentation and Unreleased changelog
history. All 184 documentation/render/brand/trust policy tests pass. No runtime,
dependency, bundled-guide, workflow, packaging, version or published artifact
change accompanies the archive. Package installation, application execution and
game behavior were not tested or claimed.
