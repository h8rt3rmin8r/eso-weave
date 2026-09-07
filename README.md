<p align="center">
  <img alt="ESO Weave" src="assets/eso-weave-banner.png" width="720">
</p>
<p align="center">
  <img alt="Version" src="https://img.shields.io/badge/version-0.14.0-2ea44f">
  <img alt="License" src="https://img.shields.io/badge/license-Apache--2.0-blue">
</p>
<p align="center">Cross-platform desktop companion for The Elder Scrolls Online.</p>

ESO Weave runs beside the game, never inside it. It provides configurable combat
weaving, fishing assistance, automatic potion use, and a live status dashboard.
The optional PixelBeacon companion addon supplies observable in-game state through
a small on-screen signal. ESO Weave never reads or writes game memory and never
touches game network traffic.

## Get ESO Weave

Download the Windows x64 installer or a Linux x86_64 package from
[GitHub Releases](https://github.com/h8rt3rmin8r/eso-weave/releases).

Start with the full [installation guide](docs/src/getting-started/installation.md),
then read the [responsible-use notice](docs/src/getting-started/responsible-use.md).

## Features

- [Weaving](docs/src/features/weaving.md) combines a basic attack with a skill
  press while preserving focus and safety gates.
- [Fishing](docs/src/features/fishing.md) automates the cast, bite, reel, and
  recast cycle through PixelBeacon observations.
- [Auto Potion](docs/src/features/auto-potion.md) uses the active quickslot when
  configured resources cross their thresholds.
- [Live interface](docs/src/features/interface.md) reports resources, runtime
  context, automation state, and installation status.
- [Ultimate resource](docs/src/features/ultimate-resource.md) shows current
  charge, active-bar cast cost, and readiness without controlling gameplay.

The complete, searchable manual is published at
[h8rt3rmin8r.github.io/eso-weave](https://h8rt3rmin8r.github.io/eso-weave/).
Its canonical source is the [documentation corpus](docs/src/README.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development requirements. Current
project procedures and historical records are indexed in the
[documentation map](docs/README.md).

## Disclaimer

This project is published for educational purposes only. It is not affiliated
with, endorsed by, or supported by ZeniMax Online Studios, ZeniMax Media Inc.,
Bethesda Softworks, or Microsoft. Automating gameplay input may violate The Elder
Scrolls Online Terms of Service. You are responsible for reviewing and complying
with all agreements that govern your account and accept all consequences of use,
including possible account suspension.

## License

Licensed under the [Apache License 2.0](LICENSE).
