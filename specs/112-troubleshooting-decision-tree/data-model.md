# Data Model: Accessible Troubleshooting Decision Tree

## Decision Tree

| Field | Contract |
| --- | --- |
| ID | `S112-D01` |
| Asset | `troubleshooting-decision-tree.svg` |
| Destination | `getting-started/troubleshooting.md` |
| Direction | Top-down |
| Intrinsic geometry | 400 by 1520 SVG units |
| Root | First failing observation |
| Decisions | Five ordered symptom-family questions |
| Evidence endpoints | Five family-specific endpoints plus one unmatched continuation |
| Edges | One root edge and ten labelled branch edges |
| Text equivalent | Existing shared diagnostic tree and symptom sections |

## Nodes

| ID | Stage | Role | Visible meaning |
| --- | ---: | --- | --- |
| `first-failure` | 1 | Root | Start with the first failing observation |
| `launch-question` | 2 | Decision | ESO Weave does not open? |
| `startup-evidence` | 3 | Endpoint | Notification before GUI or Live Log after GUI |
| `game-question` | 3 | Decision | ESO detection, activity, focus, or gameplay context fails? |
| `game-evidence` | 4 | Endpoint | Installation, runtime, focus, and game-context evidence |
| `pixelbeacon-question` | 4 | Decision | PixelBeacon lifecycle or signal fails? |
| `pixelbeacon-evidence` | 5 | Endpoint | Managed status, overlay geometry, heartbeat, and freshness |
| `input-question` | 5 | Decision | Input platform or binding evidence fails? |
| `input-evidence` | 6 | Endpoint | Platform, focus, device, and ESO binding evidence |
| `encounter-question` | 6 | Decision | Encounter capture or import is the first failure? |
| `encounter-evidence` | 7 | Endpoint | Addon status, saved authority, receipt, loss, and validation |
| `feature-evidence` | 7 | Endpoint | Feature-specific status and Live Log |

## Edges

Each edge has `data-edge`, `data-from`, and `data-to`. Every choice edge also has `data-branch="true"` and one matching `data-edge-label`.

| Edge | From | To | Label |
| --- | --- | --- | --- |
| `root-to-launch` | `first-failure` | `launch-question` | None |
| `launch-match` | `launch-question` | `startup-evidence` | Does not open |
| `launch-next` | `launch-question` | `game-question` | Opens |
| `game-match` | `game-question` | `game-evidence` | Unavailable |
| `game-next` | `game-question` | `pixelbeacon-question` | Available |
| `pixelbeacon-match` | `pixelbeacon-question` | `pixelbeacon-evidence` | Not current |
| `pixelbeacon-next` | `pixelbeacon-question` | `input-question` | Healthy |
| `input-match` | `input-question` | `input-evidence` | Unavailable |
| `input-next` | `input-question` | `encounter-question` | Valid |
| `encounter-match` | `encounter-question` | `encounter-evidence` | Fails |
| `encounter-next` | `encounter-question` | `feature-evidence` | Different symptom |

## Invariants

- Every node ID is unique and every edge endpoint resolves to one node.
- Every visible connector is annotated and every branch edge has exactly one visible label.
- The five match edges terminate at the corresponding evidence endpoints.
- The five next edges advance in order and never skip a family.
- The final next edge terminates at the unmatched continuation.
- No route crosses another route, shares a segment, or enters an unrelated node clearance zone.
- Color supplements visible wording, card position, and arrow direction.
- The SVG title, description, meaningful Markdown alternative, and adjacent prose describe the same bounded routing task.
