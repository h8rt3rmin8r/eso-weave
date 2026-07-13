# Research: ESO API Version Check Automation

## Decision: version source is a bump-detection signal, not the numeric value

**Decision**: Fetch the live ESO game client version string (for example
`12.0.6`) from the official esoui/esoui GitHub `live` branch head commit and use
it purely to detect that a client API change shipped. The numeric API version
written into the manifest is resolved locally as `max(stored last known, compiled
default)`.

**Rationale**: The exact numeric API version (for example `101050`) that the
manifest `## APIVersion:` field requires is only published on sources behind a
Cloudflare JavaScript challenge that a plain HTTP client cannot pass. Direct
verification during research:

- `wiki.esoui.com` (canonical `Current_API_Version` page and its MediaWiki API):
  returns a Cloudflare "Just a moment" interstitial even to a real browser. A
  headless client cannot obtain the number.
- `esodata.uesp.net` (page title embeds the version, for example `v101049`): same
  Cloudflare challenge.
- `en.uesp.net` MediaWiki API: bot-friendly and returns clean JSON, but does not
  carry the current numeric API version.
- `api.github.com/repos/esoui/esoui/*`: bot-friendly, stable, no challenge. Tags
  are PTS build names (`pts12.0`); the `live` branch head commit message is the
  live game version string (confirmed `12.0.6`, dated 2026-06-29, matching the
  live API `101050` release). This is a reliable bump signal but not the number.
- `api.mmoui.com/v3/game/ESO/filelist.json`: bot-friendly JSON; `UICompatibility`
  reports the game version string (`12.0.0 Season Zero Pt.2`), again not the
  numeric API version.
- Third-party library manifests (for example LibAddonMenu on GitHub) template the
  value as `@API_VERSION@` in source; the resolved number exists only in packaged
  release archives.

Given this, the reliable, honest design detects bumps over a bot-friendly channel
and fills the number from a compiled default that release upkeep advances. This
matches the operator's chosen option (GitHub detect plus compiled default).

**Alternatives considered**:

- ESOUI wiki exact fetch: rejected; Cloudflare blocks a plain client on most or
  all networks, so it degrades to the default in practice while adding fragility.
- Game-version to API-version mapping table: rejected as the default; it needs a
  new row each release, the same cadence as bumping the compiled default, for
  marginal additional auto-fill. The compiled default alone is simpler.

## Decision: blocking HTTP client is `ureq` with rustls

**Decision**: Add `ureq` (2.x, rustls TLS) as a dependency for a single blocking
HTTPS GET on a `std::thread`.

**Rationale**: The app has no async runtime and introduces none. `ureq` is a
small, synchronous client with rustls TLS (no OpenSSL system dependency), a simple
call-and-read API, and easy per-request timeout and header control (GitHub
requires a `User-Agent`). It fits the existing thread-and-channel concurrency model
and the Windows plus Linux targets.

**Alternatives considered**:

- `reqwest`: rejected; async-first and heavy (pulls tokio), contradicting the
  no-async-runtime constraint.
- `minreq`/`attohttpc`: viable and smaller, but `ureq` has the widest maintenance
  and clearest timeout/TLS story; the size difference is immaterial for a desktop
  app that already bundles egui.

`Cargo.toml` is not a pinned artifact, but the added networked dependency is
architecture-affecting, so it is recorded as a dated `CHANGELOG.md` decision.

## Decision: numeric resolution and manifest upkeep

**Decision**: Effective numeric API version = `max(stored last known, compiled
DEFAULT_API_VERSION)`. Persist the effective value back as the new last known.
When the addon is installed and the on-disk manifest carries the managed marker,
rewrite only the `## APIVersion:` field when the effective value differs from the
on-disk primary token; never downgrade.

**Rationale**: The compiled default advances only when a new ESO Weave release
raises it, so `max` cleanly rolls the stored value and the manifest forward across
app updates while never regressing. Keying the write on the marker reuses the
existing safety guarantee and confines writes to files ESO Weave owns.

**Multi-value token rule (spec FR-008a)**: The `## APIVersion:` field may list
several space-separated numeric tokens (the shipped manifest lists `101050
101054`, a live value plus a maintainer-added future value). On rewrite: set the
effective value as the primary (first) token, preserve any existing tokens greater
than the primary, and drop any tokens less than the primary. This advances the
live value, keeps a deliberately added forward-compatible value, and never lists
stale versions.

## Decision: bump surfacing

**Decision**: Compile in `DEFAULT_GAME_VERSION` (`12.0.6`) alongside
`DEFAULT_API_VERSION` (`101050`). When the fetched game version parses newer than
both `DEFAULT_GAME_VERSION` and the stored last seen game version, record the new
game version and emit a plain-language warning to the live log advising the player
to update ESO Weave. Never guess a numeric API version for an unknown game version.

**Rationale**: This delivers the "automatically detect when API version bumps
occur" requirement over a reliable channel while keeping the numeric value honest.
The live log viewer is the existing surface for such notices; no new UI framework
is required. A future slice may add a dismissible banner.

## Decision: persistence location

**Decision**: Store `last_known_api_version` and `last_seen_game_version` in
`state.json` (`SessionState`) as an additive `serde(default)` section, not in
`config.json`.

**Rationale**: These are derived runtime values, not user preferences. The
constitution and the `state.rs` module header require runtime and derived state to
live in `state.json`; `config.json` is settings only. Additive `serde(default)`
fields keep old state files loading cleanly.
