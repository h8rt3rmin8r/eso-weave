# Canonical Player-State Checklist: S106

**Purpose**: Verify truthful projection, coherence, and transport safety
**Created**: 2026-09-17
**Feature**: [spec.md](../spec.md)

## Canonical Authority

- [x] S104 public inventory is the source of truth
- [x] `AppView` and HUD retention are explicitly excluded
- [x] Every source observation is public or deliberately non-public
- [x] Bootstrap preserves all stable domains

## Observation Semantics

- [x] Unknown, unavailable, dormant, fresh, and stale are distinct
- [x] Zero and false remain valid observed values
- [x] Source and protocol facts are stable and explicit
- [x] Derived facts inherit conservative knowledge and freshness

## Concurrency and Lifecycle

- [x] Producer lock order matches worker mutation order
- [x] Publication swaps one immutable revision
- [x] Serialization occurs after all locks are released
- [x] Service generation is lifecycle-owned and revision-neutral
- [x] Shutdown cancellation applies to the new routes

## Scope

- [x] HTTP capabilities and player state are included
- [x] MCP resources remain in issue #178
- [x] Database queries remain in issue #179
- [x] Public integrator documentation remains in issue #180
