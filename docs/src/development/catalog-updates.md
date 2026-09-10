# User-Initiated Catalog Updates

ESO Weave can select a verified user catalog without modifying the packaged
catalog. Open **File > Catalog Update...** to review status, imported candidates,
collector assistance, progress, rollback, and cleanup.

No update begins automatically. The bounded startup worker checks Live version
evidence, discovers local candidates, verifies a saved user selection, and falls
back to the bundled catalog without delaying the first window. Live and PTS are
separate identities. A PTS candidate is always a preview and cannot replace the
active Live catalog.

## Reviewed candidate import

Place a complete [S073 review candidate](catalog-candidate-pipeline.md) beneath
the application configuration directory:

```text
catalog/import/live/<candidate-sha256>/
catalog/import/pts/<candidate-sha256>/
```

Use the channel and hash directory names from the candidate receipt. The app
accepts only the exact nine-file candidate contract. It rechecks sizes, hashes,
manifest identity, source policy, schema, SQLite integrity, foreign keys,
semantic checksum, channel, and directory identity before displaying an import
as installable.

A candidate hash proves that the reviewed files stayed intact. It does not
authenticate the person or service that supplied them. The install control
therefore requires an acknowledgement that the candidate came from a review or
release source you trust.

Installation copies regular files to same-filesystem staging, verifies the copy,
publishes it once beneath `catalog/versions/live/<candidate-sha256>/`, opens the
database read-only, then atomically replaces the small canonical selection file.
Cancellation before that final selection boundary leaves the prior selection
unchanged. The selection commit itself is short and non-interruptible.

## Collector-assisted candidate

The optional collector is separately marker-owned and never modifies
PixelBeacon. Before installation, the modal lists the collected categories and
the exclusion or one-way pseudonymization of account and character identity.
The collector uses only public addon API results and writes SavedVariables.

After choosing **Begin capture wait**, complete a save boundary in ESO with
`/reloadui`, logout, or exit. ESO Weave does not claim to read unflushed or
in-memory state. **Build from flushed capture** requires a later, changed, stable,
complete S071 envelope; parses it as restricted data rather than Lua; never
executes or uploads it; and sends it through the S073 pipeline. The active Live
catalog is the zero-removal baseline, so a partial capture cannot silently reduce
accepted coverage. The resulting candidate still requires a separate install
action.

The modal can delete only the regular collector SavedVariables file and can
uninstall only a marker-owned collector directory. Neither action changes
PixelBeacon.

## Recovery, rollback, and receipts

One standard-library file lock serializes installation, collector builds,
rollback, recovery, and shutdown. On startup, only abandoned private
`.update-*` staging directories are removed. Accepted immutable versions are
outside that recovery root.

Every successful selection change attempts a canonical redacted receipt under
`catalog/receipts/<generation>.json`. Receipts contain stable target identities,
hashes, version metadata, source counts, results, and stages. They contain no
absolute paths, capture values, localized records, account or character values,
credentials, or icon bytes. If receipt storage is full, the already safe
selection remains valid and the modal reports the receipt failure.

**Roll back** re-verifies and reopens the previous Live target before changing
selection. A missing, malformed, linked, corrupt, PTS, or incompatible user
selection is never repaired silently; the app visibly uses the bundled fallback.

