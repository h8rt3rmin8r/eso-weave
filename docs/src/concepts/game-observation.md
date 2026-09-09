# Game Observation and Safety State

Game Context is the gameplay detection or focused game summary. World State may
appear during a loading screen, world transition, or player activation. Roll
Dodge is also called a dodge roll, dodge state, or roll gate.

Installation, runtime, focus, and in-game telemetry are independent facts. ESO
Weave keeps them separate so a weak observation cannot impersonate a safe state.

## Installation and runtime

Windows installation candidates come from Steam, the generic ESO uninstall
entry, and Epic manifests. Linux candidates come from Steam library and app
metadata. Every candidate must contain the expected launcher and client files.
Provider-owned evidence wins over a generic entry for the same root; conflicting
strong providers or distinct roots are ambiguous.

Runtime reduces observations in this order: a present game client is **Active**;
otherwise unknown game evidence is **Unknown**; a present launcher is
**Launcher open**; an unknown launcher remains **Unknown**; and two absent
observations are **Inactive**. Closing the launcher cannot demote an active game.

The Pixel Bus worker probes installation, process presence, and focus at most one
second apart. A long configured pixel-sampling interval is capped at the next
process-probe deadline, so runtime changes are not delayed by idle sampling.
Process enumeration failure produces Unknown rather than a clean negative.

## Game Context

**Gameplay** requires all of the following:

- ESO runtime is Active.
- The ESO window is focused.
- PixelBeacon telemetry is fresh.
- The observed surface is valid and no native menu or text field is open.

Missing or invalid surface evidence is unavailable, not Gameplay. Runtime exit
clears game-derived observations, held-key state, and menu-gate state. It blocks
input and pauses autonomous features without rewriting their requested toggles.
Restart republishes a complete fresh baseline even when values appear unchanged.

Game Context reports Not Detected for every known non-active runtime, Unfocused
for a known active game without focus, Signal Unavailable for missing freshness
or surface evidence, a named surface for a decoded menu, and Unknown when process
or focus observation itself is unknown. No later axis can upgrade an earlier
uncertain one.

## World, travel, life, roll dodge, and movement

World State is **Transitioning** from player deactivation until activation has
refreshed every player-derived payload. Only that complete activation establishes
**Active**. No timer infers Active.

Travel is **Pending** from bounded recall or jump evidence until movement,
cancellation, failure, loading, or watchdog recovery. Generated input requires
World State Active and Travel Inactive. Physical input passes through and blocked
automation is discarded rather than replayed.

Life State is Alive, Dead, Recovering, or Unknown. Recovering identifies a ghost,
world-activation, or no-load path. A player-alive event begins or continues
recovery but does not authorize input. Alive requires the path's completion
evidence and a later coherent baseline that confirms the player is neither dead
nor reincarnating. Missing, invalid, loading, or lost evidence becomes Unknown
and blocks synthesis.

Roll Dodge is Active from the player-scoped dodge event until its matching fade.
A 1500 ms watchdog clears a rejected dodge with no fade. Death, loading, invalid
data, and signal loss restore Unknown until a fresh baseline is established.

Movement reports On Foot, Mounted, Sprinting, or Unknown. Keyboard-mode on-foot
sprint is a bounded inference with entry and exit debounce, lifecycle exclusions,
and stale-positive expiry. Gamepad and mounted sprint are not inferred. Explicit
Sprinting delays Auto Potion evaluation; Unknown does not fabricate a sprint.

## Invalidation and recovery order

Safety-closing observations are applied to shared atomic gates before the Pixel
Bus worker waits for controller locks. Safe recovery opens a gate only after the
corresponding engine and controllers hold the same fresh observation. This
ordering prevents a physical key from being newly suppressed while the worker
still holds stale unsafe state.

After resume, World State and Travel are explicitly invalidated. The next valid
sample republishes them even when their values match the pre-suspend values.
After game exit, the reader is reset and all game-derived values start Unknown.
After signal loss, action-driving observations clear instead of retaining stale
values.

After a death episode, the recovery capture republishes current world, menu,
travel, roll-dodge, movement, cooldown, quickslot, and resource observations
before Life becomes Alive. Fishing requires a later fresh cast observation.

The source-backed transition tables are in
[State Machines](../development/state-machines.md).
