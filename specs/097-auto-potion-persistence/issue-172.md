# Issue Snapshot: #172

**Title**: Persist requested Auto Potion enablement across restarts

**URL**: https://github.com/h8rt3rmin8r/eso-weave/issues/172

**Captured**: 2026-09-15

## Outcome

Persist the operator's requested Auto Potion enablement and restore it when ESO
Weave restarts.

## Acceptance

- Enabling and disabling the request round-trips across normal restarts.
- Old state files default the request to disabled without losing other facts.
- Startup restoration cannot synthesize input until every existing gate passes.
- UI and keyboard toggles persist one authoritative value.
- Tests cover migration, malformed data, both Boolean values, and safe startup.
- Canonical documentation describes persistence without historical caveats.

The live GitHub issue remains the project-management authority. This snapshot
keeps the spec review self-contained without replacing native issue state.
