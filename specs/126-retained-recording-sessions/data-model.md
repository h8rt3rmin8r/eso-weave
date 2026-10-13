# Retained Recording Data Model

## Controller envelope

Shared root and catalog versions remain schema1/addon1. Capture payloads remain
schema2/addon3 (and supported legacy payloads retain their original versions).
New recording controllers use state_schema_version2/addon_version5, preserving
current fields and adding:

- retained_sessions: map of 16-digit zero-padded sequence keys to terminal
  v2/controller5 snapshots (legacy v1/controller4 accepted; no nested archive or authority/current capture).
- session_sequence: exact positive integer for the current session, absent when
  no current session exists.
- next_session_sequence: exact positive integer greater than every retained and
  current sequence; persists through discard and clear.

Creation sequence establishes order, not timestamps. IDs are unique within the
whole spool. Every archived snapshot retains original capture bytes and revision.
A current stopped controller may select the next mode/channel independently of
its session; capture authority always uses the session's original provenance.

## Resume transition

Stopped continuous -> explicit warning and admission -> UserResumed marker ->
active/waiting (or partial current fight when enabled mid-combat).
The marker includes occurred_at, prior_finished_at and the prior terminal fight
ordinal. Old markers and terminal records are immutable prefixes. Failed/single/
absent/archived resume requests do not mutate data.

## Limits

32 total sessions; 1024 total terminal/current encounter slots; 100000 events;
100000 raw observations; 1024 interruptions; 33554432 estimated capture bytes.
Terminal/failure reserves remain accounted across the spool. Capacity refusal
precedes rotation, selection changes, counter allocation and authority.

## Parsed set and snapshots

ParsedCaptureSet retains the envelope plus an ordered matching controller list
and terminal records. All environments validate before filtering. One snapshot
per matching session contains only that session's members. New and archived
snapshot revisions preserve duplicate import identity.

## SQLite schema5

Existing raw_encounters and encounter_session_snapshots remain immutable.
An immutable encounter_session_order table assigns a positive unique first-import
ordinal per session. Migrated stores retain their prior deterministic session
ordering; new source sessions are admitted in creation sequence. Deletion of
records preserves order metadata so reimport cannot reorder surviving history.
Migration is transactional and changes no canonical capture or snapshot bytes.
