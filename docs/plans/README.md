# Build Plans

This directory holds ESO Weave build plans. A build plan decomposes the master
specification (`docs/ESO-Weave-Specification-v0.1.0.md`) into an ordered set of
work slices, each scoped to become one spec-kit feature under `specs/NNN-name/`.

Two documents share the word "plan"; they are distinct:

- A build plan (`docs/plans/plan-NNN.md`) is the higher level slice roadmap. It
  says what to build next and in what order, and it is what repository
  references point at for feature sequencing.
- A spec-kit feature plan (`specs/NNN-name/plan.md`) is generated per feature by
  `/speckit.plan`. It is the implementation plan for a single slice.

Repository references target this directory and its index, not any single plan
file. To add a new plan, drop `plan-002.md` (and so on) into this directory and
add one row to the table below. No other files need to change.

## Index

| Plan | Status | Scope |
| --- | --- | --- |
| [plan-001.md](plan-001.md) | Active | Initial decomposition of the master specification into ten build slices, from foundations through packaging, preceded by the constitution prerequisite. |
| [plan-002.md](plan-002.md) | Active | Brand and UX polish (slice 012): a documented brand standard applied across the app UI, the runtime and executable icon, and the Windows and Linux installers. |
