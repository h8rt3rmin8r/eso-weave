# Research: Canonical Player-State HTTP API

## Authority mapping

The S104 inventory spans `AppModel`, `GameState`, `WeaveEngine`, `FishingController`, `AutoPotionController`, `InputEngine`, catalog access, addon lifecycle, and current reader configuration. `AppView` is not an authority. It normalizes facts into labels and may replace current unknown state with a retained display-only HUD snapshot.

Decision: capture raw authorities directly and classify presentation artifacts as non-public.

## Coherent capture

The pixel worker already mutates `WeaveEngine`, `FishingController`, and `AutoPotionController` while holding their mutexes in that order. It updates `GameState` inside the same critical section for PixelBus events. The process transition path currently updates these authorities in separate critical sections.

Decision: canonical capture uses weave, fishing, potion, then game. Align the process-transition mutation path with this order. This prevents a snapshot from pairing a new game lifecycle with uncleared player observations or the reverse.

## Immutable handoff

Options considered:

1. Serialize while holding all source locks. Rejected because a slow serializer would block worker and UI progress.
2. Use an async watch channel. Rejected because producers and consumers only need latest-value reads and the synchronous application would gain unnecessary runtime coupling.
3. Store an immutable `Arc` behind `std::sync::RwLock`. Selected because publication and acquisition are short synchronous operations, handlers can serialize after cloning, and issue #178 can reuse the same handle.

## Revision and time

`time` is already a direct dependency with formatting support. Semantic snapshot content can derive equality. The publisher increments revision only on semantic differences and formats the publication instant as RFC 3339. Service generation is supplied by the lifecycle host and does not affect application-state revision.

Decision: no new dependency. Capture timestamp belongs to the immutable published revision. Age is derived from source metadata when available and otherwise remains explicit as unavailable rather than invented.

## Observation representation

The external contract has heterogeneous leaf values but one shared metadata vocabulary. A fully distinct Rust type per leaf would add hundreds of wrapper types without increasing runtime safety, while an unstructured top-level JSON value would make envelope and revision invariants too weak.

Decision: use typed envelope, publisher, capabilities, and observation metadata, with stable domain JSON objects constructed through shared observation helpers. Contract-path tests enforce the public inventory and reject presentation-only fields.

## Transport integration

The S105 router already applies Host, Origin, bearer, body, cancellation, and stopping guards before routing. Axum handlers can hold a cloned publisher and current generation in router state.

Decision: preserve the shared security middleware, add GET-only capability and player-state routes, return method-specific structured errors, and keep MCP unchanged.
