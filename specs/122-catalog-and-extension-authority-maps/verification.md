# S122 Evidence and Integration Record

Date: 2026-10-06. Scope: #221 and #223. No production Rust/addon/schema,
transport, workflow, toolchain or release behavior changes.

## Spec-kit sequence

Executed installed `/speckit-specify`, `/speckit-clarify`, `/speckit-checklist`,
`/speckit-plan`, `/speckit-tasks`, `/speckit-analyze` and `/speckit-implement`
instructions in order. Feature creation, paths, planning/tasks setup and blocking
analysis used the installed PowerShell helpers. Optional agent-context updates
followed the plan step; there were no mandatory hooks. Requirements and domain
checklists passed before implementation. Two read-only research agents traced
the current code contracts, consolidated in research/figure contracts.

Analyze: nine functional requirements, 15 tasks, 100 percent task coverage,
zero ambiguities, duplications or constitution conflicts. FR-001/002 map to
T006/007; FR-003/004 to T008/009; FR-005 to T005/007/009/010; FR-006/007 to
T005/010/011/012; FR-008 to T012/013; FR-009 to T014/015. SC-001 through SC-004
map to T005-T014. No unmapped task or critical issue. Proceeded only after PASS.

## Test-first and correction evidence

`target/s122-red.log` records failed new catalog-equivalent and expanded
eight-diagram receipt checks before policy/assets were implemented. Subsequent
policy tests pass. Mutations cover omitted source and generated equivalent
anchors, moving an anchor outside its own adjacent section, absent SVGs, missing
visible trust/channel/rights/read-only labels, exact asset bytes, receipt identity,
zoom sizes, containment and no-script availability.

Initial generated policy rejected missing visible manifest titles; public prose
now names both figures. Initial 320-pixel 200 percent zoom measured expanded
15-unit labels at only 9.375 effective pixels. New assets use 24-unit labels,
full-width nodes and independent side routes, preserving existing viewer behavior.
Final probes measure 33.6 effective source pixels and 15 modal pixels in both
themes. Complete adjacent/no-script equivalents and all containment checks pass.
Visual review of hidden browser-rendered PNGs confirmed the parallel paths and
terminal PTS branch; the longest rollback line was shortened to fit its node.

## Local automated checks

- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --all-targets --all-features -- -D warnings`: PASS.
- `cargo test --all --locked`: PASS, 1039 tests, zero failures/ignored tests.
  Includes local-service, database query, player-state and all safety surfaces.
- Documentation/renderer/brand policy unit and mutation tests: PASS, 166 tests.
- Brand policy, spelling, mdBook examples, build/link checks and generated
  documentation policy: PASS.
- Headless generated-documentation smoke: PASS, 64 paint cells, eight topology
  observations, retained syntax/figure/table checks and four new zoom/no-script
  probes. New maps each have eight nodes, seven/eight edges, 70-unit minimum
  stage gaps, no crossings or shared segments.
- New content projection: `821b3ff67357f6ce600d304b3842f154ee49ef0fecbf6175c7f55a0a0be78ed3`.
  Only DIA-008/DIA-009 are added; existing obligations remain unchanged.
- UTF-8 without BOM, LF, no long dashes/mojibake and `git diff --check`: PASS.

All console children use verified hidden noninteractive launch paths. No
installed application or game-session verification was performed or requested.
Public and bundled documentation use the same owned asset bytes.

## Review corrections and remote integration

Official PR [#254](https://github.com/h8rt3rmin8r/eso-weave/pull/254) was published
at `42a323a` under the owner's explicit push authorization. First Codex round
completed on 2026-10-06 and identified two P2 findings:

- [Portable feature path](https://github.com/h8rt3rmin8r/eso-weave/pull/254#discussion_r4198649693):
  restore forward slashes in `.specify/feature.json`. Installed Spec Kit
  prerequisites resolve the S122 spec, plan and tasks successfully on Windows.
  No Linux/macOS PowerShell run is claimed.
- [Fallback-font label overflow](https://github.com/h8rt3rmin8r/eso-weave/pull/254#discussion_r4198649700):
  wrap every label to at most 22 characters per line, enlarge nodes/canvases and
  preserve minimum 24-unit fonts, parallel routes and terminal branches. New
  dimensions are 400 by 3092 and 400 by 3272. Annotate label ownership and measure
  all label bounds with 10-unit node/canvas padding under forced monospace and
  DejaVu Sans/sans-serif fallback stacks. No font dependency is added.

`target/s122-review-red.log` records failed fallback-receipt rejection coverage
before implementation. Corrected policy tests pass (167), including wrapped
visible labels and rejection of metadata-only substitutes. The full hidden
browser passes all five sentinels, 64 paint cells, eight topology observations,
four zoom/no-script probes and four fallback-font probes. Both maps retain
70-unit stage gaps and zero crossings/shared segments. Spelling, rebuilt book,
generated policy and encoding/diff checks pass. Initial Rust checks remain valid
because no Rust source or dependencies changed in these review corrections.

Both findings received evidence-backed replies and their review threads were
resolved after `6fcf0e6` was pushed. The one authorized second round was requested
in [comment 6022292680](https://github.com/h8rt3rmin8r/eso-weave/pull/254#issuecomment-6022292680).
It completed on 2026-10-06 at 18:01:47 UTC for
`6fcf0e6649dcceb222ce7c9698f4b0ec0fbff30b`, with
[no major issues reported](https://github.com/h8rt3rmin8r/eso-weave/pull/254#issuecomment-6022337375)
and a thumbs-up on the PR body. Exactly two rounds were used. No unresolved
threads or subsequent findings remained at the integration checkpoint.

All nine head-specific checks passed, with the pull-request deployment check
intentionally skipped. Remote evidence for the reviewed implementation:

- [Cross-platform CI run 37507693774](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37507693774):
  Windows and Linux checks, dependency review, tests and bundled-documentation
  release builds passed.
- [Documentation run 37507693727](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37507693727):
  build, generated policy and Linux browser probes passed; deployment skipped.
- [CodeQL run 37507693780](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37507693780):
  Rust analysis and the CodeQL check passed. PR-ref open security-alert count: 0.
- [Issue linkage run 37507908501](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37507908501):
  issue linkage and proposed policy passed. Protected-base trust enforcement
  passed separately.

T001-T015 are complete. This final receipt changes only tasks/evidence records;
it does not change the externally reviewed implementation. CI on the receipt
commit will be checked before the owner handoff, with the exact final head/status
recorded in the PR body. No third review round may be requested. Owner final
review/merge remains pending. Plan048 archival and release remain excluded.
