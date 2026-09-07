# Game Observation and Safety State

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

## World, travel, life, roll dodge, and movement

World State is **Transitioning** from player deactivation until activation has
refreshed every player-derived payload. Only that complete activation establishes
**Active**. No timer infers Active.

Travel is **Pending** from bounded recall or jump evidence until movement,
cancellation, failure, loading, or watchdog recovery. Generated input requires
World State Active and Travel Inactive. Physical input passes through and blocked
automation is discarded rather than replayed.

Life State is Alive, Dead, Reincarnating, or Unknown. Reincarnating takes
precedence over Dead. Missing, invalid, loading, or lost evidence becomes Unknown
and blocks synthesis.

Roll Dodge is Active from the player-scoped dodge event until its matching fade.
A 1500 ms watchdog clears a rejected dodge with no fade. Death, loading, invalid
data, and signal loss restore Unknown until a fresh baseline is established.

Movement reports On Foot, Mounted, Sprinting, or Unknown. Keyboard-mode on-foot
sprint is a bounded inference with entry and exit debounce, lifecycle exclusions,
and stale-positive expiry. Gamepad and mounted sprint are not inferred. Explicit
Sprinting delays Auto Potion evaluation; Unknown does not fabricate a sprint.
