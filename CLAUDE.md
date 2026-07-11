# ESO Weave

Desktop companion application for The Elder Scrolls Online, written in Rust for
Windows 10/11 x64 and Linux x64. It runs entirely outside the game: an input
engine intercepts and synthesizes keys while the game window is focused, a weave
engine executes combat action sequences, and an optional fishing module reads a
pixel-bus signal rendered by the embedded PixelBeacon companion addon. The master
specification is `docs/ESO-Weave-Specification-v0.1.0.md`; it is the architecture
of record, and every feature traces to it. Build plans under `docs/plans/`
(index `docs/plans/README.md`) decompose the specification into the ordered work
slices that become spec-kit features and define what to build next; they are
distinct from the per-feature `specs/NNN-name/plan.md` files that `/speckit.plan`
generates.

## Build-phase autopilot

Standing authorization: every feature derived from the master specification runs
under the Build-Phase Autopilot Protocol. A verbal kickoff ("kick off the input
engine feature", "run the next feature", "autopilot this") authorizes the full
spec-kit feature sequence end to end (specify, clarify, checklist, plan, tasks,
analyze, implement, verify, commit) with no pause for authorization between
steps. Every feature MUST be spec'd through the spec-kit framework before
implementation; the master specification scopes a feature but never substitutes
for its spec.

Default to deciding, not asking: enumerate the alternatives, evaluate them
against the constitution (`.specify/memory/constitution.md`), the master
specification, and the feature scope, pick the best, proceed, and record the
rationale. Halt to the user only when no option is clearly best on an
irreversible or architecture-defining choice, the feature intent is genuinely
ambiguous, or a constitution CRITICAL conflict needs a human call.

Halt exactly once per feature: right before `git push` to `main`, with a
breakdown of notable decisions and what was built. Never push, tag, run
`cargo release`, or modify pinned artifacts without explicit authorization.

The full procedure is `docs/build-autopilot.md`. This applies to features
traceable to the master specification and to any feature or task the operator
explicitly places under autopilot; unrelated requests with no such kickoff use
normal interactive mode.

## Integration workflow

Work integrates directly to `main`. Once a push is authorized, changes are
committed and pushed straight to the `main` branch on `origin`; this project
does not use pull requests or long-lived feature branches. The review gate is
the authorization halt itself (the once-per-feature pre-push halt for autopilot
work, or an explicit go-ahead for other work), not a post-push review. The
authorization requirement is unchanged: never push to `main` without it.

## Non-negotiables

- Safety-critical test surfaces are never weakened or skipped: injected-input
  recursion breaking, input suppression scoped to the focused game window only,
  no blocking work on the hook thread, and PixelBeacon uninstall deleting a
  folder only after verifying the managed-marker line in its manifest.
- CI parity before any commit: `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all --locked`, all run in the foreground and watched to
  completion, never backgrounded.
- Pinned artifacts (`.github/workflows/**`, `rust-toolchain.toml`,
  `release.toml`, `scripts/**`, `packaging/**`, `docs/releasing.md`) change only
  with a dated decision recorded in `CHANGELOG.md`.
- Releases follow `docs/releasing.md` exactly; cutting a `vX.Y.Z` tag always
  requires explicit authorization.
- All text files are UTF-8 without BOM with LF line endings. No em-dashes or
  en-dashes anywhere, including code comments; use commas, parentheses, or
  standard hyphens.

<!-- SPECKIT START -->
For additional context about technologies to be used, project structure,
shell commands, and other important information, read the current plan
at specs/005-pixel-bus-reader/plan.md
<!-- SPECKIT END -->
