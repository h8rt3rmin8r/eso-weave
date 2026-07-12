# Feature Specification: Fishing and Weaving README Documentation

**Feature Branch**: `017-fishing-weaving-docs`

**Created**: 2026-07-12

**Status**: Draft

**Input**: User description: "Two detailed README sections, one for fishing and one for weaving, plus moving the Disclaimer down to be the next-to-last section before the License. Weaving covers default timings and an overview only; dual-bar mechanics are on hold and out of scope."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Learn to fish correctly from the README (Priority: P1)

A player who has installed the app opens the README to learn how to use the
fishing feature. They learn the exact interaction model (press the fishing hotkey
while aimed at a fishing hole; the app casts for them and they do not cast first),
what must be true for it to work (the companion addon installed and enabled and
not out of date, the beacon strip visible, the game window focused), what the
status will show as it runs, and what to check if it stops early.

**Why this priority**: The feature was field-tested and failed partly because the
correct interaction model and prerequisites were undocumented. This section is the
collaboration the feedback asked for.

**Independent Test**: A reader unfamiliar with the app can follow the Fishing
section end to end and correctly operate fishing without external help, and can
self-diagnose the reported early-stop symptom.

**Acceptance Scenarios**:

1. **Given** the README, **When** a reader reads the Fishing section, **Then**
   they can state that the hotkey casts for them and that they do not cast first.
2. **Given** the README, **When** a reader's fishing stops early, **Then** the
   troubleshooting guidance leads them to check the addon is enabled and not out
   of date, the beacon is visible, and the window is focused.

---

### User Story 2 - Understand weaving basics and default timings (Priority: P1)

A player opens the README to understand what the weaving feature does and the
timings it uses by default, without needing to read the code.

**Why this priority**: Weaving is the namesake feature and had no user
documentation. Default timings are the most-asked question.

**Independent Test**: A reader can describe, from the Weaving section, what
weaving does at a high level and can state the default global cooldown and the
light, heavy, and bash delays.

**Acceptance Scenarios**:

1. **Given** the README, **When** a reader reads the Weaving section, **Then**
   they can state the four default timings and that F1 suspends and resumes.
2. **Given** the README, **When** a reader looks for dual-bar behavior, **Then**
   they find a note that multi-bar weaving is not yet finalized and is out of
   scope, and no dual-bar mechanics are documented.

---

### User Story 3 - Disclaimer repositioned (Priority: P2)

A reader scanning the README finds the Disclaimer as the next-to-last section,
immediately before the License, rather than at the top.

**Why this priority**: A requested structural change so the usage content leads
and the legal content sits at the end with the license.

**Independent Test**: The rendered README section order is banner, Installation,
Fishing, Weaving, Disclaimer, License.

**Acceptance Scenarios**:

1. **Given** the README, **When** its sections are listed top to bottom, **Then**
   the Disclaimer is the second-to-last section and the License is last.

---

### Edge Cases

- The reader runs the game in a mode where the beacon strip is off screen: the
  Fishing prerequisites make the visible-beacon requirement explicit.
- The reader already fished manually and expects to cast first: the interaction
  model explicitly corrects this.
- A future reader looks for dual-bar weaving: the Weaving section states it is out
  of scope rather than omitting it silently.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The README MUST contain a Fishing section documenting the
  interaction model (the fishing hotkey casts the line; the player does not cast
  first), the prerequisites (the companion addon installed and enabled and not
  flagged out of date, the beacon strip visible and unobstructed, the game window
  focused), the status progression the player will see, the interact key and the
  configurable timings, troubleshooting tied to the early-stop symptom, and a
  short account-risk reminder.
- **FR-002**: The README MUST contain a Weaving section documenting a single-bar
  overview of how weaving works, the skill slots and their defaults, the weave
  types, the default timings (global cooldown, light, heavy, bash), that the
  suspend hotkey pauses and resumes the engine, and that latency adaptation is off
  by default.
- **FR-003**: The Weaving section MUST NOT document dual-bar or multi-bar weaving
  mechanics; it MUST note only that multi-bar weaving is not yet finalized and is
  out of scope.
- **FR-004**: The README section order MUST place the Disclaimer as the
  next-to-last section, immediately before the License, with the usage sections
  (Fishing and Weaving) after Installation.
- **FR-005**: The documentation MUST be accurate to the shipped behavior: the
  default fishing and weaving values and hotkeys stated MUST match the code.
- **FR-006**: All README text MUST obey the project text hygiene rules (UTF-8
  without BOM, LF line endings, no em- or en-dashes) and all links MUST resolve.

### Key Entities

- **README section order**: banner and badges, Installation, Fishing, Weaving,
  Disclaimer, License.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A reader unfamiliar with the app can operate fishing correctly using
  only the Fishing section, including knowing not to cast manually first.
- **SC-002**: A reader can state the four default weave timings and the suspend
  and fishing hotkeys from the README.
- **SC-003**: The rendered README lists the Disclaimer as the second-to-last
  section and the License as the last.
- **SC-004**: No dual-bar weaving mechanics appear in the README, only the
  out-of-scope note.

## Assumptions

- The default values documented are those shipped after slice 016: fishing arm
  timeout 8000 ms, reel delay 100 ms, recast delay 3000 ms, interact key E; weave
  global cooldown 500 ms, light 50 ms, heavy 1000 ms, bash 125 ms; F1 suspends and
  resumes, F2 toggles fishing.
- The Fishing status labels documented match slice 016 (Casting, Fishing (waiting
  for a bite), Reeling in, Recasting, Idle with an optional reason).
- This is a documentation-only change with no source or test impact.
