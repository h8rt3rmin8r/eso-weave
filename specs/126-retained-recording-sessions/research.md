# S126 Research

Date: 2026-10-13. Read-only speckit-plan research used the protected-base
controller, parser, history snapshot and database migration seams.

## Archive representation

Decision: Keep current controller fields and add retained_sessions, current
session_sequence and next_session_sequence to a v2/controller5 envelope.
Archived snapshots retain the current base keys and revisions and are terminal.
New archives declare v2/controller5 so resume markers do not masquerade as v1.
Legacy v1/controller4 snapshots remain accepted without resume markers.
Rationale: Existing validation and interrupted recovery remain reusable; terminal
payloads do not change. Alternatives: nested current wrapper (broad callback/test
rewrite), flat record union (loses session lifecycle and ownership).

## Identities and bounds

Decision: Archive keys are padded exact creation sequence tokens, gaps after
discard are valid, and the next sequence survives clear. Session IDs include
the sequence; original migrated identities stay untouched.
Keep existing event/raw/byte/encounter budgets across the whole spool, plus
32 sessions and 1024 interruptions. Cache immutable archived totals.
Alternative independent session budgets would multiply the memory/file ceiling.

## Import and history

Decision: Validate current and every archive using its original environment,
then create ordered matching session controllers and terminal records.
Each store snapshot references only its own records. Preflight all snapshot and
record collisions before inserting in one transaction.
Store schema5 adds durable session order allocated at first import. Preserve
schema1-4 canonical records and snapshots in migration; read-only old stores
remain readable. Rowid or lexical-ID ordering fails deletion/clock-reversal cases.

## Resume

Decision: Only current stopped continuous sessions resume. A new UserResumed
interruption marker stores prior_finished_at and the prior terminal ordinal.
Snapshot extension requires a newly appended, matching marker; archived and
failed sessions stay terminal. Mode, environment, identity and old fight data do
not change. Stop/resume/stop between imports remains valid.

## Integration findings

The prior store built one snapshot from parsed.state; flattening multi-session
records would wrongly assign all to the latest session. Prepare snapshots for
the ordered matching controller set instead. Stopped selection changes must not
make the original session provenance invalid. Complete warning and capacity
admission before archive rotation or sequence allocation.

No external format or API change is assumed; native Encounter.log remains #272.

## Pre-publication design refinement

New archived base controllers declare v2/controller5 because their explicit
resume markers extend the earlier interruption vocabulary. Legacy v1/controller4
archives remain accepted, but cannot declare UserResumed. Admission estimates
the actual combat-opening kind using a bounded maximum identity before rotation.
