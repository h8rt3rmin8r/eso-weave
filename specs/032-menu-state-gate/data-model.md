# Data Model: PixelBeacon Menu-State Input Gate

**Feature**: 032-menu-state-gate | **Date**: 2026-07-27

In-memory only. Nothing here is persisted; the gate has no user setting.

## MenuSurface

Which game UI surface is active, decoded from B5.

| Variant | Code | Gate |
| --- | --- | --- |
| `None` | 0 | inactive |
| `SystemMenu` | 1 | active |
| `Map` | 2 | active |
| `Inventory` | 3 | active |
| `Mail` | 4 | active |
| `Character` | 5 | active |
| `GuildStore` | 6 | active |
| `CrownStore` | 7 | active |
| `Journal` | 8 | active |
| `ChatEntry` | 9 | active |
| `Other` | 10 | active |

**Default**: `None`. This is also the value produced by every failure mode
(absent block, failed validation, lost signal), which is what makes every failure
degrade to the application's current behavior.

**Derived predicate**: `gate_active = surface != None`. The gate never depends on
recognizing a specific surface, only on the value not being `None`, so a surface
the addon could not name (`Other`) still gates.

## Gate flags

Two independent flags, each defaulting to inactive, each set through a method.

| Holder | Flag | Effect while active |
| --- | --- | --- |
| Input engine | `menu_gated` | The interception decision passes every non-exempt key through |
| Fishing controller | `gated` | No new interact keypress is initiated |

Both default to inactive because inactive means "behave as the application does
without this feature". That default is what allows every existing test to keep
passing untouched, and what makes the failure modes safe by construction rather
than by handling.

**State transitions**: set from the decoded surface on every change; both flags
always hold the same value, derived from the one signal.

## PixelBusEvent::MenuGate

Carries a `MenuSurface`, emitted only when the decoded surface changes. Routed to
both gate holders. Returns `None` from the fishing detector mapping, so it never
reaches the fishing state machine as a detector event; it reaches the controller
only as a flag.

## BlockSamples

Gains a `menu: Option<Rgb>` field for B5. The struct's derived `Default` means no
existing construction changes, which is the extensibility property slice 031 added
this struct to provide.

## MenuView

Display state for the interface, mirroring the combat and weapon-bar views.

| Field | Type | Derivation |
| --- | --- | --- |
| `detected` | `bool` | a signal has been decoded |
| `state` | `&'static str` | the surface name, "Gameplay", or "Not detected" |
| `role` | `StatusRole` | `Active` when detected, `Muted` otherwise |

## Cadence selection

`poll_interval` gains one argument.

| Fishing active | Can intercept | Interval |
| --- | --- | --- |
| yes | either | fast |
| no | yes | fast |
| no | no | idle |

"Can intercept" is false when the application is suspended, because a suspended
application neither intercepts nor synthesizes and therefore has nothing to gate.
This is what keeps the idle setting meaningful rather than dead.

## Constants shared byte for byte

| Name | Value |
| --- | --- |
| `NUM_BLOCKS` | `6` |
| `MENU_MARKER` | `0xD2` |
| `MENU_CODE_STEP` | `24` |
| `MENU_CODE_MAX` | `10` |

Agreement between the Lua and the Rust is asserted by parsing the embedded addon
source, extending the check slice 031 introduced.
