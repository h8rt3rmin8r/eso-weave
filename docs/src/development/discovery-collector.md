# Bounded Discovery Collector

The [catalog evidence lifecycle](catalog-candidate-pipeline.md#evidence-lifecycle-and-runtime-selection)
places this collector's local-only evidence alongside pinned source snapshots
without treating collection or candidate review as runtime selection.

ESO Weave includes an optional catalog module in the marker-managed
`EsoWeaveData` addon for collecting
public ESO API results on the user's own system. It is a deliberate maintainer
workflow, not part of normal application startup. It does not capture combat,
send network requests, generate input, modify gameplay, upload data, or package
game-owned image bytes.

**Catalog collection** gathers game-data definitions for a later reviewed catalog
build. It does not record fights. For player encounter recording, use
[Encounter Capture](../features/encounter-capture.md). PixelBeacon is the other
addon and supplies current screen readings; it does not collect catalog data.

The first collector version covers five bounded views: player skills, crafted
abilities, item sets, champion skills, and companions, races, and classes.
Coverage is truthful only for the recorded character, account unlocks, locale,
channel, and API version. Iterator positions are retained only as
version-scoped ordering facts. Stable API IDs remain catalog identity.

## Manage and collect

Normal users manage the exact four-file `EsoWeaveData` package from the
first-class **ESO Weave Data** row in System and State. It never shares
PixelBeacon ownership or files. Maintainers can inspect or install against an
exact Live or PTS `AddOns` directory with the CLI:

```bash
cargo run --locked --bin catalog-compiler -- collector-status --addons ADDONS
cargo run --locked --bin catalog-compiler -- collector-install --addons ADDONS --api-version 101050
```

An existing target without the collector's managed marker is reported as
`unmanaged` and is never changed. Linked, reparse, and non-regular targets are
also refused. If ESO is running or its state is uncertain, the result sets
`reload_required` so the user knows to run `/reloadui` or relog.

In ESO, start an explicit bounded run:

```text
/ewcollect start live
/ewcollect status
```

The addon performs limited work per update tick. Entering combat pauses the run;
leave combat and run `/ewcollect resume` to continue. Use `/ewcollect status`
inside ESO for the current run. `/ewcollect cancel` stops the run and retains an
incomplete snapshot, which cannot be used for a catalog build. It does not erase
the collected data. Starting a new run after a completed, cancelled or failed
run replaces that previous catalog collection, while preserving encounter data.

Run `/ewcollect start pts` for the Public Test Server, and import with the
matching channel. A reload does not reconstruct an active collection runtime;
check status and begin a fresh explicit run if it was interrupted. A completed
collection still reaches disk only when ESO saves its addon data: run
`/reloadui`, log out, or exit before importing it. Save or export any completed
collection you want to retain before starting another.

## Stage and compile

The SavedVariables file is `SavedVariables/EsoWeaveData.lua` beside the
selected environment's `AddOns` directory. Stage it to normalized JSON first:

```bash
cargo run --locked --bin catalog-compiler -- import-collector --input SAVED.lua --output STAGED.json --channel live --catalog-version VERSION
```

Import is intentionally separate from SQLite publication. Review the redacted
receipt and staged coverage, then use the ordinary
[catalog compiler](catalog-compiler.md) build command. Live and PTS inputs remain
separate and a channel mismatch fails before output changes.

The importer never evaluates Lua. Its receipt hashes the pseudonymous scope key
instead of printing it. It accepts one fixed SavedVariables table
grammar, requires a complete envelope, verifies the embedded collector SHA-256,
contiguous JSON-line chunks, Adler-32 metadata, category identities, and parent
relationships, then validates the result through the S070 catalog model. Limits
are checked before or during parsing: a 128 MiB shared-file envelope, 500,000
catalog records, 64 KiB
strings and chunks, 1,024 chunks, bounded nesting, tokens, and table entries.
Malformed, partial, oversized, aliased, or unsupported input leaves existing
staged output unchanged.

Localized names and descriptions in staged output remain user-generated local
records. Icon values are reference-only virtual paths. Neither becomes a
redistributable ESO Weave asset.

## Remove

Normal users choose **Uninstall Data** in System and State and accept the
data-specific confirmation. Maintainers can remove only a managed package with:

```bash
cargo run --locked --bin catalog-compiler -- collector-remove --addons ADDONS
```

Removal never deletes SavedVariables, staged JSON, catalogs, or an unmanaged
addon directory. Use `/ewcollect clear confirm` in ESO to clear only the catalog
subtree while preserving encounter data.
Clear also preserves imported desktop encounter history, both addon packages
and the active catalog. Save in ESO to write the cleared state to disk. An
unsupported saved-data version remains preserved and cannot be cleared by the
current module; update to a compatible release instead of editing its version.
