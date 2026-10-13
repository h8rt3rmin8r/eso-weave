# S126 Acceptance Audit

Date: 2026-10-13. Issue #273. Base: 7939729f302611f911c39216aea0c00f5dfe3ef1.

| Outcome | Implementation | Repository evidence |
| --- | --- | --- |
| Record again without deleting | Admission before terminal archive rotation; monotonic persisted IDs | Two same-clock single sessions survive reload and atomic import |
| Stop/resume versus start | Explicit continuous resume adds a stop-bound marker; start retains prior session | Stopped-to-stopped and stopped-to-active imports; repeated resume; single/archive refusal |
| Chronology and provenance | Ordered retained keys; validation before selected-environment filtering; durable store session order | Mixed environments, clock reversal, deletion/reimport and per-session snapshots |
| Capacity without overwrite | Cached immutable archive totals, whole-spool budgets and terminal/failure reserves | Session and event ceilings preserve old data and refuse mutation |
| Deliberate deletion | Confirmed inactive-session discard; clear preserves identity counter | Exact target removal, active/unknown/no-confirm refusal and catalog isolation |
| Migration and recovery | Supported controller/terminal migration; schema1-4 store migration; invalid evidence preserved | Legacy bytes/IDs retained, schema4 read-only access, canonical bytes unchanged, primitive/nested/unknown rejection |
| Source/history integrity | Strict parsing and all snapshot/record preflights in one transaction | A conflicting later session rolls back a newly importable earlier session; rejected schema1-4 imports preserve exact database bytes |
| Workflow presentation | Matching session summaries plus whole-file capacity; copyable commands | Worker generations/environment filtering; rendered source/refresh/import at 420 and 760 widths |
| Metadata disclosure boundary | Numeric generated identity grammar matches desktop validation | Malformed identifier canary remains preserved/inactive and absent from session messages |
| Recording authority | Existing warning delivery required before start and resume | Delayed/full chat checks and explicit resume warning regression |

New archives declare base controller v2/addon5 because resume adds interruption
vocabulary. Legacy controller v1/addon4 snapshots remain accepted without that new
marker. Terminal payload schema2/addon3 and catalog/root versions are unchanged.

Superseded assertions were updated only for the owner-approved behavior: emitted
controller version, retained-session start eligibility, and empty-session
restart guidance. Shared-addon catalog isolation and inactive/unknown recovery
assertions remain. Historical-store fixture construction removes the new ordering
table when deliberately reconstructing schema1.

Full local gates: 1088 Rust tests, 191 policy fixtures, fmt, Clippy, release build,
documentation test/build/link checks, spelling, trust/brand policy, browser
rendering, release-note and Debian validator fixtures. Evidence is listed in
[verification.md](verification.md). No installed/game behavior is claimed.

Second-round P2 correction: existing-store migration now shares the write
transaction with import preflight/append. Record and snapshot collisions retain
schema4 and exact file bytes; schema1-3 collision fixtures retain their layouts
before successful migration. Missing deletion also leaves schema4 unchanged.
