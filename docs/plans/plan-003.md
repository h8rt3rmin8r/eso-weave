# Build Plan 003: GUI Overhaul and Weapon-Bar-Aware Timing

Plan: 003
Status: active
Master specification: `docs/ESO-Weave-Specification-v0.2.0.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

Build plans 001 and 002 delivered the functional product and a first pass of
brand and UX polish (through v0.3.0). This plan adds two slices: a deeper GUI
ergonomics and information-design overhaul with pervasive auto-save, and a
net-new weapon-bar-aware timing capability that also closes the specification's
first deferred research item.

It traces to the master specification's GUI layer (section 10), its timing model
(section 7.3), the pixel bus (section 9), and open item R1 (section 16, weave
delay defaults research). Slice 013 does not touch any safety-critical surface.
Slice 014 modifies pinned contract surfaces (the pixel-bus contract, the reader
contract, and the addon manifest), so each such change carries a dated decision
recorded in `CHANGELOG.md`.

## Slices

### Slice 013: GUI Ergonomics, Information Design, and Auto-Save

Scope: raise the main window from a stack of look-alike buttons to a considered
interface. A resizable, terminal-styled monospace live-log panel with a drag
handle; a reusable colorized toggle-switch widget replacing checkboxes and
two-state action buttons (Suspend and Resume, Fishing, per-skill Enabled and
Override); real section headings using the bundled Inter weights; a renamed,
colorized, full-width status region (Status, Fishing, Pixel Beacon (Addon)) with
a labeled state column; labeled Skills columns with an inherited-default display
instead of a placeholder zero, and an override that targets the delay matching
each row's weave type; hover tooltips across the app and inline help text on
every setting; a full-frame Settings modal (roughly 90 percent) with a dimmed
backdrop and close-on-click-outside, reorganized into clustered groups with no
underscores in labels; and pervasive auto-save (configuration and session state)
with a debounced write and a gentle bottom-right save toast. All correctness
logic lands in the tested view-model and subsystems; the egui layer stays thin.
Feature under `specs/013-<name>/`.

### Slice 014: Weapon-Bar-Aware Adaptive Timing

Scope: detect which weapon bar is active and split skill-delay timing per bar,
with default presets derived from the equipped weapon type. Closes open item R1
by producing an evidence-based timing appendix to the master specification (ESO
global cooldown of 1000 ms; a weave target near 965 ms; per-weapon heavy-attack
channel durations, with dual wield the fastest). Implementation adds a fourth
PixelBeacon pixel block encoding the active pair and per-bar weapon types (using
`GetActiveWeaponPairInfo`, `EVENT_ACTIVE_WEAPON_PAIR_CHANGED`, and
`GetItemWeaponType`, with edge detection and re-baselining after loading
screens), a matching decode channel in the pixel-bus reader, a per-bar timing
model in the weave engine with weapon-type presets and an auto-timing
preference, and GUI surfacing of the detected bar and weapon types. Modifies the
pinned pixel-bus and reader contracts and the addon manifest, each with a dated
`CHANGELOG.md` decision. Feature under `specs/014-<name>/`.
