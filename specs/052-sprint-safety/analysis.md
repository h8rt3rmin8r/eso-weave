# Cross-Artifact Analysis

## Pre-Implementation

Status: PASS

- Specification, research, model, contract, plan, tasks, and checklists agree on keyboard-mode on-foot scope.
- All artifacts retain protocol v4, 25 blocks, B9 marker `0x43`, existing base codes, sprint code `0xA0`, and reserved mounted code `0xE0`.
- Detector timing is consistently 200 ms entry, 200 ms ambiguous exit, and 1,500 ms stale-positive expiry.
- Auto-potion blocks only explicit `Sprinting` and stores no queued attempt.
- Every acceptance requirement maps to an implementation, verification, or delivery task.
- No unresolved clarification remains.

## Post-Implementation

Status: PASS

- Addon version 19 publishes only the supported on-foot sprint code and retains
  protocol v4, 25 blocks, and reserved mounted code behavior.
- Decoder, routing, view model, auto-potion rule, controller, and lifecycle reset
  all consume one typed `MovementSignal::Sprinting` path.
- Explicit sprint blocks before resource and quickslot evaluation; stronger game,
  focus, beacon, suspension, menu, life, world, and travel blockers keep priority.
- Leaving sprint queues nothing. The next ordinary tick evaluates current readings,
  and Unknown movement remains non-blocking for valid unsupported modes.
- Focused and full-suite tests cover wire tolerance, unsupported mounted code,
  required detector evidence and exclusions, addon version agreement, routing,
  signal loss, blocker text, no-output suppression, recovery, and stronger gates.
- `cargo fmt --all -- --check`, strict all-target clippy, and
  `cargo test --all --locked` pass.
- Diff, UTF-8 without BOM, mojibake, punctuation, and cross-artifact checks pass.
