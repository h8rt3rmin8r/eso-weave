# Data Model: Bundled Offline Documentation

## EmbeddedAsset

- `path`: normalized UTF-8 path relative to `/eso-weave/`, sorted and unique.
- `content_type`: static media type derived from the extension at build time.
- `bytes`: immutable compile-time byte slice.

The production manifest contains every regular generated file. No path is absolute, empty, contains a backslash, or contains a dot segment. Duplicate normalized paths fail the build.

## DocumentationManifest

- Sorted slice of `EmbeddedAsset`.
- Root key `index.html` is mandatory.
- Generated local references resolve to a manifest entry.
- Lookup is exact and performs no filesystem operation.

## DocumentationService

- Loopback socket address.
- Stable root URL ending in `/eso-weave/`.
- Shutdown sender.
- Join handle for one listener worker.

States are `Stopped`, `Running`, and `Stopping`. First open starts or returns an error. Later opens reuse. Drop joins within the bounded shutdown interval.

## RequestTarget

- Raw target capped by request size.
- Optional bounded query removed before validation.
- `/` redirects; `/eso-weave/` maps to `index.html`; mounted trailing slash appends `index.html`.
- Percent signs, backslashes, controls, dot segments, and mount escape are invalid.

## BrowserOpenResult

- `Opened`: the operating system accepted the stable URL.
- `Unavailable`: no opener is available.
- `Rejected`: the launcher returned an error.

No browser result changes service authority or documentation content.
