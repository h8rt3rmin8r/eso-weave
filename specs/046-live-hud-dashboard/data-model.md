# Data Model: Responsive Live HUD Dashboard

## DashboardLayout

| Variant | Meaning | Ordering |
| --- | --- | --- |
| `Narrow` | Available width is below 880 points | Live HUD, then System and automation |
| `Wide` | Available width is at least 880 points | Live HUD left, System and automation right |

The projection is pure and depends only on available egui point width.

## ResourcePresentation

| Variant | Numeric value | Visible state | Role |
| --- | --- | --- | --- |
| `Observed(percent)` | 0 through 100 | exact `N%` | resource theme |
| `Low(percent)` | 0 through configured threshold | `Low: N%` | resource theme plus warning boundary |
| `Dormant` | none | `Game not active` | muted |
| `Unavailable` | none | `Signal unavailable` | warning/unavailable |

### Invariants

- Observed zero is numeric and is never Dormant or Unavailable.
- Low is impossible when the corresponding watch is disabled.
- Dormant and Unavailable expose no progress value.
- The view model never clamps a corrupt input; decoder validation remains the
  only entry into `ResourceLevel::Percent`.

## ResourceView

| Field | Type | Purpose |
| --- | --- | --- |
| `presentation` | `ResourcePresentation` | Exhaustive state semantics |
| `text` | `String` | Stable visible and accessible copy |
| `role` | `StatusRole` | Shared status boundary color |

Derived helpers expose `percent() -> Option<u8>` and `fraction() -> Option<f32>`.

## ResourceTheme

| Variant | Semantic association |
| --- | --- |
| `Health` | red family |
| `Stamina` | green family |
| `Magicka` | blue family |

Theme affects fill only. Name, percentage, fill length, and state text remain
independent cues.

## BeaconSignalLine

| Runtime | Freshness | Text | Role |
| --- | --- | --- | --- |
| not Active | any reset state | Game not active | Muted |
| Active | Fresh | Signal detected | Healthy |
| Active | NeverObserved | Not detected | Warning |
| Active | Lost | Signal lost | Error |
| Unknown | any | Unknown | Warning |

Installation is not an input to this projection.

## DashboardGeometryReceipt

Rendered-frame tests record:

- selected `DashboardLayout`
- Live HUD rectangle
- System and automation rectangle
- last resource-meter rectangles
- existing content bottom and log top

The receipt is test instrumentation only and never enters persisted state.
