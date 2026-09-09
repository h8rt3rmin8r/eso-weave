# Contract: Death Recovery and Action Authorization

## Addon event contract

| Evidence | Immediate state | Completion effect |
| --- | --- | --- |
| PLAYER_DEAD or dead query | Dead | Start or retain death episode |
| PLAYER_ALIVE | Recovering | Record Alive evidence, never authorize |
| reincarnating query | Recovering(Ghost) | Require ghost exit evidence |
| PLAYER_REINCARNATED | Recovering(Ghost) | Permit a later coherent baseline |
| PLAYER_DEACTIVATED during episode | Recovering(WorldActivation) or Dead | Require Activated |
| PLAYER_ACTIVATED during episode | Recovering(WorldActivation) | Rebaseline, then require a later baseline |
| no-load Alive path | Recovering(NoLoad) | Require a later coherent baseline |

Every transition is idempotent. Contradictory current queries take precedence
over optimistic completion evidence.

## Coherent baseline contract

Alive may be published only when:

1. the death episode has path-specific completion evidence;
2. the current query says not dead;
3. the current query says not reincarnating;
4. the world state is Active;
5. the current generation is later than the event or fallback generation that
   made completion possible.

## Reader ordering contract

- Dead, Recovering, and Unknown Life events precede all action-driving events in
  the same sample and close the gate through the pre-lock safety route.
- On recovery, current world, menu, travel, movement, cooldown, quickslot, and
  resource events precede Life Alive.
- Life Alive opens only after controllers own the current recovery capture.
- The sample generation is present in transition diagnostics.

## Controller reset contract

- Weave relies on existing shared gates and authorization epochs. No request or
  running sequence admitted before death may emit a new Down afterward.
- Fishing cancels active deadlines on every non-Alive state and emits nothing on
  Alive alone.
- Auto-potion preserves requested enablement, starts a new retry episode, and
  emits nothing until one complete configured retry interval after the coherent
  recovery tick.

## Compatibility contract

- B21 index, marker, complement checksum, Alive value, and Dead value remain.
- Legacy 0xE0 decodes as Recovering(Ghost), never Alive.
- Unknown or ambiguous values decode to Unknown.
- Old companions decode new recovery values as Unknown and remain fail closed.
