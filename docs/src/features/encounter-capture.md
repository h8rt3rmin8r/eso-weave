# Encounter Capture

The `EsoWeaveData` encounter module is an explicit local observation tool.
It records selectively subscribed encounters into a bounded ESO SavedVariables
spool so later work can import and analyze the same ordered facts outside the
game. Every scalar
value delivered by a selected source is retained exactly unless a declared hard
limit or unsupported runtime type prevents the whole observation from fitting.

This capture is not automatic. Loading or installing the addon does not authorize
recording. It remains dormant until you explicitly enable one of exactly two
modes inside ESO: `single` or `continuous`.

## Install and choose a mode

The desktop lifecycle controls install the exact package under
`addon/EsoWeaveData`. Catalog and encounter state share one versioned outer
SavedVariables root, but each command mutates only its own module subtree. The
desktop does not send commands to the addon and never writes live
SavedVariables. S096 advances the managed package manifest to version 2 so an
existing managed installation is offered the controller update.

Choose the mode and channel while capture is off, then use the same toggle to
start or stop it:

1. `/ewencounter mode single`
2. `/ewencounter channel live`
3. `/ewencounter toggle`

Use `continuous` instead of `single` for an explicitly enabled multi-fight
session. The channel remains a separate Live or PTS choice, not another mode.
The enable message warns that exact local data can include account, character,
unit, ability, effect, and other identifiers supplied by ESO.

Single mode enabled outside combat waits for the next combat entry and stops at
that encounter's exit. If enabled during combat, it starts immediately from an
exact `IsUnitInCombat("player")` observation, labels the unseen pre-activation
prefix, and stops at the next exit. That record is partial because the addon
cannot reconstruct observations from before the user's action.

Continuous mode creates one bounded session and a separate encounter record for
each combat period. It removes detailed handlers during gaps, preserves the
session identity, assigns contiguous encounter ordinals, and waits for the next
combat entry until the user toggles it off or a reported hard failure stops it.

Use these additional commands:

| Command | Result |
| --- | --- |
| `/ewencounter mode single\|continuous` | Select one of the two modes while off |
| `/ewencounter channel live\|pts` | Select the source channel while off |
| `/ewencounter toggle` | Enable the selection or disable current authority |
| `/ewencounter status` | Show selected/requested/active mode, channel, state, current-presence, session disposition, counts, last interruption, and failure class |
| `/ewencounter clear confirm` | Clear retained encounter evidence while off |
| `/ewencounter help` | Show the command summary |

Historical `arm`, `disarm`, and `stop` forms may remain compatibility aliases to
these transitions. They do not add a mode. Retained evidence is never silently
overwritten or evicted; clear it explicitly when the status instructs you to do
so.

The prior channel-specific forms remain aliases for selecting and enabling
single mode:

```text
/ewencounter arm live
/ewencounter arm pts
```

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

The complete include and exclude matrix is maintained in the S095 contract.
Selective
subscription controls addon cost; lossless retention controls what happens after
a selected callback is delivered. S095 adds no new event family.

## Deterministic replay

Addon version 3 keeps capture schema v2 and records a bounded normalization
profile containing the ESO runtime enum values used by the normalizer. On desktop
import, a pure Rust reprocessor derives normalized events only from that profile
and the raw observation stream. Compatibility events are comparison input, not
replay input. A complete current capture is accepted only when source links,
projection ordinals, times, kinds, and payloads match exactly.

Declared raw loss or clock discontinuity makes replay indeterminate because an
omitted observation can change later actor numbering or capacity decisions. Such
captures retain the existing partial, degraded-evidence behavior. Schema-v1 and
pre-profile schema-v2 history remains importable with replay explicitly
unavailable. No legacy evidence is fabricated or rewritten. Errors identify a
controlled failure class but never display compared values.

## Bounds, ordering, and incomplete capture

The complete spool is limited to 100,000 raw observations, 100,000 compatibility
events, 1,024 terminal encounters, 1,024 interruption markers, and a conservative
32 MiB estimated encounter-data budget. Each observation remains limited to 256
tagged values, each encounter to 4,096 derived actor mappings, and each raw string
to 64 KiB. Exact serialized size depends on ESO and is not claimed by the in-game
estimate.

Capacity is reserved for both encounter terminal evidence and the outer session
failure fact. A record, byte,
or string limit and an unsupported value omit the entire raw observation rather
than truncating it. Raw sequencing then continues in constant memory and one
`raw_loss` range declares exact missing sequences and the first loss reason.
Backward clock movement creates a retained temporal-discontinuity observation.
Reload, deactivation, explicit disablement during combat, or callback failure
also produces a partial result instead of a false complete result. Aggregate
exhaustion stops the controller as a visible hard failure. No limit rolls old
evidence away.

One continuous session identity links its encounters, while a positive contiguous
ordinal is the authoritative cross-encounter order. Event and raw-observation
sequences restart inside each independently replayable encounter. Repeated or
backward wall-clock values never decide session order.

Reload and relog recovery use only the last durably flushed explicit authority.
A recovered active encounter becomes partial with the controlled
`runtime-interrupted` reason and a counted `recovered_interruption` warning.
Continuous mode may return to waiting under the same session. A
reloaded or deactivated waiting single session also stops with an interruption
marker, while a waiting continuous session can remain enabled. A failed session
never retries automatically. A same-version malformed controller is preserved
unchanged, treated as inactive `state-invalid` evidence, and never executed;
`status` reports the hard failure and only an explicit `clear confirm` removes it.
A desktop exit does not
change addon authority, and an ESO or operating-system crash before a flush is
unknowable rather than reconstructed.

## Save and ownership boundary

After one or more captures, use `/reloadui`, log out, or exit ESO so the client
writes `SavedVariables/EsoWeaveData.lua`. The file stays on your system. Open File,
Encounter History in the desktop and choose Import Saved Capture to import it
through the bounded non-executing path. The action uses the Live or PTS
environment selected in Settings and stores accepted raw history only in the
per-user application data directory.

The importer treats the SavedVariables file as hostile text, validates the
controller and every terminal member before writing, then commits the whole
batch atomically. A repeated or growing valid spool is idempotent; any malformed
member, conflicting identity, changed prior prefix, or invalid ordinal rejects
the batch and preserves prior history. Session snapshots authenticate every
present member's mode, ordinal, identity, channel, and content hash before a
transaction commits. Store schema v4 accepts legacy capture v1
and raw-authority capture v2, including addon versions 2 and 3.
Migration copies legacy canonical bytes and hashes unchanged. If an older valid
store reused a session identity, read and migration assign stable deterministic
ordinals within that historical session without changing canonical evidence.
Encounter History
retains session mode and ordinal, labels derived results as observed, and exposes
partial capture loss and unresolved catalog IDs without displaying raw payload
values. Deletion remains an explicit confirmed local action.

Desktop controller facts are always labeled **Last saved capture state** or
historical. They can be stale while ESO runs and are never presented as a live
toggle, current addon state, or acknowledgement channel. Use `/ewencounter status`
inside ESO for current mode and controller state.

Encounter capture does not use Pixel Bus and has no relationship to input
authorization, Weaving, Fishing, or Auto Potion. It does not perform protected
actions, use items, change equipment, move the player, or generate input.
Native encounter-log qualification remains independent under issue #190 and does
not change this SavedVariables control or fallback path.

Continue to [Encounter Data and Metrics](../reference/encounter-data-and-metrics.md)
for the complete observation and analysis model.
