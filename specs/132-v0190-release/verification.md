# S132 Candidate Evidence

Date: 2026-10-15. Baseline: protected main 8cc94d7c169aa85057ec36daf2e8b5f3cc0e1672.
Candidate: #302 under release coordinator #301. Branch: codex/s132-v0190-release.

## Specification and Initial Evidence

The installed specify, clarify, checklist, plan, tasks and blocking analyze workflow completed before implementation. Both requirement-quality checklists pass all 24 items; analysis covers all eight functional requirements with no findings. Optional context hooks were handled by the required plan-context update; no mandatory hook is configured.

The existing release-note preview rejected missing Unreleased Highlights with exit 4 before notes were added. The first sandboxed Git Bash launch failed MSYS initialization; the authorized hidden launcher succeeded outside that sandbox and produced the actual contract rejection. Logs stay in ignored target output.

#284 head 369767958cdb022e678ebdb4cb4a81a0d928fda3 was compared directly with baseline Cargo files: only rmcp, serde_json and tokio-util change. That exact Cargo content is incorporated, with no runtime source edits.

## Preparation Gates

The release-note contracts, all 184 documentation/render/brand/trust policy tests, repository trust scan, exact brand policy, spelling, mdBook examples/build and generated-site policy pass. The first policy run rejected Plan058's lifecycle enum; correcting it to the existing Active / In Progress schema fixed the record without weakening policy.

Notes contain four Highlights bullets within the 120-word budget. Current release pages describe one combined PixelBeacon package, all four native v15 profiles and the unchanged artifact matrix. Plan058, indexes, ledger and agent context identify S132 consistently.

## Candidate Gates

Preparation commit: 51cd49a471c19bd42ef1e21d1eb323fb5da8f8d6. Installed cargo-release 1.1.2 dry run showed only the expected replacements. Authorized execution produced identity commit 9da8426f23431bf51ad7f1c6746af586faeaa2c7, changing the six expected rollover files. Configuration remains tag=false, push=false and publish=false.

Local candidate gates passed on that identity:

- Formatting and locked all-target/all-feature Clippy with warnings denied.
- All 1,207 Rust tests across 54 suite results, zero failures or ignored tests.
- Release-profile Windows binary with the complete generated documentation site.
- All ten release-profile embedded documentation contract tests.
- All 184 documentation/render/brand/trust policy tests; trust and brand scans.
- Spelling, mdBook examples/build, generated-site policy and headless rendered diagram, layout, syntax, figure and table checks.
- Release-note generator contracts, four-bullet release excerpt and Debian validator contracts. Real dpkg package cases require Linux and remain covered by hosted Linux CI.

An independent metadata audit confirms v0.19.0 / 2026-10-15 across all named identity surfaces, exactly the three intended dependency updates, no other lockfile churn, unchanged release history from v0.18.0 onward, unchanged addon/runtime/pipeline content and clean UTF-8/no-BOM/LF text across all 25 changed files. The local and remote target tag are absent; remote main remains the baseline.

Logs and generated audit output are ignored under target.

## Pull Request and Review

Official [PR #303](https://github.com/h8rt3rmin8r/eso-weave/pull/303) opened on reviewed candidate 8adc718ee73835ef1e30c530d7c0f944f60bc306. Its automatic [Codex round 1](https://github.com/h8rt3rmin8r/eso-weave/pull/303#issuecomment-6199514253) completed at 18:38:46 UTC without findings. The bot's positive reaction, empty review submissions and empty inline-thread list confirm the clean disposition. One of two authorized rounds was used; no second round is needed for delivery-record-only changes.

All nine hosted checks pass on the reviewed candidate: Windows/Linux checks and dependency review ([CI](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38787105447)), [documentation/browser validation](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38787110060), [protected trust policy](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38787105328), [issue linkage and proposed-policy tests](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38787105392), and [CodeQL](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38787105577). Deployment is correctly skipped for the PR.

The final delivery commit updates only this slice's status/task/evidence records and Plan058. Package identity, dependency graph, runtime, tests, policies and published-guide source remain identical to the reviewed candidate. Its exact head and successful checks are recorded in the PR conversation before owner handoff. Final metadata/text policy is revalidated; no additional review round is needed for these records.

## Boundary

Owner final review/merge and separately authorized tag/publication remain outstanding. #302 closes on merge; #301 stays open through later publication. #284 remains open until equivalent content is confirmed on main. No package installation or real-game behavior is tested or claimed.
