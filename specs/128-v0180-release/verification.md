# S128 Candidate Evidence

Date: 2026-10-13. Baseline: protected main 05c97041b9d12f5e5c10fae14ba1b80511c5974d.
Issue: #277. Branch: codex/s128-v0180-release. Preparation only.

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

Official PR, external reviews and final-head CI await publication. The candidate
is unmerged and untagged. No package publication or owner field check occurred.
