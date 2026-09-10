# Contract: ESO Weave Encounter Capture

## Authority

The runtime emitter is `addon/EsoWeaveEncounter/EsoWeaveEncounter.lua`.
`encounter-capture.schema.json` describes the normalized contract that issue
#133 must parse from the restricted SavedVariables table without executing Lua.

## Invariants

1. The SavedVariables root is exactly `EsoWeaveEncounterSaved`.
2. Schema and addon version are positive integers.
3. A terminal capture has explicit Live or PTS channel and source provenance.
4. Every event repeats the opaque session and encounter IDs.
5. Sequence is strictly increasing and is the ordering authority.
6. Exported monotonic time never moves backward.
7. Only the fourteen S069 event kinds are legal.
8. Actor IDs are zero or positive integers local to one encounter.
9. Arbitrary callback names, unit tags, account, character, chat, guild, and
   location strings are forbidden.
10. A discontinuity declares the immediately preceding omitted source range.
11. Complete status has no omitted event and no partial reason.
12. Partial status has a stable partial reason.
13. Stored count equals the event-array length and never exceeds 100,000.
14. Estimated bytes never exceed 33,554,432.
15. The addon disarms and unregisters capture handlers after every terminal path.
16. Addon load alone cannot create a new capture.
17. Pixel Bus, discovery collection, desktop import, metric calculation, upload,
    and action automation are absent from the contract.

## Handoff to Issue #133

The importer must treat the file as hostile text and parse only the declared
table grammar. It must validate this schema, recompute counts and ordering,
canonicalize accepted facts, calculate SHA-256, and publish atomically into a
user-owned raw store. It may not trust the addon's byte estimate or execute Lua.

S075 intentionally makes no claim that a capture is flushed until the player
uses `/reloadui`, logs out, or exits ESO after finalization.
