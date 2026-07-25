# Contract: Rendered Sizing Behavior

Slice 029 contracted the pure sizing helpers in `src/app/mod.rs`
(`specs/029-window-sizing-rebuild/contracts/sizing.md`). Those contracts are
unchanged and still hold. This document contracts the layer above them, which has
never had one: what the rendered frame must do with those helpers' output.

The distinction is the point of this slice. Every clause below is false on the
v0.8.0 code while every clause of the slice 029 contract is true.

## C1. The intrinsic extent is window-independent

For any two window sizes `w1` and `w2`, rendering the same application state at
`w1` and at `w2` MUST produce the same intrinsic extent.

- Blocks that size to their content contribute width and height.
- Blocks that expand to fill the available width contribute height only.
- The extent is recomputed each frame and is never latched.

Violation on v0.8.0: the extent is `ui.min_rect()` of the central panel, whose
width equals the available width, so the extent is a function of the window.

## C2. The enforced minimum is the intrinsic extent

The value pushed to the platform as the minimum inner size MUST equal:

| Log state | Width | Height |
| --- | --- | --- |
| Closed | `intrinsic.width` | `intrinsic.height` |
| Open | `intrinsic.width + LOG_WIDTH_BONUS` | `intrinsic.height + open_log_reserve(row)` |

before the boot-minimum substitution (which applies only while the measurement is
not yet stable) and before the work-area cap.

It MUST be pushed only when this value changes. It MUST NOT be pushed in response
to a change in window size.

## C3. A resize gesture is never clamped by a stale minimum

For a monotonically shrinking sequence of window sizes
`w0 > w1 > ... > wn` rendered as consecutive frames, the enforced minimum MUST
satisfy, at every step `i`:

```text
enforced_minimum(i) <= intrinsic_extent   (per axis)
```

That is, the enforced minimum MUST NOT rise with the window at any step, and MUST
NOT differ between step `i` and step `i+1` unless the intrinsic extent itself
changed.

Violation on v0.8.0: the enforced minimum equals the window width at each stable
step, so it decreases only as fast as the window does, one step at a time.

## C4. The log pane never covers content

On every rendered frame, with the log open:

```text
log_pane.rect.top >= central_content.rect.bottom
```

This MUST hold:

- while the splitter is being dragged, including the frame of the gesture,
- while the window is being resized,
- on both at once,
- on the first frame after a restore from persisted state.

Additionally, the height stored in application state and the height persisted to
the settings file MUST both already satisfy the boundary; the clamp is applied
before the store, not only before the render.

## C5. The modal renders at its computed extent

For any window size, the settings modal's rendered rectangle MUST satisfy:

```text
|rendered.width  - modal_extent(window.width,  460, 1040, 0.92)| <= 1.0
|rendered.height - modal_extent(window.height, 400,  880, 0.92)| <= 1.0
```

Worked values (from the unchanged growth rule, `GROWTH = 0.55`):

| Window height | Computed modal height |
| --- | --- |
| 420 | 386.4 (window fraction binds: 420 * 0.92) |
| 720 | 576.0 |
| 1200 | 840.0 |
| 1440 | 880.0 (maximum binds) |
| 2160 | 880.0 (maximum binds) |

Violation on v0.8.0: the rendered height is bounded by the space the centered
area leaves below its own top edge, roughly half the window, and never reaches
these values.

## C6. The modal shows at least half its body

At the modal's maximum size:

```text
body_max_height / settings_body_height >= 0.5
```

where `settings_body_height` is the settings content's laid-out height at the
modal's inner width on the same frame. If the measurement fails this, the
configured maximum height is raised until it holds and the new value is recorded
as a dated decision.

## C7. The frame body is drivable without a window

The application's per-frame rendering MUST be reachable through a function whose
only argument is a mutable egui `Ui`. This is what makes C1 through C6 assertable
without an operating system window, a GPU, or a display.

This is a contract on the code's shape, not on its behavior, and it exists because
three prior slices could not assert any of the clauses above.
