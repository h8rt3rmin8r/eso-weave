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
| [050](plan-050.md) | Active | S124 resolves dependency security and prepares v0.17.4 for issues #261, #262 and #263 |

Plans 039-047 completed their documentation, release, data-addon, input and
extension programs. Plan048 completed its four approved figures through S112,
S121 and S122, with S120's intervening recovery work preserved in its
[archived chronological record](../../archive/build-plans/plan-048.md).
S113-S115 remain unused historical reservations.

Plan049 completed S123; v0.17.3 subsequently published on 2026-10-07.
Plan050 owns dependency maintenance and candidate preparation only. Tag and
package publication require separate authorization after owner merge and exact
merged-head checks.

Field-verification issues #110, #129, #131 and #190 were closed as not planned
under the owner's no-field-verification direction. Their closure is not evidence
of installed behavior or live parity. Native-log ingestion remains provisional.
