# Runtime Safety Checklist: Auto-potion Restoration

**Purpose**: Guard the synthesized-input boundary during end-to-end restoration

**Created**: 2026-09-03

**Feature**: [spec.md](../spec.md)

## Authorization

- [x] Requested enablement defaults Off and remains session-only
- [x] Every trigger precondition is a positive current observation
- [x] Explicit potion classification and usability are mandatory
- [x] Cooldown cannot imply item identity or usability

## Lifecycle

- [x] Game, focus, beacon, gate, and suspension loss block immediately
- [x] Lifecycle loss does not mutate the user's request
- [x] Recovery requires fresh eligible observations
- [x] Unknown, stale, ambiguous, and unsupported states fail closed

## Input and Observability

- [x] One eligible evaluation produces at most one complete down/up pair
- [x] Every submitted attempt starts retry suppression
- [x] Effective state exposes every blocker without verbose logging
- [x] Normal diagnostics are change-only and omit raw gameplay observations
- [x] Triggered identifies the resource cause and is bounded by the next evaluation

## Verification Boundary

- [x] Deterministic tests cover the complete rule and lifecycle matrix
- [x] The repository quality gate is mandatory before push
- [x] Real-client confirmation is deferred until a fresh release and will not be claimed by this slice
