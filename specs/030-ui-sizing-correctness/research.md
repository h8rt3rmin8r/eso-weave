# Research: Application Interface Sizing Correctness

Phase 0 output for slice 030. Records what was established before design, and
what is deliberately left to be confirmed by instrumentation during
implementation.

## R1. Why the window shrink is ratcheted (issue #12)

**Decision**: Treat the window-tracking measurement as the confirmed primary
cause, and the platform gesture latch as an unconfirmed secondary mechanism to be
verified by instrumentation before any fix is written.

**Rationale**: Two facts are established by reading the v0.8.0 sources. First,
`measured = ui.min_rect().size()` at `src/app/ui.rs:435` is taken from the central
panel's `Ui`, and both `egui::MenuBar` and `ui.separator()` at
`src/app/ui.rs:430` allocate the full available width. The measured width is
therefore the window width, so `content_min_size` returns the current window width
as the enforced minimum and `MinInnerSize` at `src/app/ui.rs:467` pins the window
at its own current width. A window that cannot be narrower than itself cannot be
narrowed at all. Second, the stability gate at `src/app/ui.rs:446` reports
unstable whenever the measurement changes between frames, which is every frame of
a drag, and `content_min_size` then falls back to `BOOT_MIN_SIZE`, so a smaller
minimum is sent mid-gesture.

The step from those two facts to "one notch per gesture" requires a third claim:
that a minimum sent during a drag does not take effect until the next drag. On
Windows the sizing modal loop answers `WM_GETMINMAXINFO` when the gesture begins,
which would produce exactly that. This is consistent with the report on both axes
but has not been observed directly, and the height axis is not fully explained by
the width-tracking fact alone.

**Alternatives considered**: Widening the stability gate's epsilon, and debouncing
the minimum-size push until a gesture ends. Both were rejected in build plan 009:
each leaves the enforced minimum derived from the window and changes only how fast
the ratchet advances, so neither can satisfy a single-drag requirement.

**Open item carried into implementation**: The first task instruments a real
resize (measured extent, computed minimum, and the frames on which a minimum is
sent) and confirms or replaces the gesture-latch explanation. Recorded as decision
D8 in `plan.md`.

### R1 confirmed (task T004)

The diagnosis was run headlessly through the new harness rather than by dragging a
real window, because the load-bearing claim is about what the measurement returns,
which the harness observes directly and exactly. Rendering the same application
state at two window sizes gives:

| Window | Measured `content_extent` | Minimum sent |
| --- | --- | --- |
| 700 by 800 | 668 by 768 | 668 by 768 |
| 1600 by 1200 | 1568 by 1168 | 1568 by 1168 |

**Confirmed, and broader than the hypothesis.** The measured extent is the window
size less a constant 32 points on each axis, so it tracks the window on **both**
axes, not on width alone. The original analysis in build plan 009 and issue #12
assumed the height was content-driven and could not fully explain the reported
height ratchet; this measurement explains it. The enforced minimum is always
approximately the current window size, on both axes, so the window can never be
shrunk while the measurement is stable, and the only movement available in any
gesture comes from the stability gate momentarily dropping the minimum to the boot
floor.

**Consequence for the fix**: the `WM_GETMINMAXINFO` gesture-latch mechanism is no
longer needed to explain the defect and is not relied upon. It remains a plausible
account of why a gesture yields one notch rather than none, but the defect's
precondition is the window-derived measurement, and removing that removes the
ratchet regardless of how the platform caches the constraint. The platform
behavior stays unverified and is closed by the operator's desk validation (MV-6),
which is the only place a real drag occurs.

## R2. Why the log pane overlaps the controls (issue #13)

**Decision**: Two causes, both addressed.

**Rationale**: The clamp exists and is correct arithmetic. It is fed the wrong
input: `max_h = (window_h - content_h)` at `src/app/ui.rs:310` uses the same
window-tracking `content_extent` as R1, so the available maximum is computed
against a content height that is not the content height. Separately, the height
egui commits after the drag is read at `src/app/ui.rs:360` and stored into
`self.log_height` and persisted with no re-clamp, so a value produced past the
boundary is both rendered and remembered across restarts.

**Alternatives considered**: Clamping only on read rather than on commit. Rejected
because the persisted value would still be out of range and the first frame after
a restart would render the overlap before any read-side clamp applied.

## R3. Why the settings modal is locked (issue #14)

**Decision**: The modal is never given a height; the fix is to give it one.

**Rationale**: `src/app/ui.rs:789` calls `ui.set_width(modal_w)` and there is no
height counterpart. `egui::Modal` centers its inner `Area`, so the inner layout's
available height runs from the centered area's top edge to the bottom of the
screen, which is approximately half the window. The body scroll area at
`src/app/ui.rs:802` uses `auto_shrink([false, false])`, so it fills that residual
rather than shrinking to content, and it is capped by the residual before
`body_max_h` at `src/app/ui.rs:786` can apply. This matches the report's
"locked to a maximum height that is roughly less than HALF" precisely, and
explains why the growth curve's unit tests pass while the rendered modal does not
grow. `modal_extent(1440.0, 400.0, 880.0, 0.92)` evaluates to 880, far more than
what renders.

**Alternatives considered**: Raising the maximum, or reducing the reserved chrome
height. Both were rejected as treating the symptom: neither makes the rendered
size equal the computed size, so both would leave the modal capped by the
inherited residual at some window sizes.

## R4. Test harness for the untested glue layer

**Decision**: `egui_kittest` 0.35 as a dev-dependency with
`default-features = false`, driven through `Harness::new_ui`.

**Rationale**: The version matches the pinned `egui`/`eframe` 0.35 exactly. The
crate has five feature flags (`document-features`, `eframe`, `snapshot`, `wgpu`,
`x11`) and zero are enabled by default, so with default features off the test
build pulls no GPU stack, no windowing system, and no image codecs. This matters
for CI parity on both a Windows runner and a headless Linux runner.

`Harness::new_ui(app: impl FnMut(&mut Ui) + 'a)` drives a real egui layout with
no `eframe::Frame`. The existing `eframe::App` implementation at
`src/app/ui.rs:282` is `fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame)`
and does not use the frame argument at all, so the entire body can move to an
inherent method taking only `&mut egui::Ui` with no behavior change. That is the
seam Principle III requires and the layer has never had.

The harness also exposes `input_mut() -> &mut RawInput`, which sets `screen_rect`
and therefore simulates a window size, and `hover_at` / `drag_at` / `drop_at`,
which simulate the splitter gesture. Frames are advanced with `step` and
`run_steps`.

**Alternatives considered**: (a) `Harness::new_eframe`, rejected because it
requires the `eframe` feature and a windowing stack for no gain, since the frame
argument is unused. (b) Extracting yet more arithmetic into pure functions and
testing only that, rejected because it is precisely the strategy that has now
produced three green-suite failures: the arithmetic was never the broken part.
(c) Screenshot or snapshot testing via the `snapshot` feature, rejected as
brittle, platform-dependent, and needing a GPU.

## R5. How to express "no per-gesture ratchet" as an assertion

**Decision**: Model a gesture as a sequence of frames at monotonically shrinking
`screen_rect` values, and assert that the enforced minimum never exceeds the
intrinsic extent at any step of the sequence.

**Rationale**: The defect is a relationship between consecutive frames, not a
property of any single frame, which is why every single-frame arithmetic test
passed while the defect shipped. Stepping the harness across a shrinking screen
rect reproduces the frame sequence a real drag produces, without an OS window and
without platform-specific resize mechanics. If the enforced minimum is intrinsic,
it is constant across the whole sequence, and the assertion is trivially satisfied;
if it tracks the window, it rises with the window at each step and the assertion
fails. The assertion therefore fails on the v0.8.0 code and passes on the fixed
code, which is the definition of a useful regression test here.

**Alternatives considered**: Asserting the final size after a simulated drag.
Rejected because a ratchet that advances one notch per gesture still reaches the
right final size after enough gestures, so a final-state assertion cannot
distinguish the defect from the fix.

## R6. Whether the modal maximum is large enough (FR-017)

**Decision**: Unresolved by design, resolved by measurement during
implementation.

**Rationale**: Issue #14 asks both that the modal be given its computed size and
that the maximum "comfortably show far more of the settings body". The first is a
defect with a known fix. The second depends on the settings body's laid-out
height, which is not known without rendering it and is affected by theme and
scale. The plan therefore measures the body height at the modal's inner width and
raises the configured maximum only if half the body is not visible at the current
maximum. Guessing a new maximum now would be an unmeasured change to a constant
the previous slice established.

**Alternatives considered**: Raising the maximum immediately to a round number.
Rejected as unmeasured, and as a change to a prior slice's constant without
evidence.

## R7. Behavior when the intrinsic minimum exceeds the display

**Decision**: Cap the enforced minimum at the display work area; do not add a
scrollable central area in this feature.

**Rationale**: A small display at a high scale factor can in principle produce an
intrinsic extent taller than the work area, and enforcing it would make the window
unresizable or larger than the screen. Capping keeps the window usable. The
obvious alternative, making the central content scroll, was rejected for a
specific reason rather than a general one: a scroll area configured to fill would
reintroduce exactly the window-tracking measurement this feature exists to remove,
which is a real risk of regression for a case that has not been reported. The
limitation is recorded in the spec's edge cases and is the follow-up if the case
is ever reached in practice.
