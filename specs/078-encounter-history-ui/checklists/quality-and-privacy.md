# Encounter Quality and Privacy Checklist

**Purpose**: Guard truthful local presentation and data ownership
**Created**: 2026-09-10
**Feature**: [spec.md](../spec.md)

## Truthful presentation

- [x] Observed values never claim complete encounter or live-tool parity
- [x] Complete and degraded quality remain visible per metric
- [x] Exact declared loss ranges and reasons remain visible
- [x] Unknown IDs and catalog provenance remain visible
- [x] Unavailable numeric values are not displayed as zero
- [x] Raw summaries survive derived catalog failures

## Local ownership

- [x] Import is explicit and non-executing
- [x] No upload, telemetry, watch, or arbitrary directory scan is introduced
- [x] Raw storage stays separate from settings and catalog data
- [x] No automatic pruning or migration is introduced
- [x] Delete-one and delete-all actions require confirmation
- [x] Failed operations preserve existing data

## Runtime isolation

- [x] Disk and calculation work stays off the render thread
- [x] History work cannot authorize or generate gameplay input
- [x] Catalog access is read-only and raw encounters remain immutable
- [x] Worker and headless UI behavior are independently testable

## Notes

The planned surface consumes existing safe boundaries and adds no new game or
network authority.
