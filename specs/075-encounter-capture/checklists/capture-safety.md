# Encounter Capture Safety Checklist

**Purpose**: Review the privacy, truthfulness, and addon-confinement boundary

**Created**: 2026-09-09

**Feature**: [spec.md](../spec.md)

## Consent and Ownership

- [x] Installation or addon load alone cannot begin capture
- [x] One explicit arm authorizes at most one encounter
- [x] Existing complete data requires explicit clear before replacement
- [x] Clearing affects only the encounter SavedVariables root

## Privacy

- [x] Persisted actors are encounter-local opaque integers
- [x] Callback names, account handles, character names, chat, guild, and location are excluded
- [x] No upload or network path exists
- [x] Channel and provenance do not contain personal identity

## Integrity and Loss

- [x] Sequence is authoritative and time is nondecreasing
- [x] Terminal capacity is reserved before regular events
- [x] Overflow and lifecycle interruption cannot look complete
- [x] Callback errors tear down active handlers
- [x] Exact serialized size and live parity remain verification claims

## Confinement

- [x] Encounter capture has a distinct addon and SavedVariables identity
- [x] Pixel Bus carries no bulk encounter data
- [x] PixelBeacon and discovery collector files are never mutated
- [x] No protected action, input synthesis, gameplay mutation, packet, memory, or multi-account surface exists

## Notes

This checklist is a pre-implementation design gate. Tests must make every item
observable before S075 is complete.
