# Build Plan 006: Window Persistence, UI Defect Cleanup, and Fishing Diagnosis

Plan: 006
Status: active
Master specification: `docs/ESO-Weave-Specification-v0.2.0.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

Build plans 001 through 005 delivered the functional product, brand and UX
polish, a GUI overhaul, weapon-bar-aware timing, fishing reliability work, API
version automation, and the v0.2.0 specification rewrite (through v0.5.0). This
plan clears a batch of reported user-interface defects and makes a fourth,
evidence-driven attempt at the persistent fishing failure.

It traces to the master specification's graphical user interface (section 10),
the configuration and state persistence model (section 11), the fishing module
(section 8 and 9), and the pixel-bus and PixelBeacon signal contract (section
10.3). Slice 021 records window geometry across sessions, slice 022 corrects the
primary and skills controls, and slice 023 corrects the settings modal, the
success toast, the logging linkage, and the keybinding presentation; these three
are GUI and configuration slices with no game interaction. Slice 024 is the only
safety-surface-adjacent slice: it hardens the pixel-bus screen read and must
preserve the guarantee that fishing degrades to disabled on signal loss.

Two decisions are flagged for the pre-push halt. Slices 021 and 023 add
persisted fields to the on-disk configuration, which advances `schema_version`
with a forward migration and a `.invalid` fallback, consistent with the
constitution's configuration principle. Slice 024 changes the screen-capture
mechanism that the master specification names in section 10.3, so it updates the
specification's capture language and records a dated decision in `CHANGELOG.md`;
it stays within the bounded scope of the constitution (no game-memory access, no
packet interception, nothing inside the game beyond the screen-signal contract).

## Slices

### Slice 021: Window Geometry Persistence

Scope: record and restore the application window across sessions so it reopens at
the size, position, maximized state, and monitor it last occupied, including on
multi-monitor systems. The window currently opens at a fixed size every launch
because the eframe native options carry no restored geometry and the eframe
persistence backend is not enabled. Persist geometry through the project's own
configuration path rather than eframe storage, adding a window sub-section to the
persisted user state with inner size, outer position, a maximized flag, and
enough monitor identity to reselect the same screen; write it on change and on
close through the existing save-scheduler coalescing path. Restore the geometry
into the viewport at startup behind a bounds sanity check, so a now-disconnected
or reconfigured monitor falls back to a visible position on the primary display
rather than opening off-screen. The new fields advance the configuration
`schema_version` with a forward migration and a `.invalid` fallback; the geometry
math and the fallback selection land in tested pure helpers. Feature under
`specs/021-<name>/`.

### Slice 022: Primary and Skills Panel Controls

Scope: correct the primary status section and the skills grid. Add an addon
Update control beside Install and Uninstall that is disabled while the addon is
not installed locally and enabled whenever it is installed, even when it is
current; activating it performs an uninstall followed by an install in sequence
through the existing beacon intents. Bring the Weapon Bar line into the primary
grid so its label aligns with the Status, Fishing, and Pixel Beacon rows instead
of sitting outside the grid. Give the skills Weave dropdown a single fixed width
sized to the longest option with a small margin, so the resting field no longer
changes width with the selection. Rename the skills delay column to Delay in
milliseconds, confirm the displayed values remain the actual effective delays
(the per-slot override when set, otherwise the weave-type default), and unify the
delay cell so the dormant value when Override is off renders inside a visibly
greyed read-only box matching the active field, both boxes widened to
comfortably hold at least four digits with the numeric content right-aligned.
Introduce a shared combo-box helper that fixes width and stabilizes layout so a
dropdown never reflows the rows below it on hover or selection, and apply it to
the primary and skills dropdowns; the earlier hover-reflow fix covered buttons
and toggles but not combo-box content-width variation. The presentation changes
keep the egui layer thin and are verified observationally against the running
app. Feature under `specs/022-<name>/`.

### Slice 023: Settings Modal, Success Toast, Logging Linkage, and Key Presentation

Scope: correct the settings modal and its surrounding controls. Rework the modal
sizing so both width and height track the current window every frame, resolving
the case where a modal opened after a resize conforms to the default window size;
size it on an explicit curve where the occupied percentage of the window
decreases as the window grows while the absolute pixel size still increases up to
a sensible upper cap, so it stays proportionate on a large or ultrawide display,
and apply the same rule to height that already governs width. Give the settings
saved toast a green success accent in place of the neutral panel fill so it draws
the eye, kept theme-aware across light and dark. Link the live log verbosity
control and the settings log level so a change to either updates the other, while
preserving the required asymmetry that hiding the live log panel does not change
the logging verbosity. Present keybindings with friendly labels (Number 1 through
Number 5, E, R, X, Q, Space, F1, F2) in both the selected text and the option
list, leaving the stored and parsed key forms unchanged. Add the F2 key to the
selectable keys list, which currently omits it even though the key type defines
F2 and round-trips through its string form, so the toggle-fishing binding can be
rebound and F2 appears in the dropdown. Apply the shared combo-box helper from
slice 022 to the settings, theme, log, beacon, and keybinding dropdowns. The
correctness surface is visual and the mapping helpers are tested; verification is
observational against the running app. Feature under `specs/023-<name>/`.

### Slice 024: Fishing Signal Diagnosis and Capture Hardening

Scope: make an evidence-driven fourth attempt at the fishing failure where the
status shows Casting and reverts to Idle with no cast detected after the arm
timeout, never advancing to Fishing. The missing transition is the controller
move from Armed to Waiting, which fires only when the reader decodes the blue
waiting block on the fishing sample point while the heartbeat block is present;
the addon render and the reader decode agree on colors and coordinates, so the
failure is upstream of decoding, and the reported wording of no cast detected
rather than signal lost points at the heartbeat never being read. The prime
structural suspect is that the current single-pixel screen read does not observe
the accelerated game surface. Harden the capture by reading the four sample
points from a captured client-area bitmap using a method that works on an
accelerated window surface, kept behind the existing sampler seam so the pure
decode and observe logic and their tests are untouched, and preferring an
in-process capture over shelling out on every poll. Extend the existing
per-sample trace logging so a live session shows each block's raw color bytes,
the decoded state, and the heartbeat age unambiguously, which distinguishes a
never-present heartbeat (a capture problem) from a present heartbeat with a
never-blue fishing block (an addon interaction-detection problem) in a single
reading; this remains log-only with no new interface surface. Update the master
specification's capture-mechanism language in section 10.3 and record a dated
decision in `CHANGELOG.md`. Fishing must still degrade to disabled on signal
loss, and the beacon managed-marker uninstall guarantee is not weakened. The
feature `plan.md` defines the in-game validation run: confirm bait is selected,
the addon is current and loaded, the beacon strip is visible and unobstructed,
and the window is focused, then verify through the new trace that the heartbeat
reads present and, on a real cast, the fishing block turns blue and the interface
advances from Casting to Fishing to Reeling; if the heartbeat is present but the
fishing block never turns blue with all preconditions met, the evidence scopes a
follow-up slice at the addon interaction-detection contract rather than another
guess. Feature under `specs/024-<name>/`.
