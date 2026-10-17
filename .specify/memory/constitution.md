<!--
Sync Impact Report
- S135 / issue #319: 5.1.0 -> 5.1.1 on 2026-10-17. Metadata-only clarification
  of retired verification follow-ups; no principle, authority or gate changes.
- Modified principles: none. Added sections: none. Removed sections: none.
- Templates: plan/spec/tasks and installed command skills remain aligned; no
  template changes. CLAUDE.md and build-autopilot.md retain existing authority.
- Follow-up TODOs: none. #131 and #190 are Closed/Not planned, with unperformed
  evidence still unverified. See the S135 audit and their closure comments.
- The earlier Sync Impact Report below is retained as historical metadata.
-->

<!--
Historical Sync Impact Report (through 5.1.0)
- S129 owner-selected #285-#288/#291: 5.0.1 -> 5.1.0 on 2026-10-15. One combined PixelBeacon package, automatic bounded Catalog collection and independent optouts supersede two-package/manual authority; ownership, data integrity and historical evidence remain mandatory.
- S120 wording amendment: 5.0.0 -> 5.0.1 on 2026-10-05. Principle V
  reconciles established S100-S102 native-control/player-state evidence with
  the existing PixelBeacon surface. No new in-game feature or transport.
- Version change: 4.0.0 -> 5.0.0
- Amendment: replace one-shot-only encounter authority with exactly two explicit,
  bounded modes: single encounter and continuous until manually disabled or a
  declared hard failure. Continuous sessions retain independently bounded
  encounters, authoritative order, interruption facts, and aggregate ceilings.
- Modified principles: II and V replace one-shot and one-capture wording with
  explicit two-mode user authority, bounded session state, truthful recovery,
  and no silent eviction while preserving exact selected-source retention,
  local-only data, no uploads, no input, and no automation.
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
  CLAUDE.md ................................... aligned
  docs/src/getting-started/responsible-use.md . aligned
  docs/project/build-autopilot.md ............. aligned
- Historical follow-up entries: issue #131 for live Combat Metrics parity and
  issue #190 for native-log platforms. Both closed Not planned on 2026-09-23;
  neither closure supplies live-parity or platform qualification evidence.
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
- Combined PixelBeacon migration, replacement and removal validate exact managed inventory, links/races and ownership; legacy Data retirement preserves SavedVariables and neighboring addons, with rollback before commit and explicit cleanup state afterward.
- Combined-package catalog and retained legacy encounter state use separate namespaced subtrees.
  Clearing one module preserves the other and never deletes or rewrites the
  shared SavedVariables file from the desktop while ESO may own its in-memory
  contents.
- Any retained legacy encounter module remains dormant until the user explicitly
  enables single-encounter or continuous-until-disabled capture. These are its
  only modes. It retains selected callback values exactly, bounds each encounter
  and the aggregate session spool, declares every loss, interruption, and hard
  failure without silent eviction, uses no Pixel Bus transport, and cannot drive
  automation. Diagnostics and public evidence never disclose raw values.
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

### V. Bounded Scope: Desktop With One Combined PixelBeacon Package

The desktop runs outside the game. It MUST NOT read or write game process memory, intercept network or packet traffic, or orchestrate multiple accounts. Input sequencing remains in the desktop. One managed PixelBeacon package contains independent beacon and automatic Catalog modules. Beacon retains its bounded outbound screen-signal contract and cannot receive commands or synthesize input.

Catalog collection begins automatically at supported game-data readiness unless the persisted Catalog preference disables it. Work pauses during combat or unsuitable transitions and resumes automatically. Scan, finalization, memory, retained generations and retry are bounded. Game-managed save requests do not establish disk durability. No forced reload, synthetic command or inbound live SavedVariables write may apply preferences. Loaded Lua observes startup configuration at its next natural load.

Catalog maintenance and native encounter processing have independent persisted optouts and per-root eligibility; both are dormant without a recognized compatible combined package. Presence/capabilities never grant overwrite ownership. Catalog and legacy encounter SavedVariables retain independent namespaces. Last-completed data survives interrupted collection. Stable completed input is parsed as hostile data without executing Lua, merged with compatible accepted facts and verified before immutable atomic selection. Logs/public evidence do not reproduce raw private payloads.

Legacy addon encounter history remains readable with exact source/identity/loss facts. Consolidation MUST NOT automatically enable or mirror the old addon recorder. Native logging remains the player's game-owned action; automatic processing does not change game logging, serialization or anonymity. Existing authenticated local data reads remain independent of collection eligibility.

All package replacement/removal requires verified exact inventory/ownership, confinement, link/race checks and recovery preserving unrelated data and SavedVariables. One module cannot clear another's state. Additional game ingress or automation authority requires an explicit future decision.

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

**Version**: 5.1.1 | **Ratified**: 2026-07-11 | **Last Amended**: 2026-10-17
