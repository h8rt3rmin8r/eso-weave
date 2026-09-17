# Data Model: MCP Player-State Resources

## Resource Inventory

### Capabilities resource

- URI: `esoweave://capabilities`
- Name: `capabilities`
- Title: `ESO Weave Capabilities`
- Media type: `application/json`
- Value: The `Capabilities` document from the latest canonical snapshot
- Mutability: Read-only current value

### Player-state resource

- URI: `esoweave://player-state`
- Name: `player-state`
- Title: `ESO Weave Player State`
- Media type: `application/json`
- Value: The latest canonical `PlayerStateDocument` with the active service generation
- Mutability: Read-only current value

## Adapter State

The MCP adapter contains only:

- A cloneable handle to the canonical `SnapshotPublisher`
- The immutable service generation assigned to the current running host

It contains no resource cache, client session, revision counter, subscription table, or mutable projection.

## Read Flow

1. Match the requested URI exactly.
2. Clone the publisher's current immutable snapshot reference.
3. Select capabilities or construct the player-state document with the handler's service generation.
4. Serialize the selected canonical type once.
5. Return one JSON text resource content item.

## Invariants

- A single read derives from exactly one immutable snapshot revision.
- HTTP and MCP use the same canonical Rust types and generation value.
- Reading does not mutate the publisher or advance revision.
- Resource listing is fixed, ordered, and complete in one page.
- Unknown URIs never select a partial or normalized match.
