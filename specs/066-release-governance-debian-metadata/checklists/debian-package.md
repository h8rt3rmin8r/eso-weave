# Debian Package Checklist: Release Governance and Debian Metadata

- [x] Cargo-deb metadata has an explicit valid Maintainer.
- [x] Valid Package, Version, Architecture, Maintainer, and Description pass.
- [x] An absent package and invalid argument count fail.
- [x] Every missing or whitespace-only required field fails independently.
- [x] The release gate runs after package construction and before upload.
- [x] Pull-request CI runs the fixture suite on Linux.
- [x] Built package metadata matches the explicit maintainer value.
- [x] Existing package assets and layout remain unchanged.
- [x] A separate issue owns next-release installation and query evidence.
