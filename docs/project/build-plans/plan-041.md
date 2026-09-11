# Plan 041: v0.16.0 Release

Status: Active

Sequence:

1. S091 prepares issue #163 by adding concise v0.16.0 Highlights, auditing the
   complete post-v0.15.1 record, and repairing the one-command release rollover
   for the bundled documentation version and date.
2. After the preparation pull request merges with green required checks, the
   authorized release operator executes the reviewed v0.16.0 rollover on
   `main`, pushes the release tag, and monitors the gated package workflow.
3. The plan completes when the public GitHub Release exposes the Windows MSI,
   Linux tarball, Debian package, AppImage, and combined checksums.

Issues #110, #129, and #131 remain independent Release verification work. Their
installed and live observations may continue after publication and never block
repository progress or creation of the release artifacts.
