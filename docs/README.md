# Documentation Map

ESO Weave documentation is separated by audience and lifecycle. This boundary
keeps the reader site focused while preserving current project procedures and
historical decisions in the repository.

## Published Documentation

[`docs/src`](src/README.md) is the only source tree published by mdBook and the
only documentation intended for future application bundles. Its navigation is
defined by [`docs/src/SUMMARY.md`](src/SUMMARY.md).

## Current Project Records

[`docs/project`](project/) contains current maintainer procedures, accepted
architecture decisions, and the active build-plan sequence. This
material remains available on GitHub but is excluded from site navigation and
search.

- [Build-phase autopilot](project/build-autopilot.md)
- [Project governance](project/governance.md)
- [Release procedure](project/releasing.md)
- [Architecture decisions](project/architecture-decisions/)
- [Current build plans](project/build-plans/README.md)
- [Documentation migration ledger](project/migration-ledger.md)

## Historical Records

[`docs/archive`](archive/) preserves completed or superseded records for
chronology and traceability. Archived material is excluded from the published
site and does not direct current work.

- [Archived build plans](archive/build-plans/README.md)
- [Archived Ultimate resource announcement](archive/website/ultimate-resource-meter.md)

## Site Infrastructure

[`docs/book.toml`](book.toml) configures mdBook, while [`docs/theme`](theme/)
contains the local visual and accessibility enhancements used by the generated
site. These files support publication but are not reader or project content.
