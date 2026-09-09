# Quickstart: Release Governance and Debian Metadata

1. Run `bash scripts/validate-debian-package.test.sh` on Linux.
2. Build the release binary and `.deb` with the pinned documentation tools and
   cargo-deb path.
3. Run `scripts/validate-debian-package.sh target/debian/*.deb`.
4. Query the generated control record with `dpkg-deb -f` and confirm the exact
   Maintainer value.
5. Run documentation policy and full Rust CI parity.
6. Confirm constitution 2.0.1 and every tracked operating guide describe the
   same pre-publication, publication, and Release verification sequence.
7. Confirm #104 and #105 close through the S066 pull request while the new
   next-release verification issue remains open.
