# Quickstart: Refresh Documentation Screenshots

1. Create an ignored repository-local output directory below `target/`.
2. Run the S083 capture command from the repository root.
3. Confirm the generated manifest contains 28 successful captures.
4. Review the seven dark-wide variants against `screenshot-plan.md`.
5. Run `scripts/curate-documentation-captures.ps1` with the capture root and authorized MSI source path to crop and publish the selected PNGs.
6. Recompute dimensions, sizes, and SHA-256 digests in `docs/project/documentation-screenshots.json`.
7. Review alt text, captions, adjacent instructions, and synthetic labels.
8. Run documentation policy tests, build, linkcheck, spelling, text hygiene, and the full Cargo merge gate.

Do not launch ESO, capture a desktop, replace the maintainer-supplied Windows image without approval, or publish files directly from `target/`.
