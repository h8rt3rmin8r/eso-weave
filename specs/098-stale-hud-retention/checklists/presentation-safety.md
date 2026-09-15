# Presentation Safety Checklist: Stale HUD Retention

**Purpose**: Prevent retained display data from becoming current action evidence

**Created**: 2026-09-15

**Feature**: [spec.md](../spec.md)

## Authority Separation

- [x] Retention is owned only by `AppModel` presentation state.
- [x] Retained values are rendered types, not decoded controller inputs.
- [x] Reader and process loss routing remains unchanged.
- [x] Game, focus, signal, life, world, travel, roll, menu, Fishing, Auto Potion, and input gates remain current-evidence-only.
- [x] No retained data is persisted, logged, uploaded, or sent to an addon.

## Time and Lifecycle

- [x] One injected monotonic clock is used.
- [x] One loss timestamp governs the complete snapshot.
- [x] Cause changes and settings auto-submit cannot restart the interval.
- [x] Fresh evidence cancels stale state immediately.
- [x] Zero and expiry remove the snapshot idempotently.
- [x] Process construction and restart begin empty.

## Interface

- [x] Staleness is conveyed in text, not color alone.
- [x] Cause and age remain visible without replacing retained values.
- [x] The setting supports exact input and inclusive bounds.
- [x] Help and canonical docs state that retention never authorizes automation.
