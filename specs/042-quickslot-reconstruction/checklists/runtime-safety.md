# Runtime Safety Checklist: Quickslot Observation Reconstruction

**Purpose**: Guard the future automation boundary while rebuilding its observation

**Created**: 2026-09-03

**Feature**: [spec.md](../spec.md)

## Classification

- [x] Potion is an explicit positive discriminant
- [x] Cooldown and identity cannot create a potion classification
- [x] Empty and each non-potion family remain non-actionable
- [x] Zero-count and unusable potions remain non-actionable

## Integrity and Freshness

- [x] Missing discriminant means legacy or unreadable, never Potion
- [x] Marker, checksum, and unknown-code failures clear the positive state
- [x] Partial identity cannot create or destroy a proven classification
- [x] Signal loss clears the whole observation

## Behavior Boundary

- [x] S042 adds no new synthesized input path
- [x] Existing auto-potion enablement semantics remain unchanged
- [x] Issue #25 must explicitly adopt the new positive and usability checks
- [x] Real-client field evidence is required before issue #24 closes
