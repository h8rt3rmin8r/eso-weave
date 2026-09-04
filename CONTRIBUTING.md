# Contributing to ESO Weave

ESO Weave is a safety-sensitive desktop companion. Small fixes are welcome, but
changes that affect input, automation, PixelBeacon, or game-state truth need the
same evidence and review discipline as first-party work.

## Start with an issue

Use the structured bug or feature form. Feature work is organized through the
active milestones and coordinator issues. A feature is assigned a numbered work
slice and specified under `specs/NNN-name/` before implementation.

One actionable issue owns one independently closeable outcome and one lifecycle.
If two outcomes can be merged, released, verified, or rolled back at different
times, file separate issues. Multi-outcome initiatives belong in a coordinator
epic whose native child issues carry the actual acceptance criteria. A single
pull request may close several atomic issues when they share one coherent
implementation and validation path.

During triage, every actionable issue receives one type label, one or more area
labels, exactly one priority, exactly one effort, and a milestone. Priority means
impact and scheduling order: P0 is blocking or unsafe, P1 is a release-relevant
defect or capability, P2 is normal planned work, and P3 is deferred. Effort means
size and uncertainty: XS is a tiny edit, S is a bounded change, M is roughly one
or two days, L is substantial cross-surface work, and XL should normally be split.
Platform labels apply only to platform-specific behavior or evidence.
`needs: verification` is temporary and remains only while reproduction or field
proof is an entry gate.

Implementation completion and release verification are different lifecycles.
When merged code still needs proof from a fresh release or a real game session,
close the implementation issue and leave a separate verification issue open.
Apply `needs: verification` only to the issue that is actively waiting for that
evidence. Never hold a completed implementation issue open merely because a
different release event has not happened.

Coordinator epics use native sub-issues when available and keep the same children
in dependency order in the body. Dependencies belong on the blocked issue and in
the coordinator order. A coordinator closes only when every child and its stated
evidence gate are complete. New labels or areas should extend this grammar rather
than introduce a synonym.

## Branch and pull-request workflow

1. Synchronize with `main` and create a short-lived branch.
2. Complete the relevant spec-kit sequence for feature work.
3. Keep `CHANGELOG.md` and user-facing documentation current.
4. Run the local merge gate in the foreground.
5. Push the branch and open a pull request using the repository template.
6. Give every issue its own complete closing reference, such as `Closes #123`.
   For several issues, repeat the keyword on separate lines. A descriptive
   reference such as `Related to #123` does not close the issue.
7. Address every review thread and wait for required checks before asking a
   maintainer to merge.
8. After merge, confirm every referenced issue closed, synchronize `main`,
   update Project stages, delete the merged branch, prune stale refs,
   and update any parent issue checklist.

The issue-link check exempts Dependabot and pull requests labeled
`dependencies`. The `skip: issue-link` label is reserved for rare release or
repository-administration changes that genuinely have no atomic issue. Explain
that exception in the pull request body. Normal documentation, workflow, and
maintenance work should still begin with an issue.

Direct pushes to `main` are reserved for explicitly authorized repository
administration or release work.

## Local merge gate

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

CI runs the same checks on Windows and Linux. Linux contributors need the system
libraries listed in `scripts/linux-build-deps.sh`.

## Safety invariants

Do not weaken tests for synthesized-input recursion, game-window focus scoping,
non-blocking hook callbacks, signal-loss behavior, AddOns path containment, or
managed-marker-gated addon deletion. Unknown game evidence must remain
non-actionable.

## Pinned and release artifacts

Changes under `.github/workflows/`, plus release, packaging, toolchain, and script
artifacts named in `docs/releasing.md`, require a dated decision in
`CHANGELOG.md`. Releases follow `docs/releasing.md` and always require explicit
maintainer authorization.

All text is UTF-8 without BOM and uses LF line endings. Avoid em-dashes and
en-dashes in repository text.
