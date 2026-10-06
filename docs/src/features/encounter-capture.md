# Encounter Capture

**ESO Weave Data** records fights inside ESO so you can import them into the
desktop's **Encounter History** and inspect observed damage, healing, effects
and casts. Recording starts only when you turn it on inside the game. Choose
**single** for one fight or **continuous** for multiple fights until stopped.

PixelBeacon is the other addon: it supplies the desktop's current player-state
display and input features. Installing PixelBeacon does not install encounter
recording. **Application Log** and **File Logging** are troubleshooting
logs, not fight recordings. ESO's native combat-log files are another, still
provisional import route; use the saved-addon workflow below.

Recorded values stay on your computer. Names, identifiers and other values
supplied by ESO may be retained exactly. Hard limits and unsupported runtime
values cause a declared missing observation rather than a silently shortened
value. The desktop's ordinary summaries and application logs do not expose the
raw values.

## Install and choose a mode

1. In desktop **Settings**, select the ESO environment you actually use:
   **Live** for the released game or **PTS** for the Public Test Server.
2. In **System and State**, choose **Install Data** beside **ESO Weave Data**.
   If a managed copy needs replacement, choose **Update Data** or **Repair Data**.
   These actions install addon files; they do not start recording.
3. Enable **ESO Weave Data** in ESO's **Add-Ons** menu. Run `/reloadui` or log
   out and back in after installation, update, repair or an enablement change.
4. While recording is off, run `/ewencounter mode single` and
   `/ewencounter channel live`. Substitute `continuous` or `pts` as needed.
5. Run `/ewencounter toggle` to turn recording on. Run
   `/ewencounter status` to read its current state inside ESO.

To change mode or channel later, recording must be off **and** the retained
session must be cleared with `/ewencounter clear confirm`. Stopping alone keeps
that session. Save and import any usable fights you want to retain before
clearing, then choose the new mode/channel. Preservation through import is
optional; clearing deliberately discards the addon's retained recordings.

Catalog collection and encounter recording share the `EsoWeaveData` package and
the game's `SavedVariables/EsoWeaveData.lua` file, but retain separate data.
**SavedVariables** means ESO's saved addon data. The desktop reads the last
file ESO wrote and does not control the in-game recording toggle.

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
| `/ewencounter mode single\|continuous` | Select one of the two modes while off with no retained session; clear the old session deliberately before changing it |
| `/ewencounter channel live\|pts` | Select the source channel while off with no retained session |
| `/ewencounter toggle` | Start recording with the chosen mode, or stop recording and retain the data already collected |
| `/ewencounter status` | Show chosen mode, recording request, actual mode and state, current fight, session outcome, retained and finished fight counts, interruption and failure reason |
| `/ewencounter clear confirm` | While recording is off, remove the addon's retained encounter recordings and controller state; preserve catalog collection and desktop history |
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

## Save, import and view

1. Finish the fight in single mode. For continuous mode, stop with
   `/ewencounter toggle` when you want to finish the session. Stopping during a
   fight retains a partial recording; stopping does not delete it.
2. Run `/reloadui`, log out, or exit ESO. This writes the addon's recordings
   to disk. A completed fight in memory may still be absent from the saved file.
3. In the desktop, open **File > Encounter History** and choose
   **Import Saved Capture**. It reads the fixed file for the environment selected
   in Settings and copies accepted fights into history on this computer.
4. Select an imported fight to view **Observed Metrics**, then **Provisional
   Recommendations**. Unavailable rates, missing observations and unresolved
   ability or effect definitions stay explicit. Review prompts are questions
   based on recorded facts, not proven causes or instructions to change a build.

**Refresh** rereads imported desktop history; it does not import new ESO data.
Import the saved file again to add later completed fights. Exact duplicates are
skipped, so repeated import does not multiply records. In a running continuous
session, completed saved fights can be imported while the current fight is
omitted. A disk file that is still changing must be saved and retried.

If Import is unavailable, check the selected environment and its AddOns folder,
then save the recording in ESO. While an import, calculation or delete is in
progress, wait for that operation to finish. A missing or incompatible catalog
can prevent calculation without removing the imported fight: review its
catalog status and select a compatible game-data catalog. See
[Troubleshooting](../getting-started/troubleshooting.md#encounter-capture-is-waiting-interrupted-or-failed)
for invalid, interrupted, duplicate and failed import recovery.

## Clear the intended copy

- `/ewencounter clear confirm` removes only the addon's encounter recordings and
  controller state while recording is off. It preserves catalog collection,
  addon files and previously imported desktop history. ESO saves the cleared
  state only at the next `/reloadui`, logout or exit. Import first if you want to
  preserve an importable recording; import is not required to clear it.
- **Delete Encounter** and **Delete All** in desktop history remove only the
  chosen imported fight or all imported fights on this computer. They preserve
  ESO's saved file, both addons, catalog and settings. Importing the unchanged
  ESO file can bring deleted fights back.
- `/ewcollect clear confirm` clears only catalog collection, not encounter
  recordings. **Uninstall Data** removes both modules' managed addon files,
  while preserving the shared saved file, imported history and catalog.

If the addon reports an unsupported saved-data version, it preserves that data
and refuses to clear it. Update to a compatible ESO Weave release; do not edit
version fields to bypass the refusal. An invalid saved state of the supported
version may be cleared explicitly after accepting that those recordings will
be lost.

## Captured facts and implementation detail

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

Partial recordings, declared raw loss or clock discontinuity make replay indeterminate because an
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

Desktop controller facts are always labeled **Last saved recording state**.
They come from the last successfully imported saved file, not the newest disk
save or current activity. Refresh and failed imports keep that older summary;
changing the selected environment does not make it current. Check **Saved
channel** in the summary. It is never a live toggle, current addon state or
acknowledgement channel. Use `/ewencounter status`
inside ESO for current mode and controller state.

Encounter capture does not use Pixel Bus and has no relationship to input
authorization, Weaving, Fishing, or Auto Potion. It does not perform protected
actions, use items, change equipment, move the player, or generate input.
Native combat-log import remains provisional and does not change this supported
saved-addon workflow. A closed tracking issue is not a claim of field parity.

Continue to [Encounter Data and Metrics](../reference/encounter-data-and-metrics.md)
for the complete observation and analysis model.
