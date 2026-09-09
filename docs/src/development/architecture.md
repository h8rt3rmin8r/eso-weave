# Architecture and Ownership

ESO Weave is one Rust process composed of cooperating subsystems. The interface
owns presentation and configuration. Engines own correctness-bearing logic behind
test seams. Platform modules contain operating-system calls.

## Subsystem ownership

| Subsystem | Owns | Does not own |
| --- | --- | --- |
| Input Engine | Focus-scoped physical-event decisions, bindings, held-key bookkeeping, suspension, authorization epochs, shared synthesis gates, non-blocking handoff | Timed sequences and controller state |
| Game Observer | Installation candidates, process and launcher presence, focus, freshness, surface, and normalized Game Context | Pixel decoding and feature decisions |
| Weave Engine | Skill configuration, current timing, Global Cooldown, action sequences, and observable combat data | Physical hook callback and autonomous feature timers |
| Fishing Controller | Requested fishing state, detector events, deadlines, stop reasons, and Interact output | Pixel capture and hook decisions |
| Auto Potion Controller | Requested state, ordered eligibility rule, last attempt, and Quickslot output | Resource decoding and potion selection |
| Pixel Bus Reader | Layout negotiation, one-frame sampling, decoding, change detection, freshness, invalidation, and display description | User feature policy |
| Beacon Manager | AddOns discovery, ownership classification, embedded install files, managed in-place update and removal, block-size redeploy, and API-version upkeep | ESO runtime loading of the addon |
| Config and Session State | Separate user settings and derived runtime stores, notices, and serialization | Module-specific validation semantics |
| Logging | Global capture level, input suppression, bounded ring, and optional monthly file sink | UI presentation |
| Interface and App Model | Presentation, UI intent routing, persisted drafts, save scheduling, and view projection | Platform input and screen capture |
| Documentation Service | Immutable embedded-site lookup, bounded loopback GET and HEAD responses, browser handoff, and worker lifetime | Filesystem serving, application state, remote content, and mutation |
| Catalog Compiler and Runtime | Explicit normalized ingestion, provenance, coverage, semantic checksums, atomic publication, rollback evidence, and typed read-only queries | Startup generation, network discovery, user encounter storage, or UI-owned SQL |
| Discovery Collector | Explicit bounded public-API enumeration, deterministic local SavedVariables records, restricted staging, and an independent managed addon lifecycle | PixelBeacon, combat capture, input generation, network transfer, direct SQLite publication, or distributable game art |

## Thread model

ESO Weave uses `std::thread`, `Arc<Mutex<...>>`, and `std::sync::mpsc`; it has no
async runtime.

| Thread | Receives | Owns or calls | Must not do |
| --- | --- | --- | --- |
| Main | UI events, queued application toggles, background API outcome | Interface, App Model, save scheduler, view state | Timed input sequences |
| Interception | Platform keyboard events | Focus refresh, `InputEngine::classify`, bounded handoff | Sleep, block, or synthesize |
| Weave worker | Actions from the bounded input channel | Application-toggle forwarding and `WeaveEngine::handle` through `RealSink` | Touch the interception callback |
| Pixel Bus worker | Clock deadlines, process probes, display and pixel samples | Game observations, safety pre-routing, controller routing, fishing ticks, Auto Potion ticks | Sample through another thread or treat stale data as current |
| API version check | Stored API cache, addon root, one bounded HTTP result | Monotonic API-version resolution and managed manifest update | Delay the first window or guess a numeric ESO API version |
| Documentation worker | Bounded loopback requests after Help > Documentation is chosen | Exact embedded asset lookup and read-only HTTP responses | Read request-derived filesystem paths or access application state |

Five ownership contracts are load-bearing:

1. The interception callback never sleeps or blocks.
2. Every timed sequence runs on a worker.
3. Application hotkeys and on-screen controls converge on the same interface
   intent path.
4. Pixel-bus and interface deadlines use one monotonic clock origin.
5. Network version checking runs once in the background and never blocks startup.
6. Closing focus, suspension, or menu authorization advances an epoch observed by
   queued and running weave work; a stale epoch cannot resume after recovery.
7. Every PixelBeacon writer rechecks managed ownership at its write boundary.

Platform and hardware boundaries are represented by traits so engine, controller,
and decoder behavior can be tested with deterministic mocks.

## Data flow

Physical input follows this text sequence:

`platform event -> Input Engine decision -> pass to ESO OR suppress -> bounded action queue -> weave worker -> platform synthesis`

Game observation follows a separate sequence:

`process and focus probe + displayed pixels -> Pixel Bus Reader -> close unsafe atomic gates -> lock engines and controllers -> route observations -> feature ticks -> view model`

Configuration follows:

`UI intent -> module validation and live application where implemented -> mark store dirty -> settle interval -> JSON write`

The stores are split deliberately. `config.json` contains user settings.
`state.json` contains suspend and fishing intent, API-version cache, and window
geometry. Auto Potion request is runtime-only.

Logging follows:

`structured event -> global level and input-suppression filter -> bounded in-memory ring -> optional monthly file sink`

The Live Log reads the ring after its dropdown has applied and persisted the
global captured level used by both the ring and optional file sink.

Documentation follows a separate read-only path:

`Help action -> one application-owned 127.0.0.1 listener -> exact embedded asset lookup -> default browser`

Catalog data follows another read-only application path:

`reviewed normalized JSON -> explicit catalog-compiler command -> verified catalog.sqlite -> package path -> typed read-only application queries`

User-local API discovery precedes that path when explicitly requested:

`explicit addon install -> explicit in-game capture -> SavedVariables save -> restricted importer -> reviewed normalized JSON`

The collector is a separate addon and lifecycle boundary from PixelBeacon. It
does limited work per update tick, pauses in combat, and emits only a fixed
versioned table whose chunk payloads are deterministic JSON lines. The desktop
parser accepts that data grammar without a Lua runtime and stages through the
same strict catalog model used by reviewed source bundles.

The compiler is a second binary in the existing Cargo package, not a `build.rs`
side effect or workspace. It builds a sibling candidate in one transaction,
validates and syncs it, preserves rollback evidence, then publishes atomically.
The application holds the opened version until a controlled reopen and degrades
to an empty typed catalog with a visible diagnostic on any verification failure.

Release-profile `build.rs` runs the pinned mdBook and link-check renderer, then
emits a sorted Rust manifest into Cargo's output directory. The executable
contains those bytes directly. Debug and test profiles use a small checked
fixture so ordinary Rust compilation does not require documentation tools. The
worker accepts only GET and HEAD, rejects ambiguous paths, never consults the
filesystem, and terminates with the interface owner.

## Ordering rules

Safety-closing Pixel Bus events update shared atomic gates before the worker waits
for controller locks. Safe recovery updates the owning engine or controller
before reopening interception. This asymmetric ordering prevents a recovered
physical key from being suppressed against stale worker state.

The input callback uses `try_send`, so overload never shifts work onto the hook
thread. The tradeoff is explicit: a full or disconnected queue drops the handed
off action after its physical event was suppressed and records a warning.

The App Model and Pixel Bus worker share one monotonic origin for fishing
deadlines. Wall-clock changes therefore cannot move an armed reel or recast
deadline. The App Model publishes complete scalar reader updates through a
standard-library channel. The worker's deadline wait wakes for an update, drains
rapid edits to the newest value, and applies one configuration boundary to both
decoding and cadence. Block geometry remains startup-owned because PixelBeacon
and screen capture must change together.

See [Action Authorization](../concepts/action-authorization.md) for the complete
gate comparison, [State Machines](state-machines.md) for transitions, and
[Test Strategy](test-strategy.md) for the evidence seams.
