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
| [045](plan-045.md) | Active | S105 local service lifecycle, issue #176 |

Plans 039 and 040 completed the documentation presentation and encounter
recommendation programs. Plan 041 completed when v0.16.0 shipped with all five
required assets. Plan 042 completed the persistent data-addon subsystem through
S096 and closed epic #182. Plan 043 completed operator intent reliability and
the native-binding program through S102. Plan 044 completed documentation diagram
legibility in S103. Those plans are archived. Plan 045 establishes the local
extension surface contract and then implements its lifecycle, player state, MCP,
database query, and documentation sequence.

Installed v0.15.1 verification in issue #110, catalog-field verification in
issue #129, live Combat Metrics verification in issue #131, and native-log
verification in issue #190 remain independent Release verification work. They
do not block S105.
