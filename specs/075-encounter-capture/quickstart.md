# Quickstart: S075 Encounter Capture

## Repository Verification

1. Run the encounter addon integration test. It loads the exact production Lua
   in a vendored Lua 5.1 runtime and supplies deterministic ESO API callbacks.
2. Confirm the complete-capture fixture includes every required event family,
   strict sequence, nondecreasing elapsed time, and no personal strings.
3. Run overflow, backward-clock, stop, deactivation, clear, and callback-failure
   scenarios. Each must finalize truthfully and unregister active callbacks.
4. Run static confinement checks for forbidden transport, network, action,
   equipment, item, process-memory, packet, and cross-addon references.
5. Run the complete Cargo and documentation gates from `tasks.md`.

Repository tests prove the state machine and contract. They do not prove live
API coverage, exact SavedVariables file size, or Combat Metrics parity.

## User Flow

1. Install `addon/EsoWeaveEncounter` under the ESO environment's `AddOns`
   directory and reload the game interface.
2. Outside combat, enter `/ewencounter arm live` or `/ewencounter arm pts`.
3. Begin and finish one encounter. The addon disarms automatically.
4. Enter `/ewencounter status` to inspect terminal status and stored or omitted
   event counts.
5. Use `/reloadui`, log out, or exit ESO so the game flushes
   `SavedVariables/EsoWeaveEncounter.lua`.
6. Preserve the file locally for future issue #133 import. S075 does not yet add
   a desktop import command.
7. Enter `/ewencounter clear confirm` only when the retained capture is no
   longer needed.

## Safety Expectations

- Addon load does not arm or begin a new capture.
- Captures contain no account name, character name, chat, guild, or location.
- PixelBeacon and ESO Weave Collector remain separate and unchanged.
- Partial status and discontinuities are expected when a limit, reload, clock
  reset, stop, or callback failure prevents a complete observation.
- No capture is uploaded, imported, analyzed, or used to drive gameplay in S075.
