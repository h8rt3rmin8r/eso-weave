# Plan 027: mdBook Documentation Foundation

Status: Active

Sequence:

1. Slice 057 establishes the exact mdBook and link-checker toolchain, minimal
   branded `docs/src` book, and dependency-free policy fixtures.
2. Validate navigation completeness, local links, offline resources,
   accessibility basics, repository subpath behavior, and ignored output.
3. Build on pull requests with read-only permissions, then upload and deploy the
   exact validated artifact only from `main` through the protected Pages
   environment.
4. Leave content migration, comprehensive writing, binary embedding, and package
   verification to issues #80, #81, #82, and #84 in that order.
