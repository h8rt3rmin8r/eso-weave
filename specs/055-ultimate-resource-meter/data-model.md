# Data Model: Ultimate Resource Meter

## UltimateValue

```text
UltimateValue = Unknown | Points(u16)
```

- `Points(0)` is a valid observed charge.
- `Points(511)` is impossible because 511 is the wire unavailable sentinel.
- Values above 510 publish as unavailable rather than clamping.
- A zero maximum or zero cost normalizes to `Unknown` after transport decoding.

## UltimateTelemetry

```text
UltimateTelemetry
|-- current: UltimateValue
|-- maximum: UltimateValue
|-- front_cost: UltimateValue
`-- back_cost: UltimateValue
```

The event is atomic. Each field decodes independently from its own byte pair.
Presentation is available only when current and a positive maximum are known.
Costs remain independent so a missing value on one bar does not erase the other.

## UltimateView

```text
UltimateView
|-- presentation: Available | Unavailable | Dormant
|-- current: optional u16
|-- maximum: optional u16
|-- fill_fraction: optional f32
|-- active_bar: Front | Back | Unknown
|-- active_cost: optional u16
|-- threshold_fraction: optional f32
|-- ready: optional bool
|-- readout: string
`-- accessibility_description: string
```

### Projection rules

1. Dormant game state overrides presentation but does not mutate cached telemetry.
2. Unknown current, unknown maximum, or zero maximum produces Unavailable.
3. Fill is `current / maximum`, clamped to 0 through 1 for painting.
4. Front selects front cost and Back selects back cost. Unknown selects no cost.
5. Threshold is `cost / maximum`, clamped only for painting.
6. Ready is `current >= cost` only when current, maximum, selected cost, and bar
   are valid. Otherwise it is unknown and no Ready text appears.
7. Exact raw cost remains in accessibility text even if it exceeds maximum.

## MeterGeometry

```text
MeterGeometry
|-- row_rect
|-- label_rect
|-- track_rect
|-- quarter_segments[3]
|-- threshold_segment: optional
|-- numeric_rect
`-- ready_rect
```

Geometry depends on the allocated row and the presence of an Ultimate-style
trailing layout, never on Ready. The threshold terminates at the row bottom and
begins inside the track, using the three already reserved points below the track.

## ProtocolV5Layout

| Block | Meaning |
| --- | --- |
| B25 | Exact 9-bit Ultimate current |
| B26 | Exact 9-bit Ultimate maximum |
| B27 | Exact 9-bit front Ultimate cost |
| B28 | Exact 9-bit back Ultimate cost |

Every field reserves two unique green markers for high bit 0 or 1. Red is the low
byte and blue is its complement checksum. High bit 1 plus red 255 is unavailable.
