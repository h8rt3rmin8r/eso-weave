## Outcome

Provide exactly two encounter-capture modes that cover bounded single-fight
analysis and unattended multi-fight or raid-session collection.

## Approved control reconciliation

S092 established a no-go for real-time desktop-to-addon commands, with an
approved desktop command vocabulary of zero. The operator approved the S096
pathway that preserves this boundary:

- Capture mode, channel, and toggle authority remain explicit user actions inside ESO.
- The desktop imports terminal records and labels saved controller facts as historical.
- No custom binding, generated input, native-action piggyback, or live
  SavedVariables write is introduced to imitate a desktop toggle.

This deliberately replaces the original desktop-button assumption while
preserving the intended one-control user workflow safely.

## Scope

- Implement exactly two selectable modes:
  - Single encounter: activation is allowed before or during combat and capture
    stops at the next player-combat exit.
  - Continuous until turned off: capture spans any number of combat periods
    within declared aggregate limits until explicit disablement or hard failure.
- Provide addon-owned mode and channel selection plus one in-game toggle.
- Make selected mode, requested mode, effective state, current encounter,
  session disposition, interruption, and hard failure independently visible.
- Segment encounters inside a continuous session with stable session identity,
  contiguous ordinals, and independently replayable terminal records.
- Define explicit behavior for reload, relog, desktop exit, addon reload, crash
  durability, storage pressure, callback failure, and ingestion interruption.
- Import all terminal session members through one validated atomic transaction.
- Never add an automatic-duration mode, desktop command ingress, or silent eviction.
- Update governance, documentation, status reference, troubleshooting, and tests.

## Acceptance criteria

- [x] Single mode can start during combat, declares the unknown prefix, and ends at the next combat exit.
- [x] Single mode started outside combat waits for the next encounter and ends after it.
- [x] Continuous mode crosses combat exits and subsequent entries without another user action.
- [x] Continuous mode stops only through explicit disablement or a truthfully reported hard failure.
- [x] One in-game toggle controls the selected mode without creating extra UX modes.
- [x] Encounter and session boundaries remain reconstructable with authoritative order and loss markers.
- [x] Recovery and storage-pressure behavior cannot silently claim uninterrupted capture.
- [x] Desktop state remains read-only, flush-bound, and explicitly historical.
- [x] Batch import is atomic and idempotent, and legacy canonical evidence remains unchanged.
- [x] Documentation contains only the two approved modes and the no-command-ingress boundary.

## Dependencies

Parent epic: #182. The lossless subscribed-event contract is complete through
#198 and #200. S092 selected terminal SavedVariables import as the supported
fallback. Native-log qualification in #190 remains independent and does not
block this implementation.

## Verification

Exercise activation before combat, activation during combat, repeated encounters,
gaps, explicit stop, reload, relog, hard failure, recovery, pressure, hostile
batch input, atomic retry, and historical desktop presentation through production
Lua and Rust tests. Installed ESO observations, when needed, remain separately
tracked verification rather than repository proof.
