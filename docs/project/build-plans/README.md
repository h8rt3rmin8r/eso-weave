# Current Build Plans

This directory contains only plans that direct current repository work. Completed
and superseded plans move to the [build-plan archive](../../archive/build-plans/README.md).

Two documents use the word "plan" and serve different purposes:

- A build plan defines the chronological work-slice boundary and sequence.
- A spec-kit feature plan under `specs/NNN-name/plan.md` describes the
  implementation of one slice.

## Active Plan

| Plan | Status | Current slice |
| --- | --- | --- |
| [038](plan-038.md) | Active | Recommended S070 deterministic SQLite catalog compiler, issue #114 |

S068 merged in PR #130 and closed issue #112. S069 then merged in PR #137 and
closed issue #113. Issue #114 is now Ready. Installed v0.15.1 verification in
issue #110, catalog field verification in issue #129, and live Combat Metrics
verification in issue #131 remain independent; none blocks Plan 038.
