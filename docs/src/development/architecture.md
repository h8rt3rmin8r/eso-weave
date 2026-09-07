# Architecture and Ownership

ESO Weave is one Rust process composed of cooperating subsystems. The interface
owns presentation and configuration. Engines own correctness-bearing logic behind
test seams. Platform modules contain operating-system calls.

| Subsystem | Responsibility |
| --- | --- |
| Input Engine | Focus-scoped interception and synthesis, suspension, menu gating, and recursion protection |
| Game Observer | Installation provider, launcher/game runtime, focus, and normalized Game Context |
| Weave Engine | Skill configuration, cooldown gating, action sequences, and timing |
| Fishing Controller | Event-and-tick fishing state machine |
| Auto Potion Controller | Resource and quickslot eligibility rule |
| Pixel Bus Reader | Layout negotiation, capture, decoding, freshness, and display description |
| Beacon Manager | Safe discovery, installation, verification, update, and removal of PixelBeacon |
| Config and Session State | Separate user settings and derived runtime stores |
| Logging | Structured file and in-memory sinks |
| Interface | Immediate-mode egui application surface |

## Thread model

ESO Weave uses `std::thread`, `Arc<Mutex<...>>`, and `std::sync::mpsc`; it has no
async runtime.

| Thread | Owns | Never does |
| --- | --- | --- |
| Main | Interface, application model, and view | Timed input sequences |
| Interception | Platform hook or evdev loop | Sleep, block, or synthesize |
| Weave worker | Action queue and timed sequences | Touch the interception thread |
| Pixel bus worker | Game observation, capture, event routing, display detection, fishing ticks, and Auto Potion ticks | Sample from another thread |
| API version check | One startup manifest/client-version pass | Delay the first window |

Five ownership contracts are load-bearing:

1. The interception callback never sleeps or blocks.
2. Every timed sequence runs on a worker.
3. Application hotkeys and on-screen controls converge on the same interface
   intent path.
4. Pixel-bus and interface deadlines use one monotonic clock origin.
5. Network version checking runs once in the background and never blocks startup.

Platform and hardware boundaries are represented by traits so engine, controller,
and decoder behavior can be tested with deterministic mocks.
