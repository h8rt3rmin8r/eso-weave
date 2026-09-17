# Data Model: Canonical Player-State HTTP API

## Semantic snapshot content

- `capabilities`: schema, domains, HTTP operations, database identifiers, MCP state availability, and source protocol facts
- `application`: lifecycle, version, catalog, and managed data-addon observations
- `game`: installation, runtime, focus, context, surface, and world observations
- `pixel_bus`: layout, signal, addon, and fishing-signal observations
- `player`: weapon, combat, movement, life, roll dodge, travel, resources, latency, Ultimate, cooldowns, quickslot, and native bindings
- `automation`: fishing, auto-potion, and seven weave-slot interpretations
- `interpretation`: auto-potion, fishing, weave timing, latency, and PixelBus configuration

Semantic content excludes revision, capture time, and service generation so lifecycle changes cannot manufacture application-state revisions.

## Observation document

- `knowledge`: `observed`, `unknown`, `unavailable`, or `dormant`
- `value`: typed JSON value, absent when knowledge does not permit a value
- `observed_at`: RFC 3339 UTC string or null when unavailable
- `age_ms`: nonnegative integer or null when unavailable
- `freshness`: `fresh`, `stale`, or `not_applicable`
- `source`: stable source identifier
- `protocol`: optional protocol name, revision, layout revision, and capability reason

## Published snapshot

- `snapshot_revision`: positive monotonic integer
- `captured_at`: RFC 3339 UTC publication time
- `content`: immutable semantic snapshot content

Equality of `content` determines whether revision advances. A publisher exposes `Arc<PublishedSnapshot>` so readers cannot observe a partial replacement.

## HTTP player-state document

- `schema_version`: `1.0.0`
- `snapshot_revision`: from the published snapshot
- `captured_at`: from the published snapshot
- `service_generation`: from the running lifecycle host
- `capabilities`: copied from content
- all six content domains

## Invariants

1. All six domains are present at bootstrap and at every later revision.
2. Revision is positive and strictly increases on semantic change.
3. A request serializes exactly one immutable published snapshot.
4. Service generation never changes the semantic snapshot revision.
5. Unknown, unavailable, and dormant never become zero or false placeholders.
6. Non-public inventory never enters content.
