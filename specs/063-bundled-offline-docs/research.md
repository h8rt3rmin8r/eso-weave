# Research: Bundled Offline Documentation

## Decision 1: Generate inside production Cargo builds

**Decision**: In the release profile, `build.rs` verifies exact `mdbook 0.5.4` and `mdbook-linkcheck2 0.13.0`, runs the existing book, and generates the embedded manifest. Debug and test profiles generate the same manifest shape from a checked fixture.

**Rationale**: A release-profile Cargo invocation must prove documentation presence. Consuming an optional prebuilt directory could package stale or missing help. A profile split keeps normal compilation practical while making production strict.

**Alternatives considered**: Committed HTML drifts from canonical source. CI-only generation leaves direct release builds unsafe. Requiring mdBook for every test burdens ordinary Rust work.

## Decision 2: Generate Rust manifest source without a new crate

**Decision**: Walk the site in the build script, sort normalized relative paths, and emit Rust source containing path, media type, and `include_bytes!` entries into `OUT_DIR`.

**Rationale**: This handles content-hashed filenames with no runtime dependency. A sorted slice supports deterministic binary search.

**Alternatives considered**: An embedding crate adds dependency surface. Runtime archives or extraction add decompression, cleanup, and path authority.

## Decision 3: Use a narrow standard-library HTTP service

**Decision**: Use one dedicated standard-library listener on `127.0.0.1:0`. Parse one bounded request per connection, accept GET and HEAD, map the configured mount to embedded entries, and close after each response.

**Rationale**: The protocol surface cannot mutate, list, proxy, inspect the filesystem, or reach app state. Short timeouts and bounded parsing keep one worker sufficient.

**Alternatives considered**: A general server adds dependency and feature surface. `file://` is less reliable for routing and search. A webview adds a large platform runtime.

## Decision 4: Mount at `/eso-weave/` and reject ambiguous paths

**Decision**: Redirect `/` to `/eso-weave/`, strip only that mount, map directory paths to `index.html`, ignore a bounded query, and reject percent signs, backslashes, controls, empty or dot segments, and non-allow-listed keys.

**Rationale**: The existing site URL remains shared by Pages and the bundle. Strict rejection avoids canonicalization disagreements. Generated assets currently use compatible ASCII names.

## Decision 5: Reuse one application-owned service

**Decision**: The UI owns a `DocumentationService`. First access starts it; later actions reuse its URL. Drop signals shutdown, wakes the accept loop, and joins the worker.

**Rationale**: UI ownership matches application lifetime and avoids global state. A browser seam makes failure and reuse testable.

## Decision 6: Use native browser launch adapters

**Decision**: Windows uses `ShellExecuteW` with no child console. Linux launches `xdg-open` with null standard handles and no interactive input.

**Rationale**: Thin target adapters avoid shell interpolation and a new dependency while satisfying the hidden-process rule.

## Decision 7: Apply a restrictive mdBook-compatible CSP

**Decision**: Default to no sources; permit scripts, styles, images, and fonts only from the same origin, with the minimum inline allowance required by generated mdBook markup. Disallow objects, frames, forms, remote connections, and base changes.

**Rationale**: mdBook uses inline bootstrapping plus same-origin hashed assets. This preserves search and theming while preventing remote loading or app integration.
