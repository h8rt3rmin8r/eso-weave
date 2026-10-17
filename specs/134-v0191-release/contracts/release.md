# Candidate Review and Release Surfaces

The authoritative procedure is [releasing.md](../../../docs/project/releasing.md).

| Surface | Candidate contract |
| --- | --- |
| Application/package version | Cargo.toml 0.19.1, matching root Cargo.lock entry |
| Changelog | Fresh Unreleased, 0.19.1 with common preparation date, valid Highlights and full prior history |
| README | Governed static version badge 0.19.1 |
| Bundled documentation | Applies to v0.19.1 and the same preparation date, regenerated in the release binary |
| Collector fixture | tool_version 0.19.1 |
| Addon contracts | Retain existing independent version declarations |
| PR | codex/s134-v0191-release -> main; Closes #315 only |
| Publication | No v0.19.1 tag/release/assets created in this run; #314 remains open |

The unchanged package matrix is Windows x64 MSI, Linux x86_64 Debian/AppImage/
tarball and SHA256SUMS. Candidate identity permits build/review, not a claim of
publication or installation. Dirty state, invalid notes, identity drift, failed
checks or unresolved findings prevent a ready handoff.
