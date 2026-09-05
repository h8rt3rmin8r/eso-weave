# Build Plan 018: Life State and Automation Safety

## Context

ESO exposes authoritative player death and reincarnation state, but PixelBeacon
does not publish it and the companion therefore cannot protect autonomous input
while the player is unable to act. The responsive dashboard also needs room for
additional state signals without making the main window permanently tall.

## Slice 048: Life state safety and compact system disclosure

Implement issues #53, #54, #55, and #58 as one end-to-end slice:

1. rename System and automation to System and State;
2. make the complete panel an accessible disclosure whose open preference is
   persisted and whose collapsed layout leaves no blank region;
3. add an authoritative PixelBeacon life-state block for Alive, Dead, and
   Reincarnating, with Unknown reserved for missing or invalid evidence;
4. publish the normalized state through the reader, routing, model, and Live HUD;
5. require Alive before weave, fishing, or auto-potion synthesis;
6. discard blocked work rather than replaying stale actions after recovery; and
7. cover the wire contract, fail-closed routing, every synthesis path, disclosure
   accessibility, persistence, and responsive sizing with deterministic tests.

World-transition, roll-dodge, sprint, and effect-database work remain separate
atomic issues and are not absorbed into this slice.

## Exit gate

- Issues #53, #54, #55, and #58 close through the pull request.
- PixelBeacon and the companion agree on the 22-block protocol.
- Dead, Reincarnating, Unknown, and signal loss cannot synthesize input.
- Returning to Alive never replays a queued weave, fishing timer, or potion trigger.
- The disclosure is keyboard accessible, defaults open, persists its preference,
  and remains safe at both dashboard breakpoints.

