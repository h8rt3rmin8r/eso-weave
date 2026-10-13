# Recording Retention Contract

## In-game commands

- /ewencounter start live|pts single|continuous: new session when off; preserve
  all prior sessions. When already enabled, report state and require stop before
  a different selection. Wrong arguments never mutate state.
- /ewencounter stop: disable capture and retain data.
- /ewencounter resume: continue only current stopped continuous session with its
  original channel/mode. Repeated active resume is idempotent.
- /ewencounter sessions: bounded chronological list of session IDs, mode,
  environment, lifecycle and terminal/partial counts; no raw callback values.
- /ewencounter discard SESSION_ID confirm: remove exactly that inactive session
  while recording is off. Missing confirmation, unknown ID or active recording
  leaves all data unchanged. Save deletion using reload/logout/exit.
- /ewencounter clear confirm: clear all addon recording evidence, preserving
  selections, identity counter, catalog state and desktop history.
- Existing mode/channel/toggle/arm/disarm/status/help remain available with
  updated explanations. Invalid/unknown evidence remains preserved and inactive.

## Desktop

Inspect the selected last-saved source through the existing serialized worker.
Show selected-environment terminal counts, matching per-session summaries and
global retained capacity. Explain saved-source lag and that start needs no clear.
Keep import, refresh and history operations distinct. Import validates all
environments, filters only afterward, preflights every session, and commits one
atomic transaction; no source mutation or inferred import acknowledgment.

## Compatibility and diagnostics

Supported legacy terminal and v1 controller data migrate without terminal edits.
Malformed/nested/newer envelopes reject generically without raw data disclosure.
All stored complete/partial/loss distinctions stay unchanged. Native log import
and a live game-state channel are outside this contract.
