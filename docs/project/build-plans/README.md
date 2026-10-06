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
| [048](plan-048.md) | Active | S121 messaging audit and encounter lineage, then remaining #223/#221 figures |

Plans 039 and 040 completed the documentation presentation and encounter
recommendation programs. Plan 041 completed when v0.16.0 shipped with all five
required assets. Plan 042 completed the persistent data-addon subsystem through
S096 and closed epic #182. Plan 043 completed operator intent reliability and
the native-binding program through S102. Plan 044 completed documentation diagram
legibility in S103. Plan 045 completed the local extension surface through S109,
and Plan 046 completed the Ultimate Auto Potion watch in S110. Plan 047 then
completed the full documentation visualization audit in S111. Those plans are
archived. Plan 048 implements the four approved figures one atomic slice at a
time, beginning with the S112 troubleshooting decision tree.
The owner-approved S120 recovery bundle interrupts the remaining figure sequence;
S113 through S115 retain their reservations for later documentation work.

S120 merged as #252. The owner then authorized S121 for #249 and #222, combining
the full messaging repair with the formerly reserved S115 lineage figure.
S113/#223 and S114/#221 remain separate; unused reservations are not completed
implementation records.

Field-verification issues #110, #129, #131 and #190 were closed as not planned
under the owner's no-field-verification direction. Their closure is not evidence
of installed behavior or live parity. Native-log ingestion remains provisional
and does not block the repository-backed figure work.
