# Safety and Persistence Checklist

**Purpose**: Prevent persisted intent from becoming independent input authority

**Feature**: [spec.md](../spec.md)

## Authority Separation

- [x] Only requested enablement is persisted.
- [x] Effective state, telemetry, blockers, cooldown, retry history, and trigger
  cause remain transient.
- [x] `config.json` does not duplicate the request.
- [x] Restoration uses the existing controller boundary.

## Fail-Closed Startup

- [x] Missing and legacy state default the request to disabled.
- [x] Malformed state falls back as one complete document.
- [x] Restoration cannot tick the controller or synthesize input.
- [x] Existing closed startup gates remain closed.
- [x] Enabling a stored request does not fabricate fresh evidence.

## Persistence Integrity

- [x] The schema version advances.
- [x] UI and F3 write one authoritative request.
- [x] Coalesced and close-time saves capture the latest request.
- [x] Applying restored state does not schedule a save loop.
- [x] Existing session fields retain their values across migration.

## Non-Regression

- [x] Input engine and hook-thread behavior are unchanged.
- [x] Pixel Bus and addon contracts are unchanged.
- [x] Auto Potion trigger and retry contracts are unchanged.
- [x] Documentation removes obsolete session-only claims.
