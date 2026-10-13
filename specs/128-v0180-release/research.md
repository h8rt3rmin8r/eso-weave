# S128 Research

## Governed rollover

**Decision**: Use installed cargo-release 1.1.2 and unchanged release.toml on a clean preparation commit; inspect `cargo release 0.18.0`, then execute with `--execute --no-confirm` through a hidden, noninteractive launcher.

**Rationale**: docs/project/releasing.md is the release authority. The branch matches the configured codex/*-release restriction; tag, push and publish remain false. The six-file rollover synchronizes Cargo package/lock, changelog, badge, bundled snapshot and collector fixture. Installed version was queried successfully. The target remote tag is absent; no matching issue existed before #277. Local/hosted target identity is checked again before execution.

**Alternatives considered**: Hand-editing package, badge and snapshot metadata duplicates governed replacement logic. A new pipeline or dependency update adds no candidate value. Older release sequencing is superseded by the current procedure.

## Archive lifecycle

**Decision**: Move Plan053 to the archive after its actual owner merge and create active preparation-only Plan054; update both indexes and machine/prose ledger.

**Rationale**: Protected main at 05c97041b9d12f5e5c10fae14ba1b80511c5974d is the verified PR #276 squash merge and has the same tree as final reviewed S127. Existing generic documentation policy validates contiguous plan IDs, one active final entry, shaped evidence, destinations and index consistency. Existing spec references to the former current path must be repaired. Archive table rows stay contiguous for GitHub rendering.

**Alternatives considered**: Editing only prose leaves machine authority stale. Leaving the merged plan Active misdirects future work. Removing historical evidence is unnecessary.

## Release notes and capability truth

**Decision**: Three compact highlights cover repaired recording guidance/chat, retained recording sessions and completed native normal/separate version15 import. Preserve the full Unreleased S125-S127 records.

**Rationale**: The existing preview rejects missing Highlights with exit 4 (target/s128-notes-red.log). Native files require deliberate Live/PTS provenance and do not establish other profiles or installed/platform parity. Notes describe completed-file import rather than live tailing.

**Alternatives considered**: Publishing the long engineering chronology hides downloads. Claiming general native parity exceeds demonstrated support. Rewriting historical change records is unnecessary.

## Validation and issue lifecycle

**Decision**: Reuse release-note/Debian fixtures, docs/render/brand/trust policy suites, spelling, mdBook examples/build/link checks, complete headless render, Rust format/Clippy/locked tests and bundled release build. Verify final-head CI and every review disposition, within two rounds.

**Rationale**: The candidate changes identity and planning, with no runtime code. Existing gates exercise shipped source and packaging contracts. #277 is specifically candidate preparation and closes on owner merge, as prior S123/#255 did; the procedure's generic publication tracker rule does not turn this issue into an artifact-publication request. Later tag/publication still requires separate authority. No owner field checks are requested.

**Alternatives considered**: New tests mirroring metadata duplicate established gates. A publication or field tracker would broaden the accepted preparation scope. No unresolved clarification remains.

## Research coordination

The spec-kit Phase 0 research agent independently inspected the existing rollover and gate contracts read-only. Its findings confirm the six identity files and clean-commit prerequisite; detailed commands are captured in quickstart.md.
