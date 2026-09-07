# Weaving

While the ESO window is focused, ESO Weave can intercept a configured skill key
and submit a basic attack and skill sequence in its place. An inactive skill slot
passes its key through unchanged. Press `F1` to suspend or resume all automation.
While suspended, ESO Weave sends no input.

Weaving requires fresh evidence that the player is alive, the world is active,
travel is inactive, and roll dodge is inactive. If roll-dodge evidence is Active
or unavailable, the physical skill key passes through instead of being swallowed.
If a roll begins after a sequence starts, the remaining generated work is
discarded and any held mouse button is released.

## Skill slots

| Slot | Default key | Default type | Active by default |
| --- | --- | --- | --- |
| Skill 1 through Skill 5 | `1` through `5` | Light Attack | Yes |
| Ultimate | `R` | Light Attack | No |
| Synergy | `X` | Light Attack | No |

Every binding can be changed. Conflicting assignments are rejected.

## Weave types

"Primary" means the left mouse button, and "secondary" means the right mouse
button.

| Type | Generated sequence |
| --- | --- |
| Light Attack | Primary click, wait `d_weave`, send the skill key |
| Heavy Attack | Primary down, wait `d_heavy`, send the skill key, primary up |
| Bash Attack | Primary click, wait `d_weave`, send the skill key, wait `d_bash`, secondary down, primary click, secondary up |
| Block Casting | Secondary down, send the skill key, wait `d_weave`, secondary up |

## Timing

| Parameter | Default | Purpose |
| --- | --- | --- |
| `global_cooldown` | 500 ms | Minimum interval between submitted weave sequences |
| `d_weave` | 50 ms | Gap between a basic attack and skill key |
| `d_heavy` | 1000 ms | Heavy attack hold before the skill key |
| `d_bash` | 125 ms | Gap before the bash part of Bash Attack |

A request inside the configured global cooldown is dropped while its physical key
remains suppressed. Each slot can override the parameters used by its weave type;
a blank override inherits the global value.

ESO Weave always keeps separate base timing profiles for the primary and backup
weapon bars and selects the profile for the active bar. If the active bar is
unknown, it uses the primary profile. Weapon-aware timing then selects the
`d_heavy` preset for the active bar's weapon class; when that class is unknown,
it retains the selected profile's configured heavy attack value. See
[Weave Delay Defaults](../reference/weave-delay-defaults.md).

Optional latency adaptation computes:

```text
effective_delay = base_delay + round(k * latency_ms)
```

`k` defaults to 0.25. The adjustment applies to `d_weave` and `d_bash`, is
clamped between the base and base plus 300 ms, and does not change `d_heavy` or
`global_cooldown`. Without current latency telemetry, base values remain in use.
