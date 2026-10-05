# S120 Evidence and Delivery Record

## Specification Gates (2026-10-05)

Installed spec-kit sequence completed: specify, clarify, checklist, plan, tasks,
and read-only analyze before implementation. Research agents performed source
research only. Both requirements checklists passed without incomplete items.
The obsolete constitution V description was amended separately before analysis
to match established S100-S102 behavior; no new transport or input authority.

Analysis: 14 functional requirements, five measurable criteria, 14 tasks, 100%
coverage, zero remaining ambiguities, duplications, or critical findings. Actual
module paths were corrected before the final read-only analysis pass.

| Requirement | Tasks |
| --- | --- |
| FR-001 | T002, T004, T005 |
| FR-002 | T005, T006 |
| FR-003 | T006 |
| FR-004 | T004, T005, T006 |
| FR-005 | T002, T007, T008 |
| FR-006 | T007, T008, T010 |
| FR-007 | T007, T010 |
| FR-008 | T007, T010 |
| FR-009 | T007, T008, T010 |
| FR-010 | T009 |
| FR-011 | T002, T008, T009 |
| FR-012 | T011, T012 |
| FR-013 | T011, T012 |
| FR-014 | T002, T013 |
| SC-001 | T006 |
| SC-002 | T007, T010 |
| SC-003 | T011 |
| SC-004 | T011 |
| SC-005 | T001, T003, T013, T014 |

## Test-First Evidence

- The new four-slot keyboard/controller fixture failed on the original discovery
  implementation. It passes after controller classification precedes conflict
  counting. The same Lua-generated skill and mouse-attack cells pass through the
  protocol decoder, routing, admission, and RealSink to the full mock sequence.
- API parser fixtures failed before implementation and now cover merge history,
  independently documented numeric API, invalid revision/header/time, stale
  source, and future source. Application fixtures cover unsupported observations,
  restart, supported API, wrong environment, stale/future evidence, and offline.
- HUD transition logging failed before implementation. Loss/recovery and expiry
  now log once with zero-retention coverage. Headless rendered geometry proves
  row absence and unchanged gauge position across loss and recovery.
- Existing worker timing/latency/bar, focus, recursion, cancellation cleanup,
  package ownership, rollback, neighbor, and shared data tests remain required.

## Decisions and Limits

Both packages use reviewed embedded API declarations. Startup observations no
longer rewrite installed manifests. Install/update remains the ownership-scoped
way to deliver new addon code and declarations. A persistent API summary uses the
existing PixelBeacon status row; numeric Game API details use Data Details. It is never a
temporary notice above the gauges. Package integrity remains a separate line.

Source evidence is the pinned published UI revision documented in research.md.
The owner's installed bindings/build and real-game behavior remain unobserved.
No release, merge, or live application/game execution is part of S120.

## Delivery Gates

Local `cargo fmt --all -- --check`, full all-target/all-feature Clippy with
warnings denied, and `cargo test --all --locked` passed on Windows. The complete
suite passed 1,031 counted tests with zero failures or ignored tests and includes
the deterministic documentation scene validator. All existing
input/ownership/retention checks remain enabled.

`mdbook build docs` and link checks passed. The existing generated-site policy
passed; 175 documentation/trust/brand policy tests passed. Repository trust scan
passed for seven workflows and 54 local skills. UTF-8/no BOM/LF/mojibake checks
passed for all 39 changed/new text files, and `git diff --check` passed.
Headless generated-site diagram, layout, syntax, figure, and table smoke
sentinels all passed.

PR #252 publishes S120. The first Codex review completed on `f7303a7` and found
one P2 issue: a PTS observation could overwrite historical Live evidence. The
new Live/PTS offline-restart and legacy-state regression failed before the
correction. Independent channel history now retains both unresolved warnings,
and a supported observation clears only its own channel's warning.

The correction was committed as `d5d285a`, with formatting, Clippy and all 1,031
tests passing. The first review finding received a reply and its thread was
resolved. The second and final authorized Codex round completed on `d5d285a` at
2026-10-05 19:54 UTC with no further major issues. No third round was requested.

Initial hosted Linux, dependency, documentation and closing-issue attempts failed
before executing repository steps. Their annotations reported that a hosted
runner was not acquired. The owner resumed work after the GitHub Actions incident
was resolved. Targeted retries preserved the successful Windows result.

Hosted Windows CI, documentation, dependency review, trust/issue policy and
CodeQL checks passed on `d5d285a`. The PR security-alert query returned no open
alerts. Linux CI then passed its full test and release-build steps; the complete
CI run was green at 2026-10-05 23:35 UTC. No local parity result is presented as
an installed result.

Implementation CI receipts:

- Linux/Windows and dependency review: [run 37366057806](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37366057806).
- Documentation: [run 37366057655](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37366057655).
- CodeQL: [run 37366057661](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37366057661).
- Trust boundary: [run 37366055538](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37366055538).
- Issue linkage: [run 37366224639](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37366224639).

This final receipt changes only the slice's evidence and task records. The
implementation remains the reviewed `d5d285a` content. The final owner handoff
must also wait for green hosted checks on the receipt commit; no third Codex
round is authorized. [PR #252](https://github.com/h8rt3rmin8r/eso-weave/pull/252)
remains open for the owner's final review and merge.

The two-round review budget is exhausted. Final merge remains the owner's
responsibility; no release or live-game verification was performed.
