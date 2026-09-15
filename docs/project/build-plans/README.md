# Current Build Plans

This directory contains only plans that direct current repository work. Completed
and superseded plans move to the [build-plan archive](../../archive/build-plans/README.md).

Two documents use the word "plan" and serve different purposes:

- A build plan defines the chronological work-slice boundary and sequence.
- A spec-kit feature plan under `specs/NNN-name/plan.md` describes the
  implementation of one slice.

## Active Plans

| Plan | Status | Current slice |
| --- | --- | --- |
| [043](plan-043.md) | Active | S100 native ESO binding evidence, issue #206 |

Plans 039 and 040 completed the documentation presentation and encounter
recommendation programs. Plan 041 completed when v0.16.0 shipped with all five
required assets. Plan 042 completed the persistent data-addon subsystem through
S096 and closed epic #182. Those plans are archived. Plan 043 contains S097
Auto Potion intent persistence, S098 stale HUD presentation reliability, and the
S099 trust-boundary gate and the S100 through S102 native-binding program.

Installed v0.15.1 verification in issue #110, catalog-field verification in
issue #129, live Combat Metrics verification in issue #131, and native-log
verification in issue #190 remain independent Release verification work. Issue
#190 gates native-log platform claims but does not block S100.
