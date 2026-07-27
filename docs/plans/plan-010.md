# Build Plan 010: PixelBeacon Player-State Expansion

Plan: 010
Status: active
Master specification: `docs/ESO-Weave-Specification-v0.2.0.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

Build plan 009 (slice 030, released as v0.8.1) closed the last of the open
interface reports. Every issue still open on the tracker, apart from the
deferred specification refresh (#15), is PixelBeacon work: #9 in-combat
state, #10 native menu state, #2 Health, Stamina, and Magicka resource
levels, #3 out-of-band display detection, and #11 mount and sprint movement
state. This plan is the response and it sequences all five, because four of
them are the same shape and the order in which they run materially changes
their cost.

The four block-adding issues (#9, #10, #2, #11) each repeat one pattern: an
encoder in `addon/PixelBeacon/PixelBeacon.lua`, a matching decode point in
`src/pixelbus/mod.rs`, and a color contract shared byte for byte between
them. Today that pattern exists only as precedent set by the weapon-bar
block (B3) and it is not factored: the block count is the literal `4` in
`NUM_BLOCKS` (whose own doc comment says "adding blocks is a separate
feature"), a second literal `4` in the addon's root width, and a fixed
four-argument `observe` signature. Every one of the four issues has to widen
all three. Doing them in an arbitrary order means paying that widening cost
four times, and it means the riskiest color contract in the family (the
hundred-entry resource table in #2) would be authored before the pattern it
depends on has ever been exercised end to end.

So the plan runs the cheapest block first and treats it as the reference
implementation. Slice 031 adds one boolean block for #9, and its real
deliverable is larger than the block: it converts the block count and the
sample-point set from literals into a derived contract that the following
slices extend by adding one entry. #10 follows, because it is the highest
value of the four (it is the input-suppression gate) and because it is the
only one that reaches into the input engine, so it should be built on a
proven block pattern rather than alongside an unproven one. #2 follows that,
its color table being the one deliverable in the family that needs field
measurement rather than a decision. #3 is independent of all of them (it
touches no addon code at all) and is sequenced fourth because nothing else
waits on it until the grid-wrap feature it enables is filed. #11 runs last
because it is the only slice with an unresolved external blocker: the sprint
observable has no confirmed API and the issue itself requires that
verification land before any encoding is fixed.

One ordering constraint binds the whole plan and is why these slices do not
run concurrently. Each new block claims the next physical index and widens
the strip; two slices in flight would collide on both. The slices run one at
a time, in the order below.

This plan traces to the master specification's pixel-bus contract (section
10.3) and, for slice 032, its input-suppression scoping. Note that the master
specification is itself stale at v0.2.0 and is scheduled for the refresh
tracked in #15; each slice below updates the sections it touches rather than
waiting on that refresh.

## Slices

### Slice 031: PixelBeacon In-Combat State Block

Scope: add a fifth beacon block (B4) encoding the player's combat state, and
factor the block-extension pattern the rest of this plan depends on. Closes
issue #9.

In the addon, a `renderCombat()` sets the block from the current combat
boolean, driven by `EVENT_PLAYER_COMBAT_STATE` for instant transitions and
re-baselined from `IsUnitInCombat("player")` on `EVENT_PLAYER_ACTIVATED`
after each loading screen, exactly as the weapon block re-baselines today.
Both names are verified present in `esoui/esoui`, so this slice carries no
open API question. The render is change-detected like B3, so the read-back
signal only moves on a real transition. The block takes a dedicated green
marker channel, and the value chosen must stay clear of every green already
on the strip by more than `ReaderConfig.tolerance`: `0x00` (status magenta),
`0x80` and `0xFF` (the two fishing colors), `0xA5` (latency), and `0x5A`
(weapon). The manifest advances from version 5 to 6 so the beacon manager
offers the update, and its description line gains the new signal. The
companion reads the embedded manifest for the version, so there is no
separate pin to advance.

On the reader side the slice adds a combat sample point, a decoder that
validates the marker and maps the two colors to a tri-state (in combat, out
of combat, and an unknown or absent case consistent with every other
decoder), and a `PixelBusEvent` variant emitted only on a decoded change and
cleared on signal loss, following the weapon-bar handling in `observe`
byte for byte. The block is optional in exactly the sense B3 is: an install
still running addon version 5 draws nothing at B4, whatever is behind the
overlay fails the marker check, and the decoder yields the absent case
rather than a false reading. That backward-compatible path is a required
test, not an assumed one.

The factoring is the part that outlives this slice. `NUM_BLOCKS` becomes 5
and its "adding blocks is a separate feature" comment goes away, the addon's
root width stops being a second literal and derives from a block-count
constant, and `observe`'s fixed four-sample signature is replaced by a form
that admits a fifth without another signature break for each following
slice. `capture_dims` already derives from `NUM_BLOCKS` and needs no change
beyond it, which is the pattern the rest should match: geometry derived, not
restated. Whether `observe` takes a slice, an array, or a named struct is a
decision for the feature plan, evaluated against the existing tests in
`tests/pixelbus.rs`, which the change must carry.

Surfacing follows the weapon-bar precedent and stops there: the decoded
value routes through `src/app/routing.rs` into the view model beside the
weapon bar and is logged on change at DEBUG, so the operator can confirm the
signal in the live log. Gating weave behavior, timing, or input on combat
state is explicitly out of scope for this slice; it adds the observable and
nothing that acts on it. Safety invariants are untouched: this slice adds no
input path, and fishing still degrades to disabled on signal loss.

`CHANGELOG.md` records an `Added` entry, plus a dated decision for the
marker value and for the `observe` signature change, both being contracts
later slices inherit. The master specification's section 10.3 gains the B4
block. The feature quickstart defines the in-game validation: update the
addon to version 6, reload the interface, and confirm the live log shows the
combat state changing on entering and leaving combat and re-baselining
correctly after a loading screen. Feature under `specs/031-<name>/`.

### Slice 032: PixelBeacon Menu-State Gate

Scope: add the native menu-state signal and wire it into input suppression.
Closes issue #10.

The load-bearing deliverable is a single gate block answering "is any native
menu or UI-mode surface active", derived from the HUD and HUDUI scene test
(the addon's existing `isMenuOpen()`) or from `IsGameCameraUIModeActive`,
whichever the feature's clarify step establishes as authoritative for the
edge cases the issue names (interact dialogs and the chat-entry state). It
is derived rather than enumerated on purpose, so it stays correct for a
scene nobody listed. A second, additive block encoding which surface is
active as a shared code table is the recommended form of the issue's
per-surface request; one boolean block per surface is the alternative and
should be taken only if a consumer genuinely needs several surfaces as
simultaneous independent bits. Updates are driven off `SCENE_MANAGER`
callbacks so the gate moves instantly, with the existing 1 Hz tick as a
re-sync backstop.

The gate then suspends keystroke interception and synthesis while a menu is
up, so typing in an in-game text field is never disturbed. This slice
touches a constitution NON-NEGOTIABLE surface and the discipline is
explicit: the gate ANDs with focus and never replaces it, it may only relax
interception and can never cause suppression outside the focused game
window, and the existing focus-scoping tests are extended rather than
adjusted. Signal loss must fail safe in the direction the existing design
already fails: an absent gate block leaves suppression exactly as it is
today. Feature under `specs/032-<name>/`.

### Slice 033: PixelBeacon Resource Blocks

Scope: add Health, Stamina, and Magicka blocks reporting each pool as a
percentage of its current maximum. Closes issue #2.

Three blocks, and a shared color table at 1 percent granularity (101
entries including empty) if that survives measurement, falling back to 5
percent (21 entries) only if single-point steps cannot be distinguished
reliably. That measurement is the slice's gating deliverable and it is
empirical, not a decision: adjacent entries must stay apart under
`ReaderConfig.tolerance` and whatever the capture path does to the colors on
the way through. The table is generated from one source rather than typed
twice, since a hundred entries transcribed by hand into both `PixelBeacon.lua`
and the reader is the failure this family's shared-constants discipline
exists to prevent. Update cadence rides the existing tick with
change-detection, so steady pools keep the read-back signal quiet.
Feature under `specs/033-<name>/`.

### Slice 034: Out-Of-Band Display Detection

Scope: resolve the active game render resolution and the physical screen
geometry without reading the pixel bus. Closes issue #3.

This is the only slice in the plan that changes no addon code. It extends
the platform samplers (`src/pixelbus/windows.rs`, `src/pixelbus/linux.rs`)
to report the monitor the game window sits on, its DPI or scale, and the
window origin alongside the client rect they already resolve, and adds an
optional cross-check that parses the ESO `UserSettings.txt` video keys. The
output is a small stable descriptor, re-resolvable when the window moves,
resizes, or changes display mode. It is scoped to detection and relay only;
the grid-wrap layout it enables is a separate feature to be filed once the
descriptor exists. Two verification items carry into the slice: the
`FULLSCREEN` enum to window-mode mapping is unconfirmed and needs a second
data point, and the key names should be re-verified against a live install
in case a patch has version-suffixed one. Feature under `specs/034-<name>/`.

### Slice 036: PixelBeacon Movement-State Block

Numbered 035 when this plan was written. Because it stayed blocked while later
work proceeded, the grid wrap (build plan 011, issue #16) took 035 as the next
free spec directory, and this slice takes 036. Its position in this plan's
ordering, last, is unchanged.

Scope: add a movement-mode block covering mounted and sprinting state.
Closes issue #11.

Blocked, and deliberately last. `IsMounted()` and
`EVENT_MOUNTED_STATE_CHANGED` are verified present, but no direct sprint
boolean was found: `IsUnitSprinting`, `IsPlayerSprinting`, and
`EVENT_SPRINT_STATE_CHANGED` all return zero hits in `esoui/esoui`. The
issue requires that the real sprint observable be verified before any
encoding is fixed, and that verification is the slice's entry condition, not
part of it. If sprint proves unreliable or inconsistent between the keyboard
and gamepad UIs, the slice ships the mounted axis alone and sprint is filed
as a follow-up; encoding a flaky signal into a shared color contract is the
outcome this ordering exists to avoid. One block with a small code table
covering both axes is preferred over two boolean blocks. Feature under
`specs/036-<name>/`.
