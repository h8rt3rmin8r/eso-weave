# Troubleshooting

Signal Lost may be described as a missing beacon, heartbeat timeout, or hidden
overlay. Start with the shared diagnostic flow below for any of those reports.

Start with the first failing observation, then follow the matching branch. Do not
disable focus checks, signal validation, suspension, or managed-addon guards to
make automation run.

## Shared diagnostic flow

```text
Is ESO detected and Active?
  No -> fix installation or runtime discovery.
  Yes -> is the ESO window focused?
    No -> focus ESO.
    Yes -> is PixelBeacon Installed (current)?
      No -> fix addon discovery or lifecycle state.
      Yes -> is PixelBeacon Signal detected?
        No -> check addon enablement, reload, overlay visibility, and geometry.
        Yes -> is Game Context Gameplay and required safety state available?
          No -> close menus or wait for fresh lifecycle evidence.
          Yes -> inspect the feature-specific status and Live Log.
```

The indented text is the complete decision tree. Each `No` branch stops before
feature-specific diagnosis because later observations depend on the earlier one.

## Game discovery and runtime

| Symptom | Meaning | Next action |
| --- | --- | --- |
| **Not detected** | No validated ESO installation or active game was found | Confirm ESO is installed for the current user and launch it normally |
| **Multiple installs detected** | More than one authoritative installation root conflicts | Close ESO Weave, identify the intended installation, and use the AddOns folder override only for PixelBeacon while provider ambiguity remains visible |
| **Unknown** | The operating-system observation did not authorize a conclusion | Retry after the launcher and game settle; inspect the Live Log if it persists |
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
5. Inspect the Live Log for an explicit pass-through emission error.

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

## Discovery collector capture or import fails

The discovery collector is separate from PixelBeacon. Start with
`collector-status` against the exact Live or PTS `AddOns` directory. An
`unmanaged` result is intentionally not repaired or removed automatically. A
successful install or removal can report `reload_required`; run `/reloadui` or
relog before relying on ESO's addon state.

In ESO, `/ewcollect status` reports the current capture state. Combat pauses a
run and requires `/ewcollect resume`. After completion, run `/reloadui`, log
out, or exit so ESO writes SavedVariables. Import the matching Live or PTS file;
channel, checksum, chunk, status, size, or schema errors are fail-closed and do
not overwrite the previous staged JSON. Do not edit the capture to bypass an
error. Preserve it for diagnosis and begin a fresh explicit run.

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
The request remains on through ordinary dormant and blocked states, but it always
starts off after an application restart.

## Configuration was rejected

Invalid `config.json` content falls back to safe defaults and the rejected file
is preserved with an `.invalid` suffix. Do not repeatedly edit or delete files
while ESO Weave is running. Close the application, preserve the rejected file for
diagnosis, then change only the exact invalid field or allow a fresh default file
to be written. Session-state load failures use safe defaults but do not have the
same `.invalid` preservation guarantee.

## Use the Live Log

Open **View > Live Log**. Select INFO for ordinary lifecycle diagnosis, DEBUG for
a bounded reproduction, or TRACE only when resource sampling detail is needed.
The dropdown changes and persists the global captured level for both the Live Log
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
