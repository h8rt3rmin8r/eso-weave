# Bounded Discovery Collector

ESO Weave includes a separate, optional `EsoWeaveCollector` addon for collecting
public ESO API results on the user's own system. It is a deliberate maintainer
workflow, not part of normal application startup. It does not capture combat,
send network requests, generate input, modify gameplay, upload data, or package
game-owned image bytes.

The first collector version covers five bounded views: player skills, crafted
abilities, item sets, champion skills, and companions, races, and classes.
Coverage is truthful only for the recorded character, account unlocks, locale,
channel, and API version. Iterator positions are retained only as
version-scoped ordering facts. Stable API IDs remain catalog identity.

## Install and collect

Use the exact Live or PTS `AddOns` directory and the current numeric ESO API
version. These lifecycle commands operate only on `EsoWeaveCollector` and never
share PixelBeacon ownership or files.

```console
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
it never resumes without `/ewcollect resume`. Use `/ewcollect cancel` to
discard the active run. A completed capture still reaches disk only at ESO's
supported SavedVariables save boundary, so run `/reloadui`, log out, or exit
before importing it.

## Stage and compile

The SavedVariables file is `SavedVariables/EsoWeaveCollector.lua` beside the
selected environment's `AddOns` directory. Stage it to normalized JSON first:

```console
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
are checked before or during parsing: 64 MiB input, 500,000 records, 64 KiB
strings and chunks, 1,024 chunks, bounded nesting, tokens, and table entries.
Malformed, partial, oversized, aliased, or unsupported input leaves existing
staged output unchanged.

Localized names and descriptions in staged output remain user-generated local
records. Icon values are reference-only virtual paths. Neither becomes a
redistributable ESO Weave asset.

## Remove

Remove only a managed collector:

```console
cargo run --locked --bin catalog-compiler -- collector-remove --addons ADDONS
```

Removal never deletes SavedVariables, staged JSON, catalogs, or an unmanaged
addon directory. Preserve or delete those user-local files separately according
to the user's own retention choice.
