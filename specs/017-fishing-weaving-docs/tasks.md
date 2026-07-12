# Tasks: Fishing and Weaving README Documentation

**Feature**: 017-fishing-weaving-docs
**Spec**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)

Documentation-only feature; the single file is `README.md`. Tasks are sequential
(one file) and ordered by user story.

## Phase 1: Setup

- [x] T001 Re-read the current `README.md` to confirm the existing section
  structure and the exact Disclaimer and License blocks to move.

## Phase 2: User Story 1 - Fishing usage documentation (Priority: P1)

- [x] T002 [US1] Add a Fishing section to `README.md` after Installation covering
  the interaction model (the fishing hotkey F2 casts; the player does not cast
  first), the PixelBeacon prerequisites (installed, enabled, not out of date or
  out-of-date addons allowed, beacon strip visible, window focused, /reloadui
  after an app refresh), the status progression (Casting, Fishing (waiting for a
  bite), Reeling in, Recasting, Idle with a reason), the interact key (E) and the
  configurable timings (arm timeout, reel delay, recast delay), troubleshooting
  tied to the early-stop symptom, and a short account-risk reminder.

## Phase 3: User Story 2 - Weaving usage documentation (Priority: P1)

- [x] T003 [US2] Add a Weaving section to `README.md` after Fishing covering the
  single-bar overview, the seven skill slots and their defaults, the four weave
  types, the default timings (global cooldown 500 ms, light 50 ms, heavy 1000 ms,
  bash 125 ms), that F1 suspends and resumes and latency adaptation is off by
  default, and a single sentence that multi-bar weaving is not yet finalized and
  is out of scope. Do not document dual-bar mechanics.

## Phase 4: User Story 3 - Disclaimer reorder (Priority: P2)

- [x] T004 [US3] Move the Disclaimer section so it is the next-to-last section,
  immediately before the License, giving the order banner, Installation, Fishing,
  Weaving, Disclaimer, License.

## Phase 5: Polish and Verification

- [x] T005 Verify documented defaults match the code (fishing arm timeout 8000 ms,
  reel 100 ms, recast 3000 ms, interact key E; weave 500/50/1000/125 ms; F1
  suspend, F2 fishing) and that the status labels match slice 016.
- [x] T006 Text hygiene: confirm `README.md` is UTF-8 without BOM with LF endings,
  has no em- or en-dashes, and that all links resolve. Render and confirm the
  section order.

## Dependencies and Order

- T001 first. T002, T003, T004 edit the same file, so run sequentially. T005 and
  T006 last.
