# Contract: Continuous Session Envelope

## Version dispatch

- A module object with `state_schema_version = 1` is the S096 controller/spool.
- An object without that field and capture `schema_version` 1 or 2 is a legacy singleton.
- Any other version combination is preserved by the addon and rejected by import.

## Structural invariants

1. The controller uses closed mode, state, reason, and channel enums.
2. Requested, active, state, session disposition, current presence, and failure presence agree.
3. Record keys are exactly `0000000001` through the completed count.
4. Every record is terminal, independently valid, and has matching session/channel identity.
5. Current exists only while capturing and is never a terminal import member.
6. Marker sequences are contiguous and refer only to completed ordinals.
7. Stored counts and checked byte estimates equal recomputed nested evidence.
8. All identifiers are bounded before any log or UI projection.

## Aggregate ceilings

| Resource | Maximum |
| --- | ---: |
| normalized events | 100,000 |
| raw observations | 100,000 |
| estimated encounter bytes | 33,554,432 |
| terminal encounters | 1,024 |
| interruption markers | 1,024 |

The addon reserves terminal facts inside the current record and one outer
failure record before accepting regular data. A next encounter begins only if
its minimum start/end authority also fits. No ceiling rolls old data.

## Snapshot monotonicity

Each controller mutation increments `revision`. For a known session, a later
snapshot must retain all prior encounter/hash references and interruption facts
as exact prefixes. Older identical snapshots are harmless. Revision reuse with
different bytes, regression with unseen content, member deletion, mode/channel
drift, or changed prior hash rejects the batch.

## Recovery

Only last durably flushed explicit authority can resume. A recovered active
member becomes partial before continuous returns to waiting. Single stops.
Failed sessions do not resume. A crash before ESO flushes state is unknowable and
cannot produce an inferred marker.
