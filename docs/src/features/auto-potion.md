# Auto Potion

A Resource Watch is a Health, Magicka, or Stamina threshold, often described as
a low-resource trigger.

Auto Potion presses the active quickslot binding when an enabled resource reaches
its configured threshold. It is the only feature that turns resource telemetry
into generated input, so unavailable evidence always blocks it.

The feature starts off after every application launch. Configure at least one
Health, Magicka, or Stamina watch, then press `F3` or use the Auto Potion toggle.
Each watch has its own threshold. The rule is an OR: any enabled, fresh resource
at or below its threshold can qualify.

Every resource watch is off by default. Requiring all three resources to be low
would wait until a potion no longer helps, so the OR rule is not configurable.
Independent enables and thresholds keep that rule visible in the interface and
allow different limits for each resource.

The Quickslot key defaults to `Q`, ESO's default quickslot binding, and remains
configurable.

## Configure Auto Potion

1. Put the intended potion in ESO's active quickslot. ESO Weave does not choose
   or rotate the consumable wheel for you.
2. Open **Settings > Auto Potion**.
3. Enable at least one of Watch Health, Watch Magicka, or Watch Stamina. Each
   threshold defaults to 35 percent and accepts 0 through 100 percent.
4. Confirm **Quickslot Key** matches the in-game binding. It defaults to `Q`.
5. Leave **Minimum Retry Interval** at its 1500 ms default unless diagnosis shows
   the screen observation lags long enough to permit repeat attempts.
6. Close Settings, verify PixelBeacon and Quickslot status, then use `F3` or the
   **Auto Potion** toggle.

## Trigger contract

Every condition must hold in this order:

1. Auto Potion is requested for the current session.
2. ESO is active and focused.
3. A fresh PixelBeacon heartbeat is available.
4. ESO Weave is not suspended.
5. Game Context is positively observed as Gameplay.
6. Life State is positively observed as Alive.
7. World State is positively observed as Active.
8. Travel is positively observed as Inactive.
9. Explicit Sprinting is not present. Unknown movement does not block.
10. At least one resource watch is enabled and at least one enabled watch has a
    fresh reading.
11. The active quickslot explicitly contains a usable potion.
12. Its cooldown is ready.
13. The minimum retry interval since the last attempt has elapsed.
14. At least one fresh watched resource is at or below its threshold.

An unreadable resource is not low, an unreadable quickslot is not a potion, and
an unreadable cooldown is not ready. Loading, addon reload, or signal loss
therefore produces no keypress. Fresh positive observations must return before
the controller can become Ready or Triggered again. This fail-closed direction
is deliberate: treating unknown as permissive would fire during beacon outages,
addon reloads, and loading screens.

The retry interval is separate from the quickslot cooldown. The screen signal
can lag behind the generated keypress by at least one sampling interval, so the
retry floor prevents repeated attempts until cooldown telemetry catches up. It
defaults to 1500 ms.

Auto Potion does not infer which resources a potion restores, choose a potion,
or change the active quickslot. The operator chooses the resource watches and
quickslot item.

## Effective states

- **Off:** The feature was not requested.
- **Dormant:** ESO is inactive or unfocused.
- **Blocked:** The first current failure is beacon availability, suspension,
  Game Context, Life State, World State, travel, explicit Sprinting,
  watched-resource configuration or freshness, quickslot availability, potion
  classification or usability, cooldown, or retry interval.
- **Ready:** All prerequisites hold, but no watched resource is low.
- **Triggered:** Until the next evaluation, an attempt was submitted for a named
  resource, observed percentage, and threshold.

## Status and recovery

The interface displays the first current blocker in evaluation order.

| Visible status | Meaning and recovery |
| --- | --- |
| **Off** | The feature was not requested. Use `F3` or the toggle after configuration |
| **Dormant: game inactive** | Start the ESO game client; the request is retained |
| **Dormant: game unfocused** | Focus ESO; the request is retained |
| **Blocked: beacon unavailable** | Restore a fresh PixelBeacon Signal |
| **Blocked: input suspended** | Resume ESO Weave with `F1` or the Running toggle |
| **Blocked: game context** | Close native menus and text entry, then return to Gameplay |
| **Blocked: life state unavailable**, **player dead**, or **reincarnating** | Wait for a fresh Alive observation |
| **Blocked (world unavailable)** | Wait until World State is Active |
| **Blocked (travel pending)** | Finish or cancel travel and wait for Inactive |
| **Blocked: sprinting** | Stop explicit on-foot sprinting; Unknown movement is not treated as Sprinting |
| **Blocked: no watched resource** | Enable at least one resource watch |
| **Blocked: resources unavailable** | Restore fresh telemetry for an enabled watch |
| **Blocked: quickslot unavailable** | Restore current quickslot classification telemetry |
| **Blocked: no potion selected** | Select a potion in the active ESO quickslot |
| **Blocked: potion unavailable** | Refill a depleted potion or resolve ESO's unusable state |
| **Blocked: potion cooldown** | Wait for an explicit ready cooldown observation |
| **Blocked: retry interval** | Wait for the configured retry floor after the last attempt |
| **Ready** | Every prerequisite holds and no watched resource is currently at or below threshold |
| **Triggered: RESOURCE at N% (threshold T%)** | One press and release was submitted for the named current reading |

Roll Dodge is not an Auto Potion prerequisite. It gates weaving and physical
skill interception, while explicit Sprinting defers Auto Potion.

## Worked example

With Watch Health enabled at 35 percent, fresh Health at 34 percent qualifies
because the comparison is at or below. If the active Quickslot is a usable
potion, its cooldown is ready, and every earlier gate is safe, one press and one
release of the configured Quickslot Key are submitted. The 1500 ms retry floor
then blocks another attempt while the displayed cooldown catches up. After any
failure recovers, the controller evaluates current readings; it does not replay a
stored trigger.

Generated input uses the established platform input backend, including
injected-input recursion tagging.
The controller checks menu and suspension directly. It also checks focus, life,
world, travel, and explicit Sprinting because its timers do not pass through the
interception decision.
Roll Dodge is not an Auto Potion prerequisite.
It gates weaving and physical-input interception, not the controller's quickslot
attempt. Losing the game, focus, or
beacon blocks action without clearing the requested setting. Requested
enablement is not restored across application restarts, unlike suspend and
fishing intent. The controller ticks on the pixel-bus worker, adds no thread or
timer, and never reaches the hook thread. Normal logging records categorical
effective-state changes rather than every evaluation.

See the [Status Reference](../reference/status-reference.md#auto-potion) for exact
state vocabulary and [Troubleshooting](../getting-started/troubleshooting.md#auto-potion-is-dormant-or-blocked)
for the shared diagnostic order.
