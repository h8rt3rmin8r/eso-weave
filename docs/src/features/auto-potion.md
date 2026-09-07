# Auto Potion

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

## Trigger contract

Every condition must hold in this order:

1. Auto Potion is requested for the current session.
2. ESO is active and focused.
3. A fresh PixelBeacon heartbeat is available.
4. ESO Weave is not suspended.
5. Game Context is positively observed as Gameplay.
6. Life State is positively observed as Alive.
7. At least one resource watch is enabled and a watched reading is fresh.
8. The active quickslot explicitly contains a usable potion.
9. Its cooldown is ready.
10. The minimum retry interval since the last attempt has elapsed.
11. At least one fresh watched resource is at or below its threshold.

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
  Game Context, watched-resource configuration or freshness, quickslot
  availability, potion classification or usability, cooldown, or retry interval.
- **Ready:** All prerequisites hold, but no watched resource is low.
- **Triggered:** Until the next evaluation, an attempt was submitted for a named
  resource, observed percentage, and threshold.

Generated input uses the shared input engine, including focus scoping and
injected-input recursion protection. Menu, suspension, life, world, travel, and
roll-dodge gates are checked at the action boundary. The controller checks menu
and suspension directly because its timers do not pass through the interception
decision. Losing the game, focus, or beacon blocks action without clearing the
requested setting. Requested enablement is not restored across application
restarts, unlike suspend and fishing intent. The controller ticks on the
pixel-bus worker, adds no thread or timer, and never reaches the hook thread.
Normal logging records categorical effective-state changes rather than every
evaluation.
