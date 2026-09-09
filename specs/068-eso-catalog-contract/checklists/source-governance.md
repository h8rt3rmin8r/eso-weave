# Source Governance Checklist: ESO Catalog Source Contract

**Purpose**: Verify that future ingestion choices stay reproducible and honest

- [x] Every required category has one matrix row.
- [x] Every primary technical source has an immutable revision.
- [x] API documentation evidence includes SHA-256 hashes.
- [x] Moving branches are freshness signals, not reproducibility anchors.
- [x] Live and PTS stay separate until a reviewed promotion receipt exists.
- [x] Durable keys are IDs or explicit composites, never iterator positions.
- [x] Completeness is scoped to the source tuple and never implied globally.
- [x] Community code licensing is separate from community data licensing.
- [x] ZeniMax art bytes remain excluded from source and binary distribution.
- [x] Project placeholders remain a valid offline fallback.
- [x] Local asset bytes cannot enter repository, fixture, or release artifacts.
- [x] Collector input is bounded, non-executing, private by default, and atomic.
- [x] Missing game experiments remain unverified and have a separate owner.
