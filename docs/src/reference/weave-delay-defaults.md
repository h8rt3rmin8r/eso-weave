# Weave Delay Defaults

These values are adjustable defaults rather than fixed gameplay constants.

**Version-sensitive:** The weapon values are community estimates pending
in-game validation. They are not guaranteed ESO timing constants.

ESO's skill global cooldown is 1000 ms. Light and heavy attacks run on a parallel
track, and weaving aims to fit one basic attack and one skill into that window. A
practical light-attack-plus-skill target is about 965 ms; exceeding 1000 ms can
drop light attacks.

The lower bound on `d_weave` depends more on server latency than local execution,
which is why it defaults to 50 ms and can use latency adaptation. The
weapon-specific timing parameter is `d_heavy`.

| Weapon class | `d_heavy` default |
| --- | --- |
| Dual wield | 640 ms |
| Sword and shield | 900 ms |
| Two handed | 1050 ms |
| Destruction staff | 1180 ms |
| Restoration staff | 1360 ms |
| Bow | 1380 ms |
| None or unknown | Configured value, no preset |

Lightning staff is included in the destruction staff class reported by the bus.
