# Encounter Import Safety Checklist

**Purpose**: Guard the hostile-data, privacy, immutability, and ownership boundary  
**Created**: 2026-09-10  
**Feature**: [spec.md](../spec.md)

## Hostile Input

- [x] Input is never executed
- [x] Grammar, root, byte, string, depth, token, and entry limits are explicit
- [x] Stable no-follow read and path-race behavior are explicit
- [x] Unknown fields and payload shapes are rejected rather than dropped
- [x] Counts, sequence, time, terminal, and loss invariants are recomputed
- [x] Truthful partial and corrupt or truncated inputs are distinguished

## Privacy and Provenance

- [x] Live or PTS channel is required and preserved
- [x] Unknown numeric identifiers are retained
- [x] Names, chat, guild, location, notes, and telemetry fields are forbidden
- [x] String payloads use finite contract token sets
- [x] Receipts never include raw payloads or paths

## Store Integrity

- [x] Raw store is separate from configuration, catalog, and derived metrics
- [x] Canonical bytes and content hash are deterministic
- [x] Raw updates are absent from the API and blocked in the database
- [x] Duplicate and identity-collision behavior is explicit
- [x] Import and deletion are transactional
- [x] Corrupt and unsupported stores remain in place

## User Ownership

- [x] Import is explicit with no scan, watch, upload, or telemetry
- [x] Listing is deterministic and payload-independent
- [x] Backup is a consistent atomically published snapshot with a final hash
- [x] Single and all deletion require explicit distinct requests
- [x] No automatic expiry or pruning is allowed
- [x] Future migration requires backup and transactional ownership

## Scope

- [x] Metrics, summaries, recommendations, UI, discovery, gzip, and enrichment are excluded
- [x] Existing collector parsing behavior receives regression coverage
- [x] Full Rust and repository gates are required

## Notes

- Safety review passed before implementation on 2026-09-10.
