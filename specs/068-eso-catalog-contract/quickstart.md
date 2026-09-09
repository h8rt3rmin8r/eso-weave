# Quickstart: Consume the ESO Catalog Source Contract

## Validate the contract

From the repository root:

```powershell
node --test .github/scripts/docs-policy.test.mjs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
```

The policy rejects missing categories, transient keys, invalid enum values,
implicit PTS promotion, missing immutable hashes, and distributable game icon
bytes.

## Select a source

1. Find the category in `docs/project/catalog-sources.json`.
2. Use its stable key and listed enumeration method.
3. Resolve every `source_id` to an immutable SourceSnapshot.
4. Carry the exact version tuple and visibility into the output batch.
5. Preserve the declared completeness class or lower it if collection is more restricted.
6. Apply the category's redistribution decision before packaging output.

## Handle channels

Never combine live and PTS inputs implicitly. A PTS record remains PTS even if
its normalized content later matches live. Create a reviewed promotion receipt
or build a fresh live snapshot.

## Handle icons

Store virtual icon paths as metadata. Ship project-created placeholders. Treat
all game image bytes as local-only and prohibited from release artifacts unless
a later explicit compatible permission changes the contract.

## Handle collector output

Accept only the restricted CollectorEnvelope. Check byte limits before parsing,
validate its exact schema, canonicalize record bytes, verify the content hash,
and commit the import atomically. Never evaluate SavedVariables as Lua.
