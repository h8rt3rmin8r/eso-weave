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

## Owner Merge and Authorized Publication (2026-10-15)

The earlier boundary records candidate-delivery authority. The owner subsequently
merged PR #303 at 20:09:19 UTC as
`629bc92ffb8685a65d4a4ae2b3e793e0f3492569` and explicitly authorized tag push
and v0.19.0 publication after exact-main checks. The merged tree exactly matches
reviewed delivery head `8e5542c018902a6a009de7eb654f4ce7c198e1f1`.

Candidate #302 closed and moved to Done. #284 was closed as superseded after its
three dependency updates were confirmed on main. Local main is synchronized;
merged branch refs are removed. Plan058 is archived in the documentary closeout
for #304. Exact-main [CI 38799657796](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38799657796),
[documentation 38799657994](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38799657994)
and [CodeQL 38799657899](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38799657899)
all passed. Annotated tag object `ad97fbda0a63d2844e457dd7d04edd7db26dcd84`
was created at 20:24:12 UTC and subsequently pushed; it peels to the exact merged commit above. Main is
unchanged. No installed/game evidence is implied by repository or artifact-integrity checks.

## Published Artifact Receipt (2026-10-15)

[Workflow 38801670245](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38801670245)
completed successfully on tag v0.19.0 / commit 629bc92. Verification and both
platform package jobs passed before publication. GitHub Release 414253268,
[v0.19.0](https://github.com/h8rt3rmin8r/eso-weave/releases/tag/v0.19.0), published
at 20:40:52 UTC as the latest stable release, not a draft or prerelease.

All five assets were downloaded. Each size and SHA-256 matches GitHub's release
metadata, and all four package digests match the published SHA256SUMS file:

| Asset | Bytes | SHA-256 |
| --- | ---: | --- |
| eso-weave-0.19.0-x86_64-linux.tar.gz | 14590409 | `b65a443f050baafeb2a777724c16e3dc60f4c79606b5dbf37b189d00cf3620ca` |
| eso-weave-0.19.0-x86_64-windows.msi | 13131776 | `b74e97fbfe80639d7088b90d55951499ffb3a3600e8345c0a25897e85e957906` |
| eso-weave-0.19.0-x86_64.AppImage | 14359032 | `ace24d5634b7df39d00ec9790f6506ce08b14b23cc110e7a8bdeb77d9c5e97f3` |
| eso-weave-0.19.0-x86_64.deb | 9035384 | `d0cb5b2569641ecdd3aff29139a301cb135ae4099a56a3edfc16f9e702be1d9c` |
| SHA256SUMS | 399 | `d565952e8dcef3e0211b71230660f051b4c5f8fc1e03af8972f8b7c60ccc2adf` |

The release body exactly matches the governed Highlights excerpt plus tag-specific
changelog link. Read-only MSI database inspection reports ProductVersion 0.19.0,
ProductName ESO Weave and Manufacturer ESO Weave. The Linux tarball includes the
binary, catalog/catalog.sqlite, README and LICENSE; its catalog and text files
are byte-identical to the tagged source. Audit JSON, downloads and logs remain
ignored under target/v019-release-audit and target/s129-v019-*.

Release coordinator #301 is complete and Done; milestone v0.19.0 is closed.
Candidate #302 is closed and Done; dependency PR #284 is closed as superseded.
#304 tracks the separate documentary archive PR. There is no active feature plan
or newly assigned slice. No package was installed and no game behavior was tested.
The published tag and assets were not moved or replaced.
