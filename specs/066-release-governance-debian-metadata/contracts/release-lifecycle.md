# Contract: Artifact-Dependent Release Verification

1. Candidate preparation and all automated pre-publication gates complete first.
2. A human explicitly authorizes the release command and tag publication.
3. The tag workflow verifies, builds, validates, checksums, and publishes assets.
4. Publication creates the immutable evidence subject but proves no installed,
   UI, field, or target-environment behavior by itself.
5. A separately scoped issue in Release verification names the released asset
   and records the required artifact-dependent evidence.
6. Passing evidence closes the verification issue.
7. A mismatch creates linked implementation work and the verification issue
   remains open until a fixed release passes.
