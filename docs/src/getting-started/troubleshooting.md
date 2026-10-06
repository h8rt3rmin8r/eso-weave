# Troubleshooting

Signal Lost may be described as a missing beacon, heartbeat timeout, or hidden
overlay. Start with the shared diagnostic flow below for any of those reports.

Start with the first failing observation, then follow the matching branch. Do not
disable focus checks, signal validation, suspension, or managed-addon guards to
make automation run.

For the optional authenticated extension service, including a stopped listener,
port collision, stale discovery, bearer rejection, protocol mismatch, database
limits, or client disconnect, use [Local API and MCP
troubleshooting](../reference/local-api-and-mcp.md#troubleshooting).

## Shared diagnostic flow

<figure class="docs-flow-diagram">

![Troubleshooting decision tree routes the first failing observation to startup, game, PixelBeacon, input, encounter, or feature evidence](../assets/diagrams/troubleshooting-decision-tree.svg)

</figure>

### Shared diagnostic flow text equivalent

```text
Does ESO Weave open?
  No -> inspect Startup failure evidence before or after GUI initialization.
  Yes -> is ESO detected and Active?
    No -> fix installation or runtime discovery.
    Yes -> is the ESO window focused and Game Context Gameplay?
      No -> focus ESO, close menus, or wait for fresh lifecycle evidence.
      Yes -> is PixelBeacon Installed (current)?
        No -> fix addon discovery or lifecycle state.
        Yes -> is PixelBeacon Signal detected?
          No -> check addon enablement, reload, overlay visibility, and geometry.
          Yes -> are the platform input path and native binding evidence valid?
            No -> is native binding evidence Unavailable?
              Yes -> update or reload PixelBeacon and restore the shared signal first.
              No -> use the platform input guidance and native binding state table.
            Yes -> is encounter capture or import the first failing observation?
              Yes -> inspect addon status, saved authority, receipt, loss, and validation.
              No -> inspect the feature-specific status and Application Log.
```

The indented text is the complete decision tree. Each matching failure branch
stops before feature-specific diagnosis because later observations depend on
the earlier one.

## Game discovery and runtime

| Symptom | Meaning | Next action |
| --- | --- | --- |
| **Not detected** | No validated ESO installation or active game was found | Confirm ESO is installed for the current user and launch it normally |
| **Multiple installs detected** | More than one authoritative installation root conflicts | Close ESO Weave, identify the intended installation, and use the AddOns folder override only for PixelBeacon while provider ambiguity remains visible |
| **Unknown** | The operating-system observation did not authorize a conclusion | Retry after the launcher and game settle; inspect the Application Log if it persists |
| **Launcher open** | The launcher is present but the game client is not active | Start the game client |
| **Inactive** | The launcher and game client are absent | Start ESO |

Game exit clears game-derived observations and held-key state. Returning to the
game requires fresh focus, heartbeat, surface, life, world, travel, and roll
evidence. Dropped work is not replayed.

## Linux input or focus does not work

1. Confirm the current user has either `input` group membership or the packaged
   `/dev/uinput` udev rule described in [Installation](installation.md#linux).
2. If group membership changed, sign out and back in.
3. Confirm ESO runs under X11 or XWayland. Pure Wayland cannot provide the active
   X window and capture evidence required by the current application.
4. Confirm the physical keyboard exposes the expected keys and no other program
   has exclusively grabbed it.
5. Inspect the Application Log for an explicit pass-through emission error.

Do not use recursive permission changes or world-writable device modes.

## PixelBeacon Status is not current

| Visible state | Next action |
| --- | --- |
| **Not installed** | Choose **Install**, then `/reloadui` or relog if ESO is running |
| **Installed (outdated)** | Choose **Update** to replace the managed copy in place; then reload ESO |
| **Unmanaged (not modified)** | No lifecycle action is offered; move or remove only that exact target manually, then install a managed copy |
| **AddOns folder not found** | In Settings, select Live or PTS correctly, or enter the existing environment's `AddOns` directory as the override |

An existing PixelBeacon target whose ownership cannot be proven is shown as
**Unmanaged (not modified)**. Install, Update, Uninstall, API refresh, and
block-size redeploy refuse it without changing its contents. Resolve only that
exact target manually; do not disable the ownership guard.

## PixelBeacon Signal is missing or lost

1. Confirm PixelBeacon is enabled in ESO's addon list.
2. Run `/reloadui` or relog after install, update, or removal.
3. Keep the top-left color-block overlay visible and unobstructed.
4. If Block Size changed, redeploy the managed addon, reload ESO, and restart ESO
   Weave so both sides use the same physical geometry.
5. Wait until loading ends and **World State** returns to **Active**.
6. Use [PixelBeacon quickslot diagnostics](../features/pixelbeacon.md#fishing-signal-and-quickslot-diagnostics)
   only after the shared signal is healthy.

Missing, invalid, stale, or corrupt telemetry becomes unavailable and does not
authorize automated input.

The Live HUD may continue showing the last coherent values after a loss. Check
the logs for loss, cause changes, expiry, and recovery. No temporary notification
appears above the gauges. These values are
display-only and do not keep weaving, Fishing, or Auto Potion authorized. Fresh
observations replace them immediately; the configured **Stale Retention
(seconds)** interval then expires to the ordinary unavailable view. Set it to 0
under Appearance when immediate clearing is preferred.

## Native binding evidence is unavailable

PixelBeacon discovers bindings without changing ESO controls. Weaving, Fishing,
and Auto Potion consume their current action subset without a desktop fallback.
The evidence distinguishes these states for each native ESO action:

| State | Meaning | Safe next action |
| --- | --- | --- |
| Unavailable | PixelBeacon is old, the signal is stale or malformed, or ESO has not loaded binding data | Update PixelBeacon, reload ESO, and restore the shared signal first |
| Unbound | ESO has no keyboard or mouse assignment for the action | Bind the action in ESO's Controls menu |
| Conflicting | More than one distinct desktop assignment exists | Leave one intended keyboard or mouse chord in ESO |
| Unsupported | The only assignment is gamepad, combined, hold, unknown, or otherwise outside the portable registry | Choose an ordinary keyboard key or supported mouse control in ESO |
| Valid | One supported primary and normalized modifier set was decoded | The corresponding weave, Fishing, or Auto Potion action is usable now |

Do not install a custom binding addon or edit `Bindings.xml` to repair evidence.
PixelBeacon uses only ESO's read-only runtime binding APIs and deliberately has no
binding mutation or persistence path.

Keyboard or mouse bindings can coexist with positively identified controller
bindings. Controller bindings do not create desktop conflicts. Controller-only
actions remain unsupported; unrecognized device evidence does not authorize a
guessed desktop control. Update PixelBeacon and reload ESO to receive this
discovery repair.

If valid combat evidence still passes through, confirm the skill row is enabled
and required Attack or Block evidence is also valid. Release extra physical
modifiers that are absent from a target chord. ESO Weave refuses to synthesize a
release for a modifier you are holding.

If Fishing or Auto Potion reports a binding unavailable state, inspect the
read-only detected row in Settings, repair the action in ESO, and reload the UI.
Do not add the obsolete gameplay key back to `config.json` because it is ignored.

## ESO Weave Data is missing, outdated, unmanaged, or awaiting reload

- **Not installed**: choose **Install Data**, then obey any reload guidance.
- Addon Package Version **Update available**: choose **Update Data** or **Repair Data**;
  both stage and verify replacement before commit.
- Addon Management **Unmanaged**: move or remove only the exact `EsoWeaveData` target
  manually. No lifecycle action is offered.
- **AddOns folder not found**: select the correct Live or PTS environment or
  configure its existing `AddOns` directory.
- Reload Reminder **Required**: run `/reloadui` or relog before relying on the change.
- Lifecycle operation failed: keep the prior package, check AddOns permissions,
  and retry the named operation. Do not delete first.

Installed files, enablement, loading, the game process, catalog collection and
encounter recording answer different questions. **Unknown (check inside ESO)**
does not mean disabled. Enable ESO Weave Data in ESO's Add-Ons menu, reload after
changes and run `/ewcollect status` or `/ewencounter status` inside the game for
current activity. The saved addon file shows only what ESO wrote at its last
save, not what the addon is doing now.

If a newer game API is unsupported, update ESO Weave to a release that declares
support. An unknown or failed API check cannot prove compatibility; the desktop
keeps remembered unsupported-version guidance visible. Live and PTS have
separate API evidence and addon locations.

## Encounter capture is waiting, interrupted, or failed

Run `/ewencounter status` inside ESO. The desktop can show only a validated
**Last saved recording state** from the last successfully imported saved file.
Refresh and failed imports keep this older summary, which can predate the newest
disk save or selected-environment change. Check **Saved channel** in that history
summary, and use the in-game command for current activity. Recording is
controlled inside ESO. Start with the complete
[setup sequence](../features/encounter-capture.md#install-and-choose-a-mode).

- **Recording off**: mode/channel changes require no retained session. If one
  remains, optionally save/import wanted fights, then deliberately use
  `/ewencounter clear confirm` before choosing `single` or `continuous` and
  `live` or `pts`. Start the chosen mode with `/ewencounter toggle` inside ESO.
- **Waiting for combat**: the selected mode is on, but no fight is being recorded;
  detailed capture handlers are
  dormant until combat starts. Toggle again to disable it.
- **Recording combat**: the current encounter is recording. Toggling off finalizes it
  as a user-stopped partial record.
- **Recording interrupted**: reload or relog created an explicit gap. Single stops;
  continuous resumes only from valid durably saved authority and retains the
  interruption fact.
- **Stopped after a failure**: callback, clock, recovered-state, marker, or storage pressure made
  continued capture unsafe. Retained evidence remains in place and the failed
  request does not retry automatically. Save and import usable recordings if
  you want to retain them, then follow the reported clear/restart guidance.

A malformed same-version controller is never normalized by guessing. The addon
preserves it unchanged, keeps it inactive, and reports `state-invalid` through
`/ewencounter status`; use `clear confirm` only when you intentionally accept
discarding that unimportable state.
An unsupported saved-data version is different: the module preserves it and
refuses clear as well as recording. Update to a compatible release; do not
change version fields to bypass that refusal.

Single mode enabled during combat truthfully marks the unseen prefix and is
partial even when the next combat exit is observed. Continuous encounters share
one session identity but have separate contiguous ordinals, raw sequences, and
replay outcomes. A missing ordinal, changed prior member, malformed spool, or
identity collision rejects the complete desktop import batch without changing
existing history.

No old encounter is automatically removed when saved recording storage reaches its record,
marker, or 32 MiB estimated-data limit. The controller preserves terminal and
failure evidence and stops. Run `/reloadui`, relog, or exit ESO to flush data,
then use **Import Saved Capture** in Encounter History. An ESO or operating-system
crash before a flush can lose unflushed control and encounter state; the product
does not invent a recovery marker for facts that never reached disk.

For desktop history, distinguish these outcomes:

- **No imported encounters**: save a recorded fight in ESO, then choose **Import
  Saved Capture**. **Refresh** rereads desktop history only.
- Import unavailable: check the selected Live or PTS environment and AddOns
  folder. Enable and load ESO Weave Data, record a fight, then save in ESO.
- An operation is in progress: wait for import, refresh, detail calculation or
  deletion to finish before requesting another operation.
- No new fights or duplicates: exact repeated recordings are skipped. Save later
  completed fights and import again. A currently active fight is not a completed
  member and may be omitted from a growing saved session.
- Missing, unstable, invalid or incompatible saved data: save again, check the
  correct environment and supported package version, then retry. A malformed
  member, changed earlier record or identity conflict rejects the whole batch
  and preserves existing history. Preserve the source for diagnosis; do not
  edit it to defeat validation.
- Calculation unavailable: a missing or incompatible catalog can leave imported
  summaries visible while metrics are unavailable. Review the catalog's exact
  channel/API match; a known ID of another entity kind cannot supply a definition.
- **Incomplete observations** or unknown definitions: keep the declared missing
  sequence ranges and unresolved IDs in view. Missing duration produces an
  unavailable rate, not zero. Review prompts may be qualified or unavailable.
- Delete failed: imported records remain available unless the operation reports
  successful removal. Check application-data permissions and retry the named
  action. **Delete Encounter** and **Delete All** preserve ESO's saved file, so
  importing that unchanged file can restore deleted fights.

## Discovery collector capture or import fails

The catalog module is separate from PixelBeacon but shares the managed ESO Weave
Data package with encounter capture. Use the first-class System and State row for
package lifecycle. Maintainers may also run `collector-status` against the exact
Live or PTS `AddOns` directory. An `unmanaged` result is intentionally not
repaired or removed automatically.

In ESO, `/ewcollect status` reports current catalog collection, not fight recording. Combat pauses a
run and requires `/ewcollect resume`. After completion, run `/reloadui`, log
out, or exit so ESO writes SavedVariables. Import the matching Live or PTS file;
channel, checksum, chunk, status, size, or schema errors are fail-closed and do
not overwrite the previous staged JSON. Do not edit the capture to bypass an
error. Preserve it for diagnosis and begin a fresh explicit run.

`/ewcollect cancel` retains an incomplete, unimportable collection; it does not
erase it. Starting a new run after a finished, cancelled or failed one replaces
the previous catalog collection. A reload does not reconstruct the running
collector. Preserve a completed collection before starting another, and check
status after interruptions.

## A local ability icon uses the placeholder

S072 provides a library cache boundary but does not activate icon selection in
the application. For direct library use, confirm that the explicitly selected
source root mirrors the complete catalog virtual path and contains a PNG or DDS
file. `Missing` and `Unsupported` identify absence or file type. `Failed`
includes unsafe paths, links or reparse points, permissions, corruption,
dimension or byte limits, and cache integrity failures.

Do not move game image bytes into the repository or a release to repair a local
lookup. Keep them in the user-owned source and application-data cache. Remove or
replace only the exact failed local cache generation after preserving it for
diagnosis; another verified immutable generation remains usable.

## A catalog review candidate fails

Candidate generation is a maintainer operation and does not affect the catalog
used by the application. Read `validation.json`, `sources.json`, and the command
error before retrying. Fix the pinned request or source rather than changing a
hash to bypass verification. A failed source refresh uses cached bytes only when
the request explicitly permits stale fallback, and the resulting source report
labels that choice. Existing immutable candidates remain unchanged.

## A user catalog update or rollback fails

Open **File > Catalog Update...** and read the named failing stage. A malformed,
linked, incomplete, wrong-channel, unsupported-schema, older, checksum-invalid,
or policy-invalid candidate is intentionally unavailable for Live installation.
Replace the complete hash directory in `catalog/import/live` from the trusted
review source, then choose **Refresh candidates**. Do not edit manifests or
checksums to bypass verification.

Cancellation before selection preserves the preceding target. During the short
Selecting catalog stage, allow the atomic operation to finish. On restart, the
worker removes only abandoned `.update-*` staging and never accepted versions.
If a user selection is invalid, the status visibly identifies the bundled
fallback. A receipt warning means the selection itself succeeded but redacted
receipt storage failed, commonly because the application-data volume is full.

For a collector-assisted build, manage ESO Weave Data from System and State,
then choose **Watch for saved catalog data** before the ESO save
boundary. Then run `/reloadui`, log out, or exit and choose **Build from flushed
capture**. Unchanged, unstable, incomplete, PTS, or coverage-reducing captures
remain unaccepted. Module-local clear guidance never removes the shared package,
shared SavedVariables, or PixelBeacon.

## A skill passes through or a weave is dropped

Check the Skills row first. It must be enabled, bound to the physical key, and
outside the configured Global Cooldown. Then confirm ESO is active and focused,
ESO Weave is not suspended, Game Context is **Gameplay**, Life State is **Alive**,
World State is **Active**, Travel is **Inactive**, and Roll Dodge is **Inactive**.

A blocked physical skill passes through where the input gate can decide safely.
A request dropped after queueing is not replayed. Focus loss, suspension, or
menu-gate closure invalidates the authorization epoch for queued and running
work. Any output already held is released, but no new generated press follows
the closure.

## Fishing returns to Idle

Use the exact suffix:

- **no cast detected**: select bait, face the fishing-hole prompt, and start with
  `F2` rather than casting manually.
- **signal lost**: restore the PixelBeacon heartbeat before starting again.
- **game not active** or **game unfocused**: return to an active, focused ESO
  window. With the other safety gates clear, the retained request automatically
  attempts a fresh cast.
- **player unavailable**, **world unavailable**, or **travel pending**: wait for
  Alive, Active, and Inactive evidence, then make a fresh start as described on
  the [Fishing page](../features/fishing.md#status-meanings).

## Auto Potion is Dormant or Blocked

The Auto Potion line names the first current blocker. Use the
[Auto Potion state table](../features/auto-potion.md#status-and-recovery) in order.
The request remains on through ordinary dormant and blocked states and across a
normal application restart. After restart, wait for fresh current evidence;
remembered enablement does not make a missing or stale gate permissive.

## Configuration was rejected

Invalid `config.json` content falls back to safe defaults and the rejected file
is preserved with an `.invalid` suffix. Do not repeatedly edit or delete files
while ESO Weave is running. Close the application, preserve the rejected file for
diagnosis, then change only the exact invalid field or allow a fresh default file
to be written. Session-state load failures use safe defaults but do not have the
same `.invalid` preservation guarantee.

## Use the Application Log

Open **View > Application Log**. Select INFO for ordinary lifecycle diagnosis, DEBUG for
a bounded reproduction, or TRACE only when resource sampling detail is needed.
The dropdown changes and persists the global captured level for both the Application Log
ring and optional file logging.

Enable **Write Log to File** before reproducing a problem that must survive the
current session. See [Logging](../reference/logging.md) for paths and privacy.

## Startup failure

After logging initializes, a panic is written to the application log. A very
early panic may occur before that best-effort initialization. Before the GUI
event loop starts, the application surfaces the failure to the user: Windows shows a native
**ESO Weave failed to start** dialog, while Linux writes the same notice to
standard error. After the GUI starts, later panics use the initialized log only so an unexpected
dialog cannot interrupt play.

Record the exact dialog or terminal text, package and version, operating system,
and whether a monthly log was created. Re-verify the package checksum, then report
the evidence without including private paths or unrelated log data.
