# Data Model: Death Recovery Safety

## LifeState

| State | Actionable | Meaning |
| --- | --- | --- |
| Unknown | No | No trustworthy current B21 evidence |
| Dead | No | A death episode is active and the player is dead |
| Recovering(path) | No | The player left Dead but completion is not authorized |
| Alive | Yes | Recovery evidence and a later coherent baseline passed |

## RecoveryPath

| Path | Primary evidence | Completion boundary |
| --- | --- | --- |
| Ghost | Reincarnating query or Reincarnated event | Ghost exit evidence plus later coherent baseline |
| WorldActivation | Deactivated during death episode | Activated, rebaseline, plus later coherent baseline |
| NoLoad | Alive without ghost or deactivation | Later coherent baseline |

## Addon DeathEpisode

Runtime-only fields:

- active flag;
- current diagnostic path;
- observed Dead, Alive, Reincarnating, Reincarnated, Deactivated, and Activated evidence;
- monotonic baseline generation;
- earliest generation eligible to complete recovery;
- ghost-query fallback generation count.

No field is persisted or written to SavedVariables.

## Reader Recovery Capture

The reader owns a monotonic sample generation. A transition to Alive marks the
capture as a recovery capture. Current action-driving states are emitted even if
equal to cached values, and Life Alive is appended after them.

## Death Authorization Epoch

The input engine increments a diagnostic counter when its life gate changes from
open to closed. The existing weave and fishing authorization epochs continue to
invalidate admitted work through their atomic gates.

## Potion Retry Episode

The controller sets `recovery_retry_pending` on every non-Alive observation. On
the first tick after Alive it records that tick as the retry baseline, clears the
pending flag, and evaluates to RetryInterval. Only a later tick at or beyond the
configured interval may synthesize.
