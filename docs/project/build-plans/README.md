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
| [042](plan-042.md) | Active | S095 deterministic encounter replay, issue #200 |

Plans 039 and 040 completed the documentation presentation and encounter
recommendation programs. Plan 041 completed when v0.16.0 shipped with all five
required assets. Those plans are archived. Plan 042 now sequences the persistent
data-addon subsystem: S092 established the permanent foundation, S093 added the
first-class lifecycle and status interface, S094 established the lossless
raw-event authority, and S095 verifies its normalized projections independently.

Installed v0.15.1 verification in issue #110, catalog-field verification in
issue #129, live Combat Metrics verification in issue #131, and native-log
verification in issue #190 remain independent Release verification work. Issue
#190 gates native-log platform claims but does not block S095.
