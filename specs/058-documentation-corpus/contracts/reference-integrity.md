# Reference Integrity Contract

## Live references

Current governance, contributor guidance, source comments, workflow comments,
configuration comments, asset guidance, and published documentation must use the
post-migration path with exact case.

## Historical references

An obsolete path may remain only in a ledger original-path field or an exact
historical exception. Old spec-kit records and archived plans should receive
mechanical path repairs where their links are intended to remain navigable.
Historical changelog prose may retain a then-current literal path when changing
it would falsify the release record.

## Validation

The corpus policy rejects:

- any legacy literal outside the exact allowlist;
- any current Markdown link with a missing or case-mismatched target;
- project/archive paths in `SUMMARY.md`;
- a moved source that still exists;
- an absent destination; and
- a root README above the specified line budget.
