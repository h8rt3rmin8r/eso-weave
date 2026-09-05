# Build Plans

This directory holds ESO Weave build plans. A build plan decomposes the master
specification (`docs/ESO-Weave-Specification.md`) into an ordered set of
work slices, each scoped to become one spec-kit feature under `specs/NNN-name/`.

Two documents share the word "plan"; they are distinct:

- A build plan (`docs/plans/plan-NNN.md`) is the higher level slice roadmap. It
  says what to build next and in what order, and it is what repository
  references point at for feature sequencing.
- A spec-kit feature plan (`specs/NNN-name/plan.md`) is generated per feature by
  `/speckit.plan`. It is the implementation plan for a single slice.

Repository references target this directory and its index, not any single plan
file. To add a new plan, drop `plan-002.md` (and so on) into this directory and
add one row to the table below. No other files need to change.

## Index

| Plan | Status | Scope |
| --- | --- | --- |
| [plan-001.md](plan-001.md) | Active | Initial decomposition of the master specification into ten build slices, from foundations through packaging, preceded by the constitution prerequisite. |
| [plan-002.md](plan-002.md) | Active | Brand and UX polish (slice 012): a documented brand standard applied across the app UI, the runtime and executable icon, and the Windows and Linux installers. |
| [plan-003.md](plan-003.md) | Active | GUI overhaul and weapon-bar-aware timing: a GUI ergonomics, information-design, and auto-save slice (013), and a weapon-bar-aware adaptive-timing slice (014) that also closes research item R1. |
| [plan-004.md](plan-004.md) | Active | Fishing reliability and usage documentation: a fishing reliability and status-collaboration slice (016) that fixes the arm-to-Idle defect and refreshes the addon API version (closing R4), and a documentation slice (017) adding fishing and weaving README sections. |
| [plan-005.md](plan-005.md) | Active | API version automation, UI fixes, and specification rewrite: an ESO API version check automation slice (018) that keeps the addon manifest current, a GUI slice (019) fixing hover reflow and the settings modal, and a documentation slice (020) rewriting the master specification to v0.2.0 and correcting the fishing README bait step. |
| [plan-006.md](plan-006.md) | Active | Window persistence, UI defect cleanup, and fishing diagnosis: a window geometry persistence slice (021), a primary and skills controls slice (022) adding an addon Update button and fixing alignment, dropdown width, and the delay column, a settings modal and logging slice (023) fixing modal scaling, the success toast, log-level linkage, and keybinding presentation including the missing F2, and a fishing slice (024) hardening the pixel-bus capture read and its diagnostics. |
| [plan-007.md](plan-007.md) | Active | Fishing interaction detection rewrite (slice 025): replace PixelBeacon's one-shot `EVENT_CLIENT_INTERACT_RESULT` fishing detection (an error-alert channel that never fires on a successful cast, root cause of the fourth field failure) with poll-authoritative detection mirroring the game's own reticle, add fishing-controller transition logging, and update the specification's detection contract. |
| [plan-008.md](plan-008.md) | Active | Fishing bite signal correction (slice 026): remove slice 025's prompt-comparison bite trigger (the reel-in prompt is the standing cast prompt, not a bite indicator, so every cast insta-reeled and burned bait) and make the lure-scoped bait-consumption inventory event the sole bite signal, matching both proven references; PixelBeacon advances to version 5. |
| [plan-009.md](plan-009.md) | Active | Application interface sizing correctness (slice 030): close the three open v0.8.0 interface reports (#12 ratcheted window shrink, #13 log pane overlapping the Skills controls, #14 settings modal locked under half its content) by rebuilding the enforced content extent on an intrinsic, window-independent basis, hard-enforcing the log boundary and the modal extent, and making the previously untested `src/app/ui.rs` sizing glue verifiable with headless rendered-frame tests. |
| [plan-010.md](plan-010.md) | Active | PixelBeacon player-state expansion: the five open tracker issues sequenced one slice at a time, since each new block claims the next index and widens the strip. Slice 031 adds the in-combat block (#9) and factors the block-extension pattern out of its literals; slice 032 adds the menu-state gate and wires it into input suppression (#10); slice 033 adds the Health, Stamina, and Magicka blocks (#2); slice 034 adds out-of-band display detection (#3); the movement-state block (#11) is last and remains gated on verifying the sprint observable, taking slice 036 after the grid wrap claimed 035. |
| [plan-011.md](plan-011.md) | Active | Pixel bus grid wrap (slice 035): wrap the beacon squares from an ever-widening strip into a 2D grid on both sides of the contract, using a fixed shared column count rather than one derived from the live client width, and validate the grid's extent against the display descriptor slice 034 produced. Lands while the block count is nine, where the wrapped layout is identical to the strip it replaces (#16). |
| [plan-012.md](plan-012.md) | Active | Skill cooldowns, quickslot state, and auto-potion: slice 037 adds six cooldown blocks and takes the grid to exactly its single-row maximum (#18), slice 038 adds the quickslot blocks and crosses onto a second row (#19), and slice 039 builds auto-potion, the first consumer that acts on a beacon-derived value and therefore the first to synthesize input from one (#20). The consumer is isolated behind the observables because its correctness depends on those readings being proven first. |
| [plan-013.md](plan-013.md) | Complete | Runtime and context truth (slice 041): detect installation provider and launcher/game lifecycle (#22), separate focus, PixelBeacon freshness, and valid in-game surface evidence, replace Game Menu with Game Context, and make game-derived values dormant while ESO is inactive (#23). |
| [plan-014.md](plan-014.md) | Active | Delivery pipeline governance (slice 044): audit historical milestone and verification metadata (#45), enforce atomic issue scope and pull-request closing linkage (#46), and create a minimal milestone and Stage oriented GitHub Project (#47). |
| [plan-015.md](plan-015.md) | Active | Negotiated pixel geometry (slice 045): publish PixelBeacon's live physical-width capacity in a versioned invariant header, derive reader capture and payload points from that one validated authority, retain heartbeat-gated legacy reads, and keep release validation separate (#42 and #43; #44 remains verification). |
| [plan-016.md](plan-016.md) | Active | Responsive Live HUD dashboard (slice 046): group live game observations separately from system and automation readiness, render truthful accessible resource meters, and preserve Skills and sizing safety across one responsive breakpoint (#28 and #29). |
