# Data Model: Dashboard Layout Cohesion

## EffectiveDashboardLayout

| System and State | Width | Result |
| --- | --- | --- |
| Collapsed | any finite width | Stacked |
| Expanded | below 880 points | Stacked |
| Expanded | at least 880 points | Paired |

Invalid or non-finite width remains conservatively stacked.

## ExpandedCardGeometry

| Field | Meaning |
| --- | --- |
| `card_width` | Full available width when stacked, or half usable width when paired |
| `body_height` | Live HUD rendered body height used as the expanded shared minimum |
| `gap` | Existing inter-card gap applied only between cards |

The two expanded outer rectangles differ by at most one rendered point in width
and height. Collapsed System and State uses its natural disclosure-header height.

## DashboardRowGeometry

| Region | Sizing rule |
| --- | --- |
| Label | Stable shared width for dashboard labels |
| Value | Remaining width after insets, gaps, and optional interaction region |
| Interaction | Fixed width for two compact lifecycle buttons plus one gap |

The value region is always allocated even when its text is short, which makes
its right boundary independently testable.

## InteractionGroup

| State | Controls |
| --- | --- |
| Toggle row | One named switch at the shared origin |
| Setup needed | Install, optionally followed by Uninstall |
| Update needed | Update followed by Uninstall |
| Current | Uninstall only when managed removal is available |

All lifecycle buttons use the same width and height.

## FieldLabelRegistry

The registry contains only visible field and settings labels governed by title
case. It excludes dynamic states, descriptions, tooltips, toasts, menu actions,
button captions, and other prose.

Required exact migrations:

| Previous | Replacement |
| --- | --- |
| Active weapon bar | Weapon Bar |
| Life state | Life State |
| Roll dodge | Roll Dodge |
| World state | World State |
| Auto-potion | Auto Potion |
| PixelBeacon installation | PixelBeacon Status |
| PixelBeacon signal | PixelBeacon Signal |

## ResourceGroup

A rendering-only ordered collection of resource descriptors. Production remains
three items in S054; tests may supply four to prove the group boundary is not
tied to Magicka. No domain or telemetry field is added.
