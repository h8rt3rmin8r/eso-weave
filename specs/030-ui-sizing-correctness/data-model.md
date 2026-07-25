# Data Model: Application Interface Sizing Correctness

Phase 1 output for slice 030. This feature adds no persisted data and changes no
stored schema. The entities below are the in-memory sizing quantities that the
frame computes and the tests assert on.

## Entities

### IntrinsicExtent

The width and height the laid-out controls require, independent of the window
they are shown in. Replaces the window-tracking `content_extent` value as the
source of the enforced minimum.

| Field | Type | Meaning |
| --- | --- | --- |
| `width` | points (f32) | Widest content-sized block laid out this frame. |
| `height` | points (f32) | Total height of the laid-out central content, including chrome and trailing padding. |

Accumulation rules:

- A block that sizes to its content contributes both its width and its height.
- A block that expands to fill the available width contributes its height only
  (FR-007). This is what makes the value window-independent.
- Accumulation resets at the start of each frame and completes when the central
  content has been laid out.

Validation:

- Both fields are strictly positive once the first frame has been laid out.
- Neither field may vary when only the window size varies. This is the property
  the rendered-frame tests assert, and the one the v0.8.0 value violates.

### EnforcedMinimum

The value pushed to the platform as the window's minimum inner size.

| Field | Type | Meaning |
| --- | --- | --- |
| `width` | points (f32) | `IntrinsicExtent.width`, plus the log width bonus while the log is open. |
| `height` | points (f32) | `IntrinsicExtent.height`, plus the open log reserve while the log is open. |

State transitions:

- Before the content has been measured and the measurement is stable, the
  enforced minimum is the boot minimum (480 by 420), unchanged from the previous
  slice (FR-002).
- Once stable, it is derived from `IntrinsicExtent` and may decrease (FR-004).
- It is capped at the display work area (FR-008).
- It is pushed to the platform only when its value changes (FR-005), never in
  response to a window geometry change.

### LogPaneBoundary

The lowest position the log pane's top edge may take, for a given window.

| Field | Type | Meaning |
| --- | --- | --- |
| `max_height` | points (f32) | `window_height - IntrinsicExtent.height`, floored at the six-line minimum. |
| `min_height` | points (f32) | Six lines of log text plus the frame margins. Unchanged. |

Validation:

- On every frame, the pane's rendered top edge is at or below the central
  content's rendered bottom edge (FR-010). This is the invariant the tests assert
  per frame, including the frame of the gesture itself.
- A height committed by a drag is clamped into `[min_height, max_height]` before
  it is stored and before it is persisted (FR-011).
- A restored height is clamped before the first frame is shown (FR-012).

### ModalTargetExtent

The size the settings modal's growth rule calls for at the current window size.

| Field | Type | Meaning |
| --- | --- | --- |
| `width` | points (f32) | `modal_extent(window_width, 460, 1040, 0.92)`. Unchanged rule. |
| `height` | points (f32) | `modal_extent(window_height, 400, 880, 0.92)`. Unchanged rule, except as FR-017 permits. |
| `body_max_height` | points (f32) | `height` less the reserved heading, separator, and close row. |

Validation:

- The modal's rendered rectangle equals this extent on both axes within a
  hairline (FR-014). The v0.8.0 defect is exactly the failure of this identity.
- The extent is non-decreasing in the window size and bounded by the configured
  maximum (FR-015).
- The extent never exceeds the window (FR-016).

### SettingsBodyHeight

The total laid-out height of the settings content at the modal's inner width,
measured on the same frame. Used only to evaluate FR-017 and SC-004.

Validation:

- At the modal's maximum size, `body_max_height / SettingsBodyHeight >= 0.5`. If
  the measurement shows otherwise, the configured maximum height is raised until
  it holds, and the new value is recorded as a decision.

## Relationships

```text
IntrinsicExtent ──> EnforcedMinimum ──> platform minimum inner size
       │
       └─────────> LogPaneBoundary ──> committed and persisted pane height

window size ──> ModalTargetExtent ──> rendered modal rectangle
                        │
                        └──> body_max_height ──> compared against SettingsBodyHeight
```

The single edge that must not exist, and whose presence is the root defect of
issue #12, is a path from the window size back into `IntrinsicExtent`.

## Persisted state

Unchanged. `log_panel_height` in the `ui` settings section and the window
geometry in session state keep their current shapes and meanings. The only
behavior change is that a value read back from either is clamped into its valid
range before the first frame is rendered.
