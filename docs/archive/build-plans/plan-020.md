# Build Plan 020: Roll-Dodge Truth and Weave Safety

## Context

ESO exposes a roll-dodge effect event, but a sprint-rejected roll can produce an
entry event without the matching completion. ESO Weave currently publishes no
roll state and can begin or continue generated weave work during that interval.

## Slice 050: Bounded roll-dodge state and generated-weave gate

Implement issues #57 and #60 in one end-to-end slice:

1. observe player-targeted ability 28549 effect gained and faded events;
2. recover a missing completion through a fixed 1,500 ms watchdog;
3. invalidate state on death, zoning, signal loss, and game exit, ignoring late
   combat events until a fresh lifecycle baseline;
4. publish Unknown, Inactive, and Active through protocol-version-3 B23;
5. preserve negotiated versions 1 and 2 at their historical payload extents;
6. pass physical skill input through while Active or Unknown;
7. drop queued work and cancel new generated down events in running sequences;
8. present Roll dodge truthfully with the generated-weave blocker explained; and
9. cap safety-authoritative companion sampling so Active cannot fall wholly
   between supported reads; and
10. cover normal completion, the known sprint-rejection defect, lifecycle edges,
   protocol compatibility, non-replay, and recovery with deterministic tests.

Sprint detection and auto-potion sprint deferral remain separate issues.

## Exit gate

- Issues #57 and #60 close through the pull request.
- PixelBeacon and the companion agree on protocol version 3 and 24 payload blocks.
- Active and Unknown produce no generated weave down events.
- Physical player input and application toggle hotkeys retain their behavior.
- A missing completion cannot strand Active past 1,500 ms.
