# Contract: Roll-Dodge State and Weave Gate

## Addon input

- Event: `EVENT_COMBAT_EVENT`
- Filters: target `COMBAT_UNIT_TYPE_PLAYER`, ability ID `28549`
- Entry: `ACTION_RESULT_EFFECT_GAINED`
- Completion: `ACTION_RESULT_EFFECT_FADED`
- Recovery: fixed 1,500 ms from the most recent gained event
- Invalidation: player death and player deactivation
- Baseline: Inactive after completed player activation rebaseline

## Pixel bus

- Layout protocol: version 3, header code `0x60`
- Payload count: 24, B0 through B23
- B23 point: negotiated payload index 23
- Marker: green `0xF9`
- States: red `0x20`, `0x80`, `0xE0`
- Checksum: blue `255 - red`
- Failure value: Unknown

## Compatibility

- Legacy protocol retains its fixed historical layout.
- Negotiated version 1 exposes B0 through B21.
- Negotiated version 2 exposes B0 through B22.
- Negotiated version 3 exposes B0 through B23.
- Unsupported versions reject the layout rather than guessing its extent.

## Generated weave behavior

- Unknown and Active cause bound physical skill keys to pass through.
- Unknown and Active enqueue no new weave action.
- The worker rechecks state before cooldown accounting and sequence start.
- A running sequence observes gate closure during waits and emissions.
- Down events after closure are suppressed; releases for held generated inputs run.
- No request is retained for replay after Inactive.
- Toggle hotkeys remain exempt from the roll gate.
