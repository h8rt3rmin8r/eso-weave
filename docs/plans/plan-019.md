# Build Plan 019: World Transition Truth

## Context

PixelBeacon re-baselines player observations after a loading screen but does not
publish the interval in which those observations are stale. The companion can
therefore continue to display the last visible gameplay values while ESO is
leaving one world state and constructing the next.

## Slice 049: Authoritative world-transition state

Implement issue #56 as one end-to-end observable slice:

1. add a B22 world-state block with Unknown, Transitioning, and Active values;
2. publish Transitioning immediately from `EVENT_PLAYER_DEACTIVATED`;
3. re-baseline every dependent payload before publishing Active from
   `EVENT_PLAYER_ACTIVATED`;
4. decode invalid, absent, or lost evidence as Unknown;
5. route the normalized state into the shared game observation model;
6. present the state truthfully in Live HUD, including dormant game handling;
7. advance the managed addon manifest and protocol documentation; and
8. cover wire agreement, lifecycle ordering, signal loss, routing, and display
   behavior with deterministic tests.

Pre-loading recall detection and synthesis gating remain in #59. Roll dodge and
sprint detection remain in their existing atomic issues.

## Exit gate

- Issue #56 closes through the pull request.
- PixelBeacon and the companion agree on the 23-block protocol.
- Loading entry publishes Transitioning before stale observations can be treated
  as current.
- Loading exit publishes Active only after the new-world baseline is complete.
- Missing, corrupt, or lost B22 evidence presents as Unknown.
