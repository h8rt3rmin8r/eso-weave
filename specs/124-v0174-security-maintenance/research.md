# S124 Research

## Selected dependency updates

| Package | Baseline | Target | Source PR |
| --- | --- | --- | --- |
| rustls | 0.23.41 | 0.23.45 | #257 |
| thiserror | 2.0.20 | 2.0.21 | #258 |
| ureq | 3.4.1 | 3.4.2 | #258 |
| rmcp | 3.4.0 | 3.5.0 | #258 |
| tokio | 1.53.1 | 1.53.2 | #258 |
| tokio-util | 0.7.16 | 0.7.19 | #258 |
| libc | 0.2.189 | 0.2.190 | #258 |
| mlua | 0.12.1 | 0.12.2 | #258 |
| getrandom (direct) | 0.3.4 | 0.4.3 | #239 |
| base64 (direct) | 0.22.1 | 0.23.1 | #240 |

Source heads at kickoff: #257 b8b0d144f74b52cd588862176889c6bf87fbb25a; #258 a102be9fd332efe8618a5569d3dea0f49f2f3878; #239 2908210f3bf28260b2faa49e14fb4e7516183520; #240 4ed1a60afc10b5820429304d9e4d701908e89413; #247 26924f29a6c0da1b5746aee37301739d6f9033ec.

**Decision**: Resolve the exact selected versions on current main instead of cherry-picking entire automated lockfile patches. The rustls PR contains an unrelated tempfile/getrandom edge change, so preserve the current edge unless dependency resolution requires otherwise. Inspect all resolved rustls instances against GHSA-2mjx-qc3c-rqvc (affected >=0.23.13,<0.23.45).
**Rationale**: This removes the demonstrated advisory while keeping every transitive change attributable to the selected scope.
**Alternatives considered**: Whole-lockfile replacements include avoidable unrelated graph churn; skipping 0.x upgrades silently narrows the accepted ten-package bundle.

## Scanner pins

**Decision**: Use official CodeQL Action v4.38.2 commit 2892aa5e19bbd11bc0cff5427e3b750a04d9e3c2 for init and analyze; verify annotated upstream tag resolution and record a dated changelog decision.
**Rationale**: Version comments and complete immutable commit identity stay aligned; triggers and permissions need no change.
**Alternatives considered**: Floating tags discard the existing pin boundary; changing scanner workflow policy is unrelated.

## Planning and release preparation

**Decision**: Archive Plan049 with chronological October 7 evidence: S123 merged PR256 at 4cbbe4b, v0.17.3 publication, and PR260 AppImage pin repair at e65c00c. Add Plan050 for S124 with machine/prose/index consistency and update the managed agent-context reference.
**Rationale**: Existing policy already validates contiguous IDs, a single final active plan, exact destinations and concrete archive table links. Preserve the prior GFM table fix by adding archive rows contiguously.
**Alternatives considered**: Updating only the human index leaves machine authority stale; broad policy changes are unnecessary.

**Decision**: Prepare v0.17.4 through installed cargo-release 1.1.2 and unchanged release.toml after a clean preparatory commit, using the supported non-interactive confirmation flag. Existing configuration disables tag, push and publication.
**Rationale**: One governed command synchronizes the six established identity surfaces. Owner approval covers candidate preparation and PR publication; a later tag remains separate.
**Alternatives considered**: Manual identity rewrites risk drift; immediate publication exceeds the requested final-review boundary.

## Evidence strategy

**Decision**: Reuse credential/cursor, HTTP/MCP authority and hostile Lua import coverage before and after upgrades, then full Rust, release-note, release-profile, documentation and trust gates. Add narrowly relevant regression coverage only when a demonstrated gap or compatibility change needs it.
**Rationale**: Small configuration diffs can affect protocol parsing and exact values. Passing unrelated checks cannot substitute for these existing contracts.
**Alternatives considered**: A mirrored version-assertion unit suite adds little beyond direct inventory evidence; field checks and new verification-only issues conflict with the owner's workflow.
