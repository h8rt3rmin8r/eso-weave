# Native Completed-File Import Contract

Scope: S127/#272, normal/separate log version15. Native files are never executed.

## Grammar and profile

Each consumed record ends in LF (optional preceding CR). UTF-8 is required.
A trailing unterminated record is counted and unconsumed. Timestamp/type precede
positional fields. Commas delimit only outside quoted strings and balanced bracket
groups. Quoted separators/Unicode and explicit doubled-quote/backslash escapes are
tested. Unsupported inline/verbose known-record layouts produce controlled rejection.

BEGIN_LOG has7 fields and a checked epoch/version plus three quoted metadata strings.
BEGIN/END_COMBAT and END_LOG have2 fields. Unit-state expansion has10 fields;
target '*' reuses the source state. Recognized shapes and domains are strictly
validated; extra opaque END_CAST integers remain exact and uninterpreted.
Supported shapes are listed in research.md; metadata lists stay balanced/bounded.
Unknown type lines remain bounded records and explicit counts, never silently
promoted into recognized metrics.

## Boundaries and evidence

A BEGIN_LOG starts isolated context. Nested/orphan combat boundaries and reversed
combat endpoints reject. Open combat at run/file end is incomplete, not complete.
An unfinished run can contain independently completed fights. Complete fight order
uses native source ordinals. Outside-combat records are counted separately; relevant
unit/ability/effect/player context accompanies each imported fight in original order.
Original source remains unchanged.

## Reader and identities

Stable no-follow source snapshots enforce native bounds in plan.md and detect
replacement/append/truncation during read. Explicit environment is user-selected
provenance if the header provides no authoritative environment.
Copying identical files changes neither native IDs nor content. Appending later
runs changes neither preceding run nor completed combat identities.
No API/addon version is derived from log version or game patch.

## Storage and UI

Whole batch and migration commit together after identity/content preflights.
Collision/invalid/unsupported input preserves previous store bytes/schema.
Native and addon history coexist; deletion/backup/reimport respect both.
History source selection is explicit, with native path, Inspect and Import actions.
Source key and generation are echoed; stale results cannot replace current selection.
Default UI/errors/receipts show controlled source facts/counts and capability reasons,
without names, raw IDs, field arrays or source lines.

## Metric capabilities

Paired elapsed duration, observed counts and reviewed native quantities are source
facts. Actor-specific quantities require observed local-unit relation. Hit and
overflow stay separate. Uptime, callback-only snapshots/actions, addon replay,
catalog API joins and recommendations need evidence this profile does not supply
and remain explicitly unavailable. Unknown observations reduce completeness claims.
No installed/platform or callback parity is claimed.

Cumulative context work across all runs is bounded to20 million record visits
and512MiB original context text. Reservations precede snapshot cloning/scanning
and include irrelevant metadata. A controlled work-limit rejection occurs before
store creation or migration. Replacement/removal updates cached context size.
