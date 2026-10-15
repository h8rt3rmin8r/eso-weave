# Encounter Capture

Encounter History imports retained saved-addon fights and completed supported
native encounter-log files. The single managed **PixelBeacon** package supplies
current screen signals and automatic Catalog collection. It does not load or
resume the former ESO Weave Data encounter recorder. Installing the combined
package does not start encounter recording.

**Application Log** and **File Logging** are troubleshooting logs, not fight
recordings. Native completed-file import supports normal/separate log version 15;
live tailing and automatic native recording enablement remain unavailable.

Recorded values stay on your computer. Names, identifiers and other values
supplied by ESO may be retained exactly. Hard limits and unsupported runtime
values cause a declared missing observation rather than a silently shortened
value. The desktop's ordinary summaries and application logs do not expose the
raw values.

## Completed native recordings

Finish a native recording in ESO or select a stable completed copy. Open
**File > Encounter History**, choose **Native Encounter Log**, select the intended
Live or PTS environment in Settings, and enter the native file path. Choose
**Inspect Native File** before **Import Native File**. Inspection creates no history;
accepted completed fights appear under **Native Observations**. Live or PTS is
user-selected provenance, not inferred from the native header. Follow
[Encounter Ingestion](../development/encounter-ingestion.md#completed-native-files)
for supported records, stable-file validation, and incomplete-fight handling.

## Import retained addon captures

Open **File > Encounter History** and choose **Import Saved Capture** to read the
selected environment's existing `SavedVariables/EsoWeaveData.lua`. This preserved
identity contains old encounter evidence alongside Catalog data. The desktop
copies accepted fights into local history without executing or changing the
source. **Refresh Saved Source** inspects the disk file; **Refresh Imported History** rereads desktop copies. Neither starts a recorder or imports new data.
The import and deletion contracts below remain supported for retained recordings.

## Historical addon recorder setup

The following recorder commands and capture semantics describe earlier releases.
They are retained to interpret existing files, not as setup instructions for the
current combined addon. Earlier releases offered a separate ESO Weave Data package
and required explicit in-game activation, such as `/ewencounter start live single`
or `/ewencounter start pts single` for the Public Test Server. The current package
neither registers these recorder commands nor restarts their saved authority.

To change the next session's mode or channel, stop recording first. Earlier
sessions keep their original mode and environment. Starting again keeps those
recordings; clearing is a separate deliberate deletion choice.

The historical Catalog collector and encounter recorder shared the
`EsoWeaveData` package and the game's `SavedVariables/EsoWeaveData.lua` file, with
separate module data. The current combined addon preserves that saved identity.
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
combat entry until the user runs `/ewencounter stop` or a reported hard failure stops it.

Historical commands were:

| Command | Result |
| --- | --- |
| `/ewencounter start live single` | Choose Live and single fight mode, then start; repeating start keeps an active recording |
| `/ewencounter start pts single` | Start one fight in the Public Test Server |
| `/ewencounter start live continuous` | Start continuous recording in Live; substitute pts for the Public Test Server |
| `/ewencounter stop` | Stop and retain recordings; repeating stop keeps them |
| `/ewencounter mode single\|continuous` | Select one of the two modes while recording is off; applies only to the next session |
| `/ewencounter channel live\|pts` | Select the next session's source channel while recording is off |
| `/ewencounter toggle` | Start recording with the chosen mode, or stop recording and retain the data already collected |
| `/ewencounter status` | Show OFF, WAITING FOR COMBAT or RECORDING, retained counts and the next action |
| `/ewencounter status details` | Show optional technical state, interruption and failure details |
| `/ewencounter clear confirm` | While recording is off, remove the addon's retained encounter recordings and controller state; preserve catalog collection and desktop history |
| `/ewencounter help` | Show the command summary |

Historical `arm`, `disarm`, `mode`, `channel` and `toggle` forms remain compatible.
Explicit start validates all arguments and capacity before changing configuration
or retaining the prior session. Repeated active start is idempotent. These
commands add no mode; retained evidence is never silently overwritten or evicted.

The prior channel-specific forms remain aliases for selecting and enabling
single mode:

```text
/ewencounter arm live
/ewencounter arm pts
```

## Historical start, resume, or discard

After a session stops, another start command begins a distinct session and keeps
the prior one. No clear or desktop-import acknowledgment is required. The addon
retains up to **32 sessions** within the same whole-spool observation, fight,
interruption and estimated-byte limits. Earlier sessions are immutable archives.

For a stopped continuous session, choose deliberately:

- **Resume** with `/ewencounter resume` to continue its original identity, mode,
  environment and next fight ordinal. A counted resume marker preserves the
  previous stop time. Finished fights remain unchanged.
- **Start** with `/ewencounter start live continuous` (or the matching environment/mode)
  to begin a distinct session and retain the previous one.
- **List** with `/ewencounter sessions` to see chronological IDs, provenance,
  status, counts and capacity.
- **Discard one** with `/ewencounter discard SESSION_ID confirm` while recording is
  off. It deletes exactly that addon session; other recordings, catalog data
  and desktop history remain. Save/import first only if you want to keep it.
- **Clear all** with `/ewencounter clear confirm` deliberately removes all addon
  recordings. It preserves the identity counter, so later sessions do not reuse
  earlier identities.

Only the latest stopped continuous session can resume. Single, failed and
archived sessions cannot resume. A valid failed session can be retained by a new
start when capacity permits. Full capacity refuses a new start or resume before
rotation or selection changes, and explains deliberate discard choices. It never
silently replaces or evicts data.

Supported older captures and controllers migrate without changing terminal
payloads or identities. Invalid or unsupported evidence stays preserved and
inactive. A reload during an active fight retains its explicit partial and
interruption facts; existing durably saved continuous authority can recover as
before. Stopping and resuming are separate from this reload recovery.

## Historical save and current import

1. Finish the fight in single mode. For continuous mode, stop with
   `/ewencounter stop` when you want to finish the session. Stopping during a
   fight retains a partial recording; stopping does not delete it.
2. Run `/reloadui`, log out, or exit ESO. This writes the addon's recordings
   to disk. A completed fight in memory may still be absent from the saved file.
3. In the desktop, use **File > Encounter History** and choose
   **Import Saved Capture**. It reads the fixed file for the environment selected
   in Settings and copies accepted fights into history on this computer.
4. Select an imported fight to view **Observed Metrics**, then **Provisional
   Recommendations**. Unavailable rates, missing observations and unresolved
   ability or effect definitions stay explicit. Review prompts are questions
   based on recorded facts, not proven causes or instructions to change a build.

**Refresh Imported History** rereads imported desktop history; it does not import new ESO data.
Import the saved file again to add later completed fights. Exact duplicates are
skipped, so repeated import does not multiply records. In a running continuous
session, completed saved fights can be imported while the current fight is
omitted. A disk file that is still changing must be saved and retried.

If Import is unavailable, check the selected environment and its AddOns folder,
then save the recording in ESO. While an import, calculation or delete is in
progress, wait for that operation to finish. A missing or incompatible catalog
can prevent calculation without removing the imported fight: review its
catalog status and select a compatible game-data catalog. See
[Troubleshooting](../getting-started/troubleshooting.md#retained-encounter-capture-or-import-fails)
for invalid, interrupted, duplicate and failed import recovery.

## Clear the intended copy

Historical recorder deletion commands below apply only to the former addon. The
current package preserves its SavedVariables and does not register these commands.

- `/ewencounter clear confirm` removes only the addon's encounter recordings and
  controller state while recording is off. It preserves catalog collection,
  addon files and previously imported desktop history. ESO saves the cleared
  state only at the next `/reloadui`, logout or exit. Import first if you want to
  preserve an importable recording; import is not required to clear it.
- **Delete Encounter** and **Delete All** in desktop history remove only the
  chosen imported fight or all imported fights on this computer. They preserve
  ESO's saved file, the combined addon, Catalog and settings. Importing the unchanged ESO file
  can bring deleted fights back.
- The former `/ewcollect clear confirm` command cleared only old Catalog data.
  Current PixelBeacon **Uninstall** removes the combined managed package while
  preserving the shared saved file, imported history and Catalog generations.

A historical recorder reporting an unsupported saved-data version preserved that
data and refused to clear it. The current importer also preserves unsupported
evidence; use a compatible reader rather than editing version fields. Historical
clear commands for a malformed supported state deliberately discarded recordings.

## Retained addon capture format

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

## Deterministic replay of retained captures

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

## Historical recorder bounds and recovery

The complete spool is limited to 32 sessions, 100,000 raw observations, 100,000 compatibility
events, 1,024 terminal or current encounters, 1,024 interruption markers, and a conservative
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

## Retained saved-file ownership and import

For captures produced by the historical recorder, use `/reloadui`, log out, or exit ESO so the client
writes `SavedVariables/EsoWeaveData.lua`. The file stays on your system. Open File,
Encounter History in the desktop and choose Import Saved Capture to import it
through the bounded non-executing path. The action uses the Live or PTS
environment selected in Settings and stores accepted raw history only in the
per-user application data directory.

The importer treats the SavedVariables file as hostile text, validates the
current controller, every archived session and every terminal member before
writing. It then selects the requested environment and commits all matching
records and per-session snapshots atomically. A repeated or growing valid spool is idempotent; any malformed
member, conflicting identity, changed prior prefix, or invalid ordinal rejects
the batch and preserves prior history. Session snapshots authenticate every
present member's mode, ordinal, identity, channel, and content hash before a
transaction commits. Store schema v6 accepts legacy capture v1
and raw-authority capture v2, including addon versions 2 and 3.
Migration copies legacy canonical bytes and hashes unchanged. If an older valid
store reused a session identity, read and migration assign stable deterministic
ordinals within that historical session without changing canonical evidence.
Store versions 1-5 remain readable and migrate transactionally on import.
Durable first-import session order preserves grouping after clock reversal,
record deletion and reimport; source session order comes from persisted creation
sequences, never wall-clock sorting.
Encounter History
retains session mode and ordinal, labels derived results as observed, and exposes
partial capture loss and unresolved catalog IDs without displaying raw payload
values. Deletion remains an explicit confirmed local action.

**Last saved source** inspects the selected saved file before an import. It
shows the selected Live or PTS environment, file path, last-save timestamp,
saved controller state, finished and partial/interrupted counts, matching
per-session provenance, and whole-file retained capacity. Inspection
is bounded and read-only; it does not create history. **Refresh Saved Source**
rereads that file, while **Refresh Imported History** rereads desktop copies.
Changing the environment or folder rejects an older pending inspection result.
Opening the workflow rereads the source. Successful and failed import attempts
clear and refresh its summary, so an import error cannot leave older readiness
or counts displayed as the current inspection.

The file can lag the historical recorder's activity. Its existence cannot prove
that a recorder is enabled, loaded or recording now. In former releases,
`/ewencounter status` reported current activity. An initialized file with no
retained session means no recording has been saved; repeated import cannot create
a fight. A stopped **empty session** means recording started and stopped before a
fight, so there are no encounters to import. Historical start/resume/clear choices
above explain retained controller states; the current addon does not resume them.

Historical command replies used ESO's system-chat route independently of the debug
sink. Replies waited for initializing chat; recording stayed off until its
enablement warning could be delivered. DebugLogViewer and LibDebugLogger required
no changes. The bounded 64-message reply queue declared any omitted overflow when
chat became ready. These are retained producer semantics, not current commands.

Encounter capture does not use Pixel Bus and has no relationship to input
authorization, Weaving, Fishing, or Auto Potion. It does not perform protected
actions, use items, change equipment, move the player, or generate input.
Completed normal/separate version 15 native files have their own Native Encounter
Log selection, inspection, import and factual metrics. Follow [Encounter Ingestion](../development/encounter-ingestion.md)
for that workflow. Live tailing remains unavailable. A closed tracking issue is
not a claim of field parity.

Continue to [Encounter Data and Metrics](../reference/encounter-data-and-metrics.md)
for the complete observation and analysis model.
