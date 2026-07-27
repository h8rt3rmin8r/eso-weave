# Research: Out-Of-Band Display Detection

**Feature**: `specs/034-display-detection/` | **Date**: 2026-07-27

This resolves the technical unknowns the plan depends on. Three of them are
"does this operating system call exist in the bindings we already have", one is
"what does the settings file actually look like", and one is the question the
source issue left open, which turns out to be answerable without asking anyone
to do anything.

## R1: Windows monitor and DPI queries, without a new dependency

**Question**: Can the monitor the game window sits on, its rectangle, and its
scale be resolved with the `windows-sys` features the crate already enables?

**Finding**: Yes, all of it. `Cargo.toml` already enables
`Win32_Graphics_Gdi` (for the existing capture path) and `Win32_UI_HiDpi` (for
the existing window sizing work). Those two cover everything needed:

| Call | Module | Already enabled |
| --- | --- | --- |
| `MonitorFromWindow` | `Win32::Graphics::Gdi` | yes |
| `GetMonitorInfoW`, `MONITORINFO` | `Win32::Graphics::Gdi` | yes |
| `GetDpiForMonitor`, `MDT_EFFECTIVE_DPI` | `Win32::UI::HiDpi` | yes |
| `ClientToScreen`, `GetClientRect` | already used by the sampler | yes |

**Decision**: Use `GetDpiForMonitor` with `MDT_EFFECTIVE_DPI` rather than
`GetDpiForWindow`.

**Rationale**: `GetDpiForWindow` answers "what DPI should this window be drawn
at given its own DPI-awareness context", which for a window belonging to a
process that declared no awareness is 96 regardless of the physical display. We
are querying a window belonging to another process (the game), whose awareness
we neither control nor know. `GetDpiForMonitor` asks about the monitor, which is
the thing we actually want to know about, and gives the same answer no matter
who is asking.

**Alternatives considered**: `GetDpiForWindow` (rejected above);
`EnumDisplayMonitors` plus manual rectangle intersection (more code, same answer
as `MonitorFromWindow`); `GetDeviceCaps(LOGPIXELSX)` on the screen device
context (system-wide, wrong on mixed-DPI setups).

**Consequence**: No `Cargo.toml` change on Windows, so no pinned-artifact
decision is needed for this feature.

## R2: Linux, and what X core can and cannot tell us

**Question**: What can the X11 path report, and does it need a new x11rb
feature?

**Finding**: The core X protocol supplies the window's geometry
(`get_geometry`), its absolute origin (`translate_coordinates` against the
root), and the screen's pixel dimensions (from the connection setup, which the
existing sampler already reads for the root window). It does not have a concept
of a per-monitor scale factor at all. Per-monitor rectangles need the RandR
extension, which is an optional `x11rb` feature that this crate does not enable
directly. It may be enabled transitively by `winit` through `eframe`, since
`winit` uses RandR for monitor enumeration, but Cargo feature unification is not
something to build a compile on when it has not been observed.

**Decision**: Use only core `xproto`, which the existing sampler already uses.
Report the surface size, the surface origin, and the X screen's dimensions as
the display size. Report the scale as unknown.

**Rationale**: Three reasons, in order of weight. First, the wrap layout that
this feature exists to unblock is bounded by the surface size, and the surface
size is exactly what core X reports precisely. Second, this repository has no
continuous-integration test job; the only place Linux is compiled is the release
pipeline, so a Linux-only compile error introduced here would not surface until
a release is cut. That argues strongly for using calls whose availability is
already demonstrated in the file being edited. Third, reporting an unknown scale
is what the specification requires (FR-006) rather than a shortfall against it,
and the honest unknown is more useful than a fabricated 1.0.

**Alternatives considered**: Enabling `x11rb/randr` explicitly and reporting
per-monitor rectangles. This is the better long-term answer and should be a
follow-up once someone can compile and run it on a multi-head Linux box. It is
rejected here because it adds an unverifiable dependency change to a slice that
does not need it. Deriving a scale from the screen's millimetre dimensions was
also considered and rejected: those values are famously fabricated by drivers,
and a plausible wrong scale is worse than no scale.

**Known limitation to record**: on a multi-head X session the reported display
size is the union of all heads, not the head the window is on. Stated in the
plan, the data model, and the module documentation, so nobody mistakes it for
per-monitor geometry.

## R3: The settings file

**Question**: What exactly is being parsed, and where is it?

**Finding**: The file is `UserSettings.txt` in the game's per-environment data
directory, which is the parent of the `AddOns` directory the beacon installer
already resolves. Every line is `SET <Key> "<value>"`, with the value quoted
even when it is numeric. The keys of interest, their example values, and the
version-suffix behaviour are recorded in issue #3 against a live install from
July 2026.

**Decision on the path**: Derive it as the parent of the resolved AddOns
directory, joined with the file name.

**Rationale**: This satisfies FR-012 in the strongest available sense. It does
not re-derive the documents directory, does not restate the
`Elder Scrolls Online/<environment>` segment, and inherits every property the
existing resolution already has, including the manual path override and the
Linux Proton prefix handling. A second path-building function would have been
two statements of the same fact, which is the failure the cross-language marker
registry exists to prevent one directory over.

**Decision on key matching**: Strip an optional trailing `.` followed by digits,
then compare the remaining base key case-insensitively.

**Rationale**: The suffix is the documented failure mode (the game bumps it when
a setting's meaning changes, so a literal match silently stops matching after a
patch). Case-insensitivity is free and guards against a rename that changes only
capitalisation. Note that base keys are compared whole, not by prefix, so
`FULLSCREEN` and `FullscreenWidth` remain distinct.

**Decision on duplicates**: The last assignment in the file wins, matching how
a sequentially written settings file would be read back by anything else.

## R4: The window-mode enum, and why this feature does not need it

**Question**: Issue #3 records that the `FULLSCREEN` integer's mapping to
fullscreen, windowed, and borderless is unconfirmed, and asks for it to be
verified before shipping. What is the actual dependency?

**Finding**: The mapping is needed for exactly one thing: choosing which of the
two stored resolution pairs is the live one. When the game is running, the
operating system reports the live surface directly, so the question does not
arise. When the game is not running, the question arises but has no consequence,
because nothing is currently laid out pre-launch.

**Decision**: Do not map the value. Report it raw. Produce a configured
descriptor only when both stored pairs are identical, in which case the mapping
is irrelevant.

**Rationale**: Guessing produces a confident wrong answer on precisely the
installs where it matters, which are the ones whose two pairs differ. On the one
install measured, they differ by a lot (3440x1440 against 5160x2160), so a wrong
guess would not be a near miss. Blocking the slice on verification trades a
working feature for a task that has to be remembered and performed.

**The useful part**: when both a measurement and a stored reading are available,
the measured surface size normally matches exactly one stored pair, and that
match is evidence about what the mode value meant on that install. Recording it
as a diagnostic accumulates the evidence issue #3 asked for from ordinary use,
without anyone running a procedure. The feature does not act on the inference;
it writes it down. If enough of these accumulate consistently, a later slice can
adopt the mapping on evidence rather than on assumption.

## R5: Where re-resolution rides, and on which seam

**Question**: The descriptor must stay current without a new thread or timer
(FR-008). Where does it go, and how is it tested without hardware (FR-026)?

**Finding**: The pixel bus worker thread in `src/main.rs` already loops at a
sampled cadence, already re-resolves a missing sampler, and already calls into
the sampler once per iteration. The Windows sampler's `capture` already performs
`ClientToScreen` and `GetClientRect` on every iteration, which is most of a
measurement.

**Decision**: Add a defaulted `display()` method to the existing
`SurfaceSampler` trait rather than introducing a second trait and a second trait
object.

**Rationale**: `SurfaceSampler` is already the seam between the decoders and the
operating system, it is already implemented by both platform backends and by the
mock, and `resolve_sampler` already produces exactly one boxed instance of it.
Adding a parallel `DisplayProbe` trait would mean a second resolution path, a
second boxed object, and a second mock, to express the same "this is where the
operating system is" boundary. The default implementation returns no
measurement, so the mock and any future backend opt in rather than being broken
by the addition.

**Decision**: The change detection, reconciliation, and descriptor construction
live in a pure type that performs no input or output, and the settings read is
supplied to it as a closure it calls only when it decides a read is warranted.

**Rationale**: This is what makes FR-016 ("read only when the measurement
changed") testable rather than aspirational: a test counts how many times the
closure was invoked across a scripted sequence of measurements. It also keeps
every decision in the feature reachable from a unit test with no window, no
file, and no display.
