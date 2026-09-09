<!--
Sync Impact Report
- Version change: 2.0.0 -> 2.0.1
- Amendment: clarify the chronological boundary between mandatory
  pre-publication gates and artifact-dependent post-publication verification
- Modified principles: none; Development Workflow and Quality Gates now makes
  the existing release-verification lifecycle explicit
- Added principles: none
- Added sections: none
- Removed sections: none
- Templates:
  .specify/templates/plan-template.md ......... aligned (generic Constitution
    Check gate references this file; no edit needed)
  .specify/templates/spec-template.md ......... aligned (no constitution
    references; no edit needed)
  .specify/templates/tasks-template.md ........ aligned (no constitution
    references; no edit needed)
  CLAUDE.md, docs/project/build-autopilot.md .. aligned with chronological
    release verification
  docs/project/governance.md .................. aligned with separate
    implementation and artifact-evidence lifecycles
  docs/project/releasing.md ................... aligned with post-publication
    verification and the Debian metadata gate
  .github/workflows/ci.yml, release.yml ....... aligned with the reusable
    Debian package validator
- Follow-up TODOs: issue #107 verifies the first released Debian package that
  contains S066
-->

# ESO Weave Constitution

## Core Principles

### I. Spec-Driven Development (NON-NEGOTIABLE)

Every feature traces to an actionable issue and the canonical shipped behavior
and architecture documentation under `docs/src`, then is built through the full
spec-kit sequence before any implementation code: specify, clarify, checklist,
plan, tasks, analyze, then implement. Work lands as a numbered
`specs/NNN-name/` slice holding its `spec.md`, `plan.md`, and `tasks.md`.
Current build plans under `docs/project/build-plans/` set slice order and
boundaries. When new or changed behavior makes canonical documentation
incomplete, the implementing slice updates it or links a separately scoped
documentation issue. The `/speckit.analyze` gate MUST pass and MUST NOT be
weakened or skipped.

Rationale: issue scope, numbered implementation evidence, and the published
corpus form one reviewable authority chain. A single monolithic specification
became stale and mixed user, architecture, and project lifecycles.

### II. Safety-Critical Surfaces Are Sacrosanct (NON-NEGOTIABLE)

These behaviors MUST always be covered by tests that are never weakened,
skipped, or made conditional:

- Injected-input recursion breaking: synthesized input is flagged so the engine
  never intercepts its own output.
- Input suppression scoped to the focused game window only; the app never hooks
  input globally.
- No blocking work on the input hook thread; interception callbacks classify
  and hand off, and all timed sequences run on a dedicated worker thread.
- PixelBeacon uninstall deletes a folder only after verifying the managed-marker
  line in its manifest; an unmanaged folder is never deleted.
- AddOns discovery never writes outside the resolved AddOns directory.
- Fishing degrades to disabled on SignalLost rather than firing inputs blind.

Rationale: each surface, if wrong, silently breaks input handling or destroys
data outside the app. These are the exact failure modes the design exists to
prevent.

### III. Test-First With Explicit Seams

Implementation follows test-first discipline: a failing test is written before
the code that satisfies it, and the red-green-refactor cycle is honored.
Platform and hardware dependencies are crossed through trait seams (the `Input`
trait, the `BiteDetector` trait) so the weave engine, fishing controller, and
decoders are unit-testable with mock backends, without the game or physical
devices.

Rationale: the engine cannot be verified by hand against a live game. The seams
are what make it verifiable at all.

### IV. CI Parity Before Every Commit (NON-NEGOTIABLE)

Any commit that adds or changes buildable Rust sources MUST first pass the full
merge gate: `cargo fmt --all -- --check`,
`cargo clippy --all-targets --all-features -- -D warnings`, and
`cargo test --all --locked`. These run in the foreground and are watched to
completion; they are never backgrounded, because cargo buffers test output
until the run ends and a backgrounded run cannot be told apart from a hung one.
A red result that cannot be fixed within the current work halts progress.
Documentation-only or configuration-only commits that touch no Rust source have
no cargo gate to run, but still obey the text hygiene rules below.

Rationale: local parity with CI keeps `main` continuously releasable and
prevents the misdiagnosed hangs that backgrounded test runs have caused.

### V. Bounded Scope: Outside The Game

ESO Weave runs entirely outside the game and its addon ecosystem. It MUST NOT
read or write game process memory, intercept network or packet traffic, provide
in-game functionality beyond the PixelBeacon screen-signal contract, or
orchestrate multiple accounts. The weave engine has no in-game dependency of any
kind; only the fishing module depends on PixelBeacon.

Rationale: the scope boundary is the project's technical and ethical contract.
Crossing it changes what the software is.

## Platform, Configuration, and Text Hygiene Constraints

- Targets are Windows 10 and 11 x64 and Linux x64; macOS is out of scope.
  Cross-platform behavior is achieved with one trait and per-OS backend modules
  in a single Rust crate; promotion to a Cargo workspace requires a documented
  justification.
- Configuration stores user settings only. No session, runtime, or derived
  state is ever written to the config file. Config is JSON, UTF-8 without BOM,
  LF, pretty-printed, carries a top-level `schema_version`, migrates older
  schemas forward on load, and on corruption falls back to defaults while
  preserving the bad file with a `.invalid` suffix and surfacing a notice.
- Logging is structured with a runtime-selectable level and an always-available
  in-memory ring buffer. Input contents are never logged above DEBUG and never
  while suspended.
- All text files are UTF-8 without BOM with LF line endings. Em-dashes and
  en-dashes MUST NOT appear anywhere, including code comments; use commas,
  parentheses, or standard hyphens.

## Development Workflow and Quality Gates

- Features run under the Build-Phase Autopilot Protocol
  (`docs/project/build-autopilot.md`): one kickoff runs the spec-kit sequence end to
  end, the agent decides routine questions itself and records the rationale, and
  halts once before the first remote push unless publication was explicitly
  authorized at kickoff.
- Feature integration uses a short-lived branch and an official pull request to
  `main`. Hosted CI and review threads are merge gates, the operator performs the
  final review and merge, and the agent completes post-merge branch and tracker
  housekeeping.
- Direct pushes to `main` are limited to explicitly authorized repository
  administration or release work.
- Pinned artifacts (`.github/workflows/**`, `rust-toolchain.toml`,
  `release.toml`, `scripts/**`, `packaging/**`, `docs/project/releasing.md`, plus
  `.gitattributes`, `.gitignore`, `LICENSE`) change only with a dated decision
  recorded in `CHANGELOG.md`.
- Releases follow `docs/project/releasing.md` exactly. Cutting a `vX.Y.Z` tag and
  running `cargo release` always require explicit human authorization, as does
  every remote push unless that authorization was supplied at kickoff for the
  named work.
- Required CI, safety, release-note, packaging, repository, and authorization
  gates complete before publication. A criterion that can only be evaluated
  against a downloadable release, installed package, field environment, or
  production surface follows publication in a separate verification issue.
  That issue may be opened earlier but remains in Release verification until its
  evidence passes. Publication creates the evidence subject and is never proof
  of installed behavior. A mismatch creates linked implementation work and the
  verification issue stays open until a fixed release passes.

## Governance

This constitution supersedes other process conventions where they conflict.
`CLAUDE.md` and `docs/project/build-autopilot.md` provide runtime development guidance
and MUST remain consistent with these principles; where they appear to conflict,
the constitution wins.

Amendments are made by editing this file, recording the change in the Sync
Impact Report at its top, and bumping the version by semantic versioning: MAJOR
for backward-incompatible governance or principle removals or redefinitions,
MINOR for a new principle or materially expanded guidance, PATCH for
clarifications and wording. Every feature's `plan.md` includes a Constitution
Check that MUST pass before implementation, and the `/speckit.analyze` gate
verifies ongoing compliance. Complexity that violates a principle MUST be
justified in writing against the principle it strains, or be removed.

**Version**: 2.0.1 | **Ratified**: 2026-07-11 | **Last Amended**: 2026-09-08
