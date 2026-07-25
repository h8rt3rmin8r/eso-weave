# Build Plan 009: Application Interface Sizing Correctness

Plan: 009
Status: active
Master specification: `docs/ESO-Weave-Specification-v0.2.0.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

The application window has now carried a sizing defect through four
consecutive releases. Issues #4, #5, #6, and #7 were the v0.6.x round;
issue #8 asked for a ground-up review after v0.7.0; slice 029 delivered
that ground-up rebuild in v0.8.0. Three new reports followed the v0.8.0
install, and the first of them is filed explicitly as a zero-tolerance
regression report rather than a request for another point patch. This
plan is the response, and it contains one slice covering all three
reports together, because they share one surface and, on the evidence
below, largely one root cause.

The three reports are the whole of the currently open interface work.
Issue #12 records that shrinking a window persisted at a large size
moves it by roughly one text line per drag gesture, on width as well as
height, so reaching a sane size takes many repeated drags; a single
continuous drag should reach the true content minimum. Issue #13 records
that the live log pane can again be dragged over the interactive Skills
controls, which is the exact behavior reported in issue #5, declared
fixed in slice 027, and rebuilt in slice 029; the log pane covering an
interactive control has always been disallowed and is a hard fail. Issue
#14 records that the settings modal does not grow with the window as
slice 029 specified, and is locked to a height under half of its own
content, so the body scrolls almost immediately while the window has
enormous free space around it. The remaining open issues (#2, #3, #9,
#10, and #11) are all PixelBeacon addon features and are out of this
plan's scope.

The reason this defect class keeps shipping is structural, and this
slice exists to fix the structure and not only the symptoms. All three
defects live in `src/app/ui.rs`, whose module header declares the layer
"excluded from the unit-tested surface and validated with the manual
checklist in the feature quickstart". The pure helpers that layer calls
are correct: `content_min_size`, `measurement_stable`,
`clamp_log_height`, `split_log_height`, and `modal_extent` each carry
tests in `tests/app_window_sizing.rs` and `tests/app_view_model.rs`, and
every one of those tests was green when v0.8.0 shipped these three bugs.
The bugs are entirely in the untested glue between the helpers and egui,
which is a surface the current test strategy cannot reach by
construction. A slice that only corrects the glue would be the fourth
patch to pass its own suite and fail in the field, so this slice's
primary obligation is to make that glue verifiable.

Reading the v0.8.0 sources gives a grounded starting hypothesis for each
report, to be confirmed by instrumentation during implementation rather
than assumed. For issue #12, the enforced minimum is captured as
`ui.min_rect().size()` on the central panel, and both the menu bar and
`ui.separator()` allocate the full available width, so the measured
width is the window width rather than the content width; the enforced
minimum width is therefore pinned to whatever the window already is. The
per-gesture ratchet follows from the stability gate: while a drag is in
progress the measurement changes every frame, the gate opens the minimum
back down to the boot floor mid-gesture, and on Windows the sizing modal
loop answers `WM_GETMINMAXINFO` when the gesture begins, so a minimum
relaxed during a drag only takes effect on the next drag. That is one
notch per gesture on both axes. For issue #13, the log pane's available
maximum is computed against that same contaminated content extent, and
the height egui commits after a drag is stored and persisted with no
re-clamp, so an over-drag is both rendered and remembered. For issue
#14, the modal sets its width but never its height, and `egui::Modal`
centers its inner area, so the inner layout's available height runs from
the centered area's top edge to the bottom of the screen, roughly half
the window; the body scroll area fills that residual and is capped well
below the computed extent before that extent can apply. The single
unifying observation is that the enforced content extent must be
intrinsic, meaning a function of the laid-out controls, the theme, and
the scale, and never a function of the current window size. An intrinsic
extent does not change while the window is being dragged, so it is
stable by construction, the stability gate stops lagging, and there is
nothing left for the ratchet to advance against.

This plan traces to the master specification's graphical user interface
sections (11.1 for the main window and its minimum, 11.2 for the live
log viewer, and 11.3 for the settings modal). The slice is application,
test, and documentation in scope, with no PixelBeacon addon change and
no change to the input, weave, or fishing engines.

## Slices

### Slice 030: Application Interface Sizing Correctness

Scope: close issues #12, #13, and #14 by rebuilding the window content
measurement on an intrinsic basis, hard-enforcing the log pane boundary
and the settings modal extent, and making the previously untested
`src/app/ui.rs` sizing glue verifiable with headless rendered-frame
tests.

The content measurement is rebuilt first, because the other two defects
consume its output. The central panel's `ui.min_rect()` capture is
replaced by an extent derived only from content-sized blocks: the status
grid, the weapon bar row, the skills grid, the button rows, and the
conditional uninstall confirmation row, unioned, plus a fixed allowance
for chrome whose extent is not content-derived. Full-width chrome,
meaning `ui.separator()` and the menu bar, must not contribute width.
The boot floor still applies for the frames before any block has been
laid out, so nothing is clipped at startup, and the extent may still
shrink when a control row disappears, preserving slice 029's
no-permanent-latch behavior. The existing pure helpers
`content_min_size` and `measurement_stable` are kept, and their contract
is unchanged; what changes is the value fed into them. Alongside this,
the minimum-size push gains discipline: `MinInnerSize` is sent only when
the intrinsic extent actually changes, which is when a row appears or
disappears or the theme or scale changes, and never in response to
window geometry or during a resize gesture.

The log pane boundary is then made unconditional. Its maximum continues
to be the window height less the content height, now computed against
the corrected extent, and the height egui commits after a drag is
re-clamped before it is stored in state and before it is persisted, with
the corrected value forced on the following frame. The invariant the
slice enforces and tests is stated plainly: on every frame, the log
pane's top edge is at or below the central content's bottom edge, under
drag, under window resize, and under the two combined.
`clamp_log_height`, `log_min_height`, `open_log_reserve`, and
`split_log_height` are unchanged.

The settings modal is given an explicit extent on both axes. The modal's
area is constrained to a centered rectangle of `modal_extent` width by
`modal_extent` height so the rendered rectangle matches the computed
extent, instead of inheriting whatever residual space a centered area
happens to leave. The body scroll area keeps its reserved room for the
heading, separator, and close row, so at a large window the body shows
substantially more than half the settings content and scrolls only for
the remainder, while at a small window the modal still fits within its
configured fraction of the window. The `modal_extent` growth curve
itself is correct, already tested, and unchanged.

The verifiability work is the slice's anti-regression measure and is not
optional. `egui_kittest` is added as a dev-dependency at the version
matching the pinned egui, and a headless suite (for example
`tests/app_ui_sizing.rs`) renders real frames and asserts rendered
geometry rather than helper arithmetic: the pushed `MinInnerSize`
against a known intrinsic content extent, a simulated multi-step shrink
drag that reaches the content minimum in one gesture per axis with no
per-gesture ratchet, a log splitter drag past the boundary asserting the
committed height never exceeds the window less the content in any frame
including the frame of the gesture, and the rendered modal rectangle
against `modal_extent` at a small, a medium, and a very large window.
Because this suite makes the layer testable, the claim in the
`src/app/ui.rs` module header that the layer is excluded from the
unit-tested surface stops being true and is rewritten, and the new
dev-dependency is recorded as a dated decision in `CHANGELOG.md`
following the `ureq` precedent from slice 018. Constitution principle
III (test-first) applies normally: each rendered-frame assertion is
written red before the fix that satisfies it.

Two alternatives were considered and rejected. Widening the stability
gate's epsilon, and debouncing the minimum-size push until a gesture
ends, both leave the enforced minimum derived from the window and only
change how quickly the ratchet advances, so neither can satisfy the
single-drag requirement. Abandoning the enforced minimum entirely was
also rejected: it would reopen the original clipping defect that issue
#4 reported.

The known residual risk is recorded: the Windows `WM_GETMINMAXINFO`
gesture-latch explanation for the ratchet is inferred from the symptom
and the winit path, not yet observed directly, so the implementation
begins by instrumenting a real resize (measured extent, computed
minimum, and the frames on which a minimum is sent) and confirming or
replacing that explanation before any fix is written. If the intrinsic
measurement alone does not remove the ratchet, the instrumentation is
what identifies the remainder.

The master specification is corrected where it has drifted: section 11.1
states a fixed minimum of 480 by 420, which has been the boot floor
rather than the enforced minimum since slice 029, so 11.1 gains a short
statement of the sizing model (a boot floor until the content is
measured, then an intrinsic content-derived minimum), 11.2 gains the log
pane's never-overlap boundary, and 11.3 gains the modal's growth
behavior. `CHANGELOG.md` records a `Fixed` entry naming all three issues
plus the dated dependency decision.

Validation is deliberately stricter than a green suite, because a green
suite is what shipped these three defects. The feature quickstart
requires the exact reproduction from each report, run at the desk with
no game: persist an oversized window on both axes, restart, and shrink
to the content minimum in one drag per axis; open the live log and drag
the splitter upward hard, at the minimum window size and immediately
after both enlarging and shrinking the window, confirming the pane stops
dead before the Skills rows every time; and open Settings at a large
window and confirm the modal is near its maximum extent, grows
continuously with diminishing returns as the window grows, and shows far
more than half its content. The quickstart also absorbs the still-owed
manual validation from slice 029 (MV-1 through MV-5 in
`specs/029-window-sizing-rebuild/quickstart.md`), since those items cover
the code this slice replaces and one desk session should close both.
Feature under `specs/030-<name>/`.
