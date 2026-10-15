# S130 Research

Date: 2026-10-15. Baseline: merged S129 at `9a2580d07238822a2952f488856a419cc63a8f14`.

## Status

Phase 0 is complete for owner-approved normal/separate v15; the broader producer-grammar gap is deferred to #296/#297. No implementation code, schema migration, game setting change or installed-game verification has been performed.

## Resolved Integration Decisions

- Extend the existing application-lifecycle maintenance authority, keeping Catalog and native workers independently scheduled and cancellable.
- Read bounded appended prefixes; do not repeatedly stable-import or load lifetime files.
- Preserve schema-1 evidence/identities and add explicit logical identity reconciliation rather than rewriting canonical rows.
- Commit fights and progress atomically, with revision checks and a shared cancellation/commit admission boundary.
- Preserve historical reads, read-only authenticated local databases and source-qualified capabilities independently of collection eligibility.
- Move required calculation/recommendation work before removing all Encounter History presentation, then audit final current text/assets/contracts/generated output.

Alternatives rejected: whole-file reimport, UI-owned ingestion, per-chunk full-history validation, token-check-only commit admission, destructive identity rewriting, and source-unsupported capability parity.

## Evidence and Detailed Inventories

- [Native format research](research-native.md): reviewed producer/help/parser sources, exact supported evidence and unresolved serialization.
- [Runtime and storage research](research-runtime.md): actual lifecycle, transaction, identity and consumer seams.
- [Surface research](research-surfaces.md): complete UI removal seams, policies, contracts, visual and generated-output inventory.
- [Issue acceptance inventory](issue-acceptance.md): all acceptance lines from the five owner-selected issues, still uncompleted.

## Resolved Requirement Boundary

FR-002 and issue #289 now require normal/separate v15 for S130 under direct owner approval. The all-four producer-grammar gap remains relevant evidence for #296/#297, which must establish exact serialization before claiming their profiles. S130 uses reviewed positional grammar and preserves controlled unsupported-profile outcomes.

Owner resolution: S130 proceeds with normal/separate v15 and controlled unsupported-profile handling. Affected GitHub/spec acceptance is updated under explicit authorization; #296/#297 own the other combinations. No unresolved product or technical clarification remains for this supported grammar.
