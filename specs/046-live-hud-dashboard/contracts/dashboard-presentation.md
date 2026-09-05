# Contract: Dashboard Presentation

## C1. Section ownership

Every pre-Skills field has one owner:

| Field | Owner |
| --- | --- |
| Game Context | Live HUD |
| Health, Stamina, Magicka | Live HUD |
| Combat, Movement, Weapon Bar | Live HUD |
| Quickslot classification, availability, cooldown | Live HUD |
| Game installation and state | System and automation |
| ESO Weave active/suspended | System and automation |
| PixelBeacon installation and signal | System and automation |
| Fishing requested/effective | System and automation |
| Auto-potion requested/effective | System and automation |
| Install, Update, Uninstall | System and automation |

## C2. Responsive geometry

`dashboard_layout(width)` returns Wide exactly when `width >= 880.0` points.
Narrow rectangles satisfy `live.bottom <= system.top`. Wide rectangles satisfy
`live.right <= system.left` and have aligned top edges. Live HUD is created first
in both modes so accessibility and visual reading order agree.

## C3. Resource geometry

All three meters in one frame have equal height and available track width. An
observed `p` produces `p / 100` fill. There is no easing or time input. Text uses
a reserved trailing value area so digit changes do not resize the row.

## C4. Resource semantics

- `0%` has a numeric accessibility value of zero.
- `Game not active` and `Signal unavailable` have no numeric value.
- Low includes the word `Low` in visible and accessible text.
- Resource name and state are present in one progress-indicator description.

## C5. Contrast and redundant cues

Normal text reaches 4.5:1 against its surface. Meter tracks, fills, and meaningful
state boundaries reach 3:1 against adjacent surfaces. Resource identity appears
as text and not only hue. Status meaning appears as text and not only a dot color.

## C6. Stable downstream behavior

The Skills grid source order, controls, intent generation, and cooldown values
are unchanged. Log, modal, persistence, input, beacon manager, and pixel protocol
contracts continue to pass their existing tests.

## C7. Addon actions

Exactly one primary action exists for NotInstalled/AddonsNotFound (Install) and
InstalledOutdated (Update). InstalledCurrent has no primary setup action.
Uninstall appears only when the model's existing `uninstall_enabled` guard is
true and still requires confirmation before raising `UninstallBeacon`.
