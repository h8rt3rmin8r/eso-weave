# Plan 041: v0.16.0 Release

Status: Complete, Archived

Sequence:

1. S091 prepared issue #163 by adding concise v0.16.0 Highlights, auditing the
   complete post-v0.15.1 record, and repairing the one-command release rollover
   for the bundled documentation version and date.
2. After the preparation pull request merged with green required checks, the
   release operator executed the reviewed v0.16.0 rollover on `main`, pushed the
   release tag, and monitored the gated package workflow.
3. The plan completed when the public GitHub Release exposed the Windows MSI,
   Linux tarball, Debian package, AppImage, and combined checksums.

Delivery evidence: [v0.16.0](https://github.com/h8rt3rmin8r/eso-weave/releases/tag/v0.16.0)
contains all five required release assets.

Issues #110, #129, #131, and #190 remain independent Release verification work.
Their installed and live observations do not make this completed plan active.
