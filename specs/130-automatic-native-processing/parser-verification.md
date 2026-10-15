# S130 parser verification

Date: 2026-10-15. Work: T005-T008. Scope: normal/separate native log version 15;
normal/inline and verbose support remains deferred to #296/#297.

## Implemented behavior

- Pure IncrementalNativeParser accepts at most 256 KiB, consumes a prefix and
  returns a completed fight immediately at its validated END_COMBAT. At most
  one fight is returned per call. Empty feeds are idempotent and never EOF.
- Exact bytes wait for LF before UTF-8 interpretation, including split BOM,
  UTF-8, CRLF, quoted escapes and nested groups. Active run/context/pending
  combat survives arbitrary chunking and checkpoint restart.
- Native schema 2 reuses the original models, canonical encoder and projector.
  Physical u64 provenance is retained without the old lifetime coordinate
  limits. Schema1 validation/canonical encoding remains unchanged.
- Typed-header/run-local fight keys and normalized evidence fingerprints let
  the store reconcile copies/extracted runs with old rows without replacing
  their original IDs, canonical bytes or hashes. Divergent evidence retains
  the same candidate key with a different fingerprint for controlled collision.
- Current context, line, tokens, nesting, pending fight and encoded checkpoint
  remain bounded. Unknown valid types retain evidence; malformed/unsupported
  segments recover only at a fresh validated supported BEGIN_LOG.
- Checkpoints use controlled errors, version/integrity validation, a JSON
  shape pre-scan before decoded allocation and bounded sequence visitors.
  Restore rebuilds metadata and pending-combat caches from retained records.
  Source identity/generation/witness association remains caller-owned.

## Test-first evidence

Parser partition/restart/identity/resource tests were authored before the
implementation. Initial aggregate compilation was temporarily blocked by
unrelated in-flight interface edits; this was not counted as a parser failure.

A subsequent focused red run executed 14 tests: 11 passed and 3 failed. The
failures demonstrated maximum-length CRLF split handling, incomplete-fight
accounting after an invalid closing boundary, and syntactically valid checkpoint
corruption. Each failed assertion was retained and passed after its fix.
Red evidence: `target/s129-s130-parser-boundary-red2.log`.

Final focused command:

```text
cargo test --locked --test encounter_incremental --test encounter_native
```

Result: **45 passed, zero failed, zero ignored** (22 incremental and 23 retained
native tests). Log: `target/s129-s130-parser-final-focused.log`.

Coverage includes every byte partition and checkpoint after each byte of the
sanitized synthetic recording; representative S127 fixture chunks of 1, 2, 7,
31, 256 and 4096 bytes; immediate completion without END_LOG; empty-feed
idempotence; unknown evidence; malformed UTF-8/depth/boundaries; supported recovery;
corrupt/version/channel/state checkpoint rejection; explicit generation
termination; BOM provenance; copy/extracted-run aliases; distinct typed run
headers; divergent-evidence fingerprints; exact supported quantities; and the
reviewed normal/separate scribed ABILITY_INFO extension for schema2 only.

The actual long-stream regression feeds 2,048,000 metadata updates in bounded
chunks, crosses both 256 MiB and 2 million records, keeps the resulting checkpoint
under 4 KiB, then emits a valid fight beyond both old coordinate ceilings.
Another test crosses 10000 recording runs. Context/pending-fight limit tests
demonstrate controlled recovery without fabricated complete fights.

## Schema1 baseline check

The frozen schema1 tiny-fixture canonical SHA-256 is:

```text
7ac4836f32eb7441dbab6676a7d4768bb46ebe181d8b89836070b0b83ba88e92
```

It was independently reproduced using the pre-existing 2026-10-13 S127 compiled
library artifact, linked through a hidden temporary harness. That artifact
predates S130 parser edits. Baseline logs are
`target/s129-s130-parser-baseline-compile2.log` and
`target/s129-s130-parser-baseline-run.log`. The generated synthetic canonical
file's SHA-256 matches the current retained regression exactly. No private owner
recording was used or disclosed by this comparison.

## Final hygiene and limits

Owned Rust files were formatted with rustfmt. Scoped `git diff --check` passed.
The primary agent owns final repository-wide formatting, Clippy, locked full
tests and delivery gates after integration. This receipt does not claim those
gates, installed-game behavior, producer flush cadence, platform field evidence
or deferred format fidelity. No commit, push, PR, release or merge was performed
by this parser work unit.
