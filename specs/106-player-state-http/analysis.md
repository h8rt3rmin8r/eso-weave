# Spec-Kit Analysis: Canonical Player-State HTTP API

**Date**: 2026-09-17

**Gate**: PASS

## Authority and Scope

- Issue #177 maps to the next step of active Plan 045 and is unblocked by completed issues #175 and #176.
- The packet implements the S104 canonical player-state contract through the S105 lifecycle host without changing its loopback, authentication, or shutdown boundaries.
- MCP resources, query execution, and complete public documentation remain owned by issues #178 through #180.

## Consistency

- Specification, plan, data model, HTTP contract, checklist, and tasks use the same six-domain schema 1.0.0 model.
- Snapshot revision is application-state authority; service generation remains lifecycle authority.
- `AppView`, HUD retention, logs, paths, credentials, and controller internals are consistently non-public.
- Success criteria trace to inventory, fixture, publisher, HTTP, loss, concurrency, and shutdown tests.

## Security and Safety

- Existing Host, Origin, bearer, body-size, cancellation, and loopback rules remain in front of every route.
- No write, input, automation-control, database-query, MCP-resource, raw-log, filesystem, or credential surface is added.
- HTTP handlers clone one immutable reference and perform no source locking or application mutation.

## Constitution

- Full spec-kit artifacts precede implementation.
- Test-first tasks cover canonical truth, revisions, concurrency, transport behavior, and lifecycle integration.
- Existing dependencies suffice, and no pinned process artifact changes.
- Text hygiene and full CI parity remain blocking gates.

## Findings Resolved

1. Serializing `AppView` would expose presentation state and stale retained values. The design introduces a separate canonical authority.
2. Independent controller reads could pair pre-transition and post-transition facts. Capture and process mutation use the established weave, fishing, potion, then game order.
3. Letting the lifecycle host own application content would couple state to enablement. The publisher is application-owned and generation is overlaid by the handler.
4. A fully untyped JSON root would weaken invariants, while hundreds of leaf wrapper types would add low-value complexity. The selected model types the envelope, capability, observation, and publisher contracts while enforcing domain paths through maintained tests.

No CRITICAL, HIGH, or unresolved MEDIUM finding remains. Implementation may proceed.
