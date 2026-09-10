<p class="landing-wordmark">
  <img src="assets/brand/eso-weave-banner.png" alt="">
</p>

# <span class="visually-hidden">ESO Weave</span> Documentation

<dl class="project-metadata" aria-label="Documentation snapshot">
  <div>
    <dt>Handle</dt>
    <dd><code>eso-weave</code></dd>
  </div>
  <div>
    <dt>Applies to</dt>
    <dd>v0.15.1</dd>
  </div>
  <div>
    <dt>Released</dt>
    <dd><time datetime="2026-09-09">2026-09-09</time></dd>
  </div>
  <div>
    <dt>Repository</dt>
    <dd><a href="https://github.com/h8rt3rmin8r/eso-weave">github.com/h8rt3rmin8r/eso-weave</a></dd>
  </div>
</dl>

This build-time documentation snapshot follows package metadata in `Cargo.toml`
and release dates in `CHANGELOG.md`; update this block whenever those authorities
change.

ESO Weave is an offline-first desktop companion for The Elder Scrolls Online.
It provides focus-scoped combat weaving, optional fishing, and cautious Auto
Potion behavior without reading game memory or network traffic.

This manual explains the shipped application, its safety boundaries, and the
PixelBeacon screen-telemetry addon. Start with a task below, or use search for a
visible label, status, hotkey, addon, overlay, or failure message.

## Choose a path

- [Getting Started](getting-started/) covers installation, first launch,
  troubleshooting, and responsible use.
- [Features](features/) explains current user-visible behavior.
- [Concepts](concepts/) preserves scope, state, and input-safety reasoning.
- [Reference](reference/) collects settings, statuses, protocol, logging, and
  timing facts.
- [Development](development/) records architecture, brand, and repository
  conventions.

## Common tasks

- [Install ESO Weave](getting-started/installation.md)
- [Complete first launch](getting-started/first-launch.md)
- [Configure a weave](features/weaving.md)
- [Start Fishing](features/fishing.md)
- [Configure Auto Potion](features/auto-potion.md)
- [Install or update PixelBeacon](features/pixelbeacon.md)
- [Interpret a status](reference/status-reference.md)
- [Look up a setting](reference/settings.md)
- [Diagnose a problem](getting-started/troubleshooting.md)

ESO Weave is a combat weaving companion and desktop companion, but it remains
operator-controlled software. Read [Responsible Use](getting-started/responsible-use.md)
before enabling automation on a live account.
