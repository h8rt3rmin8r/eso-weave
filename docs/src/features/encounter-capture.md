# Encounter Capture

The `EsoWeaveData` encounter module is an explicit local observation tool.
It records one selectively subscribed encounter into ESO SavedVariables so later
work can import and analyze the same ordered facts outside the game. Every scalar
value delivered by a selected source is retained exactly unless a declared hard
limit or unsupported runtime type prevents the whole observation from fitting.

This capture is not automatic. Loading or installing the addon does not authorize
recording. It remains dormant until you arm one Live or PTS encounter.

## Install and arm

The desktop lifecycle controls install the exact package under
`addon/EsoWeaveData`. Catalog and encounter state share one versioned outer
SavedVariables root, but each command mutates only its own module subtree.

Outside combat, enter one of:

```text
/ewencounter arm live
/ewencounter arm pts
```

The channel is always explicit. Arming during combat waits until combat ends,
then starts at the next clean combat boundary. One arm authorizes at most one
encounter, and normal combat end disarms automatically. The arm message warns
that exact local data can include account, character, unit, ability, effect, and
other identifiers supplied by ESO.

Use these additional commands:

| Command | Result |
| --- | --- |
| `/ewencounter status` | Show state plus stored and omitted event counts |
| `/ewencounter disarm` | Cancel an arm before capture starts |
| `/ewencounter stop` | End an active capture as partial |
| `/ewencounter clear confirm` | Clear the retained encounter envelope |
| `/ewencounter help` | Show the command summary |

A complete or partial retained capture blocks another arm until you clear it.
This makes replacement a deliberate local decision.

## Captured facts

Schema v2 keeps two related streams. `raw_observations` is authoritative: each
selected callback or normalization-dependent API read carries its raw source
sequence, monotonic time, API and source versions, input and return counts, and
contiguous tagged values. Tagged nil, boolean, string, and finite-number values
preserve positions and exact values. Unknown combat results and extra future
scalar arguments remain raw even when no normalized fact exists.
One raw observation is limited to 256 tagged values. A larger callback is
omitted whole and declared as record-limit loss.

The `events` stream remains a linked compatibility projection for current
metrics and recommendations. It records encounter boundaries, damage, healing,
effects, resources, casts, weapon-bar changes, death, resurrection, boss health,
performance, quickslots, and discontinuities. Each v2 projection identifies its
primary raw source sequence and projection ordinal. Encounter-local actor numbers
in this derived stream do not replace or redact source-exact raw names, tags, and
identifiers.

If the initial combat-state callback itself exceeds a hard raw bound, the
partial encounter-start boundary references declared-lost raw sequence 1. No raw
value is fabricated, and the retained loss range explains the missing source.

The selected-source matrix is maintained in the S094 contract. Selective
subscription controls addon cost; lossless retention controls what happens after
a selected callback is delivered. S094 adds no new event family.

## Bounds and incomplete capture

One capture is limited to 100,000 raw observations, 100,000 compatibility events,
256 tagged values per observation, 4,096 derived actor mappings, 64 KiB per raw
string, and a conservative 32 MiB estimated SavedVariables budget. Exact
serialized size depends on ESO and is not claimed by the in-game estimate.

Capacity is reserved for raw and normalized terminal evidence. A record, byte,
or string limit and an unsupported value omit the entire raw observation rather
than truncating it. Raw sequencing then continues in constant memory and one
`raw_loss` range declares exact missing sequences and the first loss reason.
Backward clock movement creates a retained temporal-discontinuity observation.
Reload, deactivation, explicit stop, or callback failure also produces a partial
result instead of a false complete result.

## Save and ownership boundary

After capture, use `/reloadui`, log out, or exit ESO so the client writes
`SavedVariables/EsoWeaveData.lua`. The file stays on your system. Open File,
Encounter History in the desktop and choose Import Current Capture to import it
through the bounded non-executing path. The action uses the Live or PTS
environment selected in Settings and stores accepted raw history only in the
per-user application data directory.

The importer treats the SavedVariables file as hostile text, validates every
bound and sequence, computes the canonical hash, and preserves prior history on
failure. Store schema v2 accepts legacy capture v1 and raw-authority capture v2.
Migration copies legacy canonical bytes and hashes unchanged. Encounter History
labels derived results as observed and exposes partial capture loss and unresolved
catalog IDs without displaying raw payload values. Deletion remains an explicit
confirmed local action.

Encounter capture does not use Pixel Bus and has no relationship to input
authorization, Weaving, Fishing, or Auto Potion. It does not perform protected
actions, use items, change equipment, move the player, or generate input.

Continue to [Encounter Data and Metrics](../reference/encounter-data-and-metrics.md)
for the complete observation and analysis model.
