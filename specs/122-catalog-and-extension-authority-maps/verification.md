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

## Remote integration

Push and official PR publication are explicitly authorized at kickoff. Initial
external review and CI evidence will be recorded here; at most one additional
Codex round is authorized. Respond to every finding, resolve completed threads,
and wait for green checks. Owner final review/merge remains pending. No release.
