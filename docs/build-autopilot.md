# Build-Phase Autopilot Protocol

Version: 1.0.0
Adopted: 2026-07-10
Status: operating procedure for the coding agent
Project: eso-weave (`github.com/h8rt3rmin8r/eso-weave`)

This document is the operating procedure for running a spec-kit feature under
autopilot. It is governed by the project constitution
(`.specify/memory/constitution.md`) and reaffirms, never weakens, its principles.
The constitution is project law; this document is the how. Where they appear to
conflict, the constitution wins.

## Purpose

Every feature derived from the master specification
(`docs/ESO-Weave-Specification-v0.1.0.md`) runs the same spec-kit sequence. The
default agent behavior pauses for authorization between each step and raises
routine decisions to the user that, in practice, are approved as recommended.
Autopilot removes that friction: one verbal kickoff runs a full feature end to
end, the agent makes the routine decisions itself and records them, and the agent
halts once, right before the push to `main`, with a breakdown for review.

## Trigger

The user starts an autopilot feature run with a verbal kickoff naming the feature
or the next feature, for example:

- "Kick off the input engine feature"
- "Run the next feature"
- "Autopilot 003"

The operator may also place any other feature or task under autopilot with an
explicit request, for example:

- "Run the beacon installer under autopilot"
- "Autopilot this"

On trigger, the agent runs the entire feature sequence below without pausing for
inter-step authorization.

## Per-feature sequence

The agent runs these steps in order, with no halt between them:

1. `/speckit.specify` creates `specs/NNN-*/`, `spec.md`, and
   `checklists/requirements.md`, drawing scope from the relevant sections of the
   master specification.
2. `/speckit.clarify` runs under the decision policy below. The agent answers
   clarification questions itself from the feature spec, the constitution, the
   master specification, and the feature's stated scope and acceptance criteria.
   Only genuinely unanswerable questions are escalated.
3. `/speckit.checklist` adds domain checklists where the feature warrants them.
4. `/speckit.plan` produces `research.md`, `data-model.md`, `contracts/`, and
   `quickstart.md`.
5. `/speckit.tasks` produces `tasks.md`.
6. `/speckit.analyze` is the blocking gate. The agent resolves findings. A
   genuine CRITICAL conflict that needs a human decision triggers an early halt.
7. `/speckit.implement` executes the tasks under test-driven discipline. Tests
   covering the safety-critical behaviors below are required, not optional.
8. Verify with CI parity: `cargo fmt --all -- --check`,
   `cargo clippy --all-targets --all-features -- -D warnings`, and
   `cargo test --all --locked`. Run these in the foreground and watch them to
   completion; never launch the test suite in the background and poll for its
   output. `cargo` buffers test output until the run ends, so a background run
   cannot be distinguished from a dead one, and doing so has caused misdiagnosed
   hangs. A red result that cannot be fixed within the feature triggers a halt
   with the failure.
9. Commit locally as `feat(NNN): <title>` (NNN is the spec-kit feature number)
   with the agent's `Co-Authored-By:` attribution trailer, and update the
   `CHANGELOG.md` `[Unreleased]` section (an Added line, plus a dated Decisions
   entry for any architecture-affecting choice).
10. Halt before `git push`. Present the breakdown below and wait for explicit
    authorization.

## Decision policy

This is the core behavioral change. For any decision point that the default
behavior would raise to the user, the agent instead:

- Enumerates the viable alternatives.
- Evaluates them against the constitution, the master specification, the
  feature's stated scope and acceptance criteria, and existing code patterns.
- Picks the best-supported option, proceeds, and records the decision and its
  rationale in the feature's `plan.md` or `spec.md`, and in `CHANGELOG.md`
  Decisions when the choice is architecture-affecting.

The agent halts to the user only when one of these holds:

- No option is clearly best and the choice is materially irreversible or
  architecture-defining.
- The feature's intent or scope is genuinely ambiguous in the master
  specification.
- A constitution CRITICAL conflict cannot be resolved without a human decision.

## The pre-push halt breakdown

At the single halt, the agent presents:

- The feature number and title, and what was built: the spec, plan, and tasks
  artifacts, the code modules, and the tests.
- The notable decisions made and why (the decision log).
- The verification results for fmt, clippy, and tests, with evidence of pass or
  fail.
- Any deviations or open risks against the feature's acceptance criteria.
- The exact `git push` command awaiting authorization.

## Always-halt guardrails

These hold regardless of the decision policy:

- Never `git push`, tag a release, or run `cargo release` without explicit
  authorization.
- Never weaken or skip the `/speckit.analyze` gate.
- Never weaken or skip the safety-critical test surfaces of this project:
  - Input-engine safety: injected-input recursion breaking, suppression scoped
    to the focused game window only, and no blocking work on the hook thread.
  - Beacon-manager safety: uninstall deletes a `PixelBeacon` folder only when
    the managed-marker line is verified present in its manifest, and discovery
    never writes outside the resolved AddOns directory.

Pinned process artifacts (`.github/workflows/**`, `rust-toolchain.toml`,
`packaging/**`, `scripts/**`, `.gitattributes`, `.gitignore`, `LICENSE`) may be
modified when a feature's scope requires it, provided the change is recorded as
a dated decision in the changelog. Autopilot does not halt separately for this.
The changes surface at the once-per-feature pre-push halt and must pass the CI
merge gate before merge. Cutting a release (a `vX.Y.Z` tag) still requires
explicit authorization.

## Scope and expiry

Autopilot is valid for features derived from
`docs/ESO-Weave-Specification-v0.1.0.md`. It also applies to any other feature or
task when the operator explicitly requests an autopilot run (for example "run the
MSI packaging under autopilot" or "autopilot this"). Such an explicit request
authorizes autopilot for the named work and is itself the renewal.

Absent an explicit request, work not traceable to the master specification falls
back to normal interactive mode. When the master specification is superseded by a
new version, the standing authorization lapses and requires renewal against the
new document; per-request autopilot remains available regardless.
