# Research: Death Recovery Safety

## D1. Own recovery authorization in PixelBeacon

**Decision**: Replace direct query-to-Alive mapping with a small addon-side death
episode arbiter. Events provide prompt lifecycle evidence and periodic current
queries provide convergence and a coherent-baseline boundary.

**Rationale**: The companion cannot reconstruct callbacks hidden behind one
sampled pixel. Authorizing in the addon keeps the decision beside the ESO API
evidence and preserves the existing outside-the-game pixel contract.

**Rejected**: An unconditional desktop delay. Time alone cannot distinguish
ghost, loading, no-load, missed-event, or stalled recovery paths.

## D2. Retain B21 and encode recovery paths

**Decision**: Keep Alive at 0x20, Dead at 0x80, and legacy ghost recovery at
0xE0. Add distinct tolerance-separated values for world-activation and no-load
recovery without changing the marker, checksum, block index, or block count.

**Rationale**: Old companions fail closed on new values, new companions interpret
the legacy 0xE0 value safely, and no protocol geometry migration is required.

**Rejected**: A second diagnostic block. It expands layout and cross-language
surface without adding an authorization capability.

## D3. Use path-specific completion plus a later baseline

**Decision**: Ghost recovery uses Reincarnated evidence or a bounded repeated-query
fallback, load recovery requires Activated and rebaseline, and no-load recovery
requires Alive evidence. Every path then waits for a later query generation that
confirms not dead, not reincarnating, and active world state.

**Rationale**: This uses authoritative lifecycle evidence where available while
remaining robust to duplicate and some missed callbacks. A missing activation
cannot be guessed safely and remains blocked.

## D4. Route recovered Alive last in its capture

**Decision**: On an Alive transition, the reader emits current world, menu,
travel, movement, cooldown, quickslot, and resource observations before the Life
Alive event, even when their values did not change.

**Rationale**: Opening after cache refresh removes the same-cycle stale-input
race without adding locks or work to the hook thread.

**Rejected**: Timestamps on every public observation type. That broad model is
unnecessary when one deterministic capture boundary can establish freshness.

## D5. Start a new potion retry episode at coherent recovery

**Decision**: The controller records the first Alive tick as the new attempt
baseline and suppresses output until the configured retry interval elapses.

**Rationale**: The rule directly matches the reported regression and works whether
or not a pre-death attempt existed.

**Rejected**: Clearing `last_attempt_ms` on death. `None` makes the first recovered
eligible tick fire immediately, which is the bug.

## D6. Use existing shared gates and epochs

**Decision**: Keep the S060 atomic gate and authorization epoch mechanism. Add a
diagnostic death epoch at the `InputEngine::set_life_gated` transition rather than
inventing a second coordinator.

**Rationale**: Existing sinks already cancel queued and running work correctly.
The missing contract is recovery authorization and controller reset, not another
synthesis engine.

## D7. Preserve release verification separation

**Decision**: Repository tests close #109. Installed ESO scenario evidence remains
in #110 after a v0.15.1 artifact is explicitly published.

**Rationale**: Event sequences from live PvE and PvP cannot be manufactured by
unit tests, while implementation correctness must not wait inside the same issue.
