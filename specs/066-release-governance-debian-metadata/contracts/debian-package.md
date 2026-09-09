# Contract: Debian Package Metadata Validation

1. Invoke `scripts/validate-debian-package.sh <package.deb>` on Linux.
2. Reject any argument count other than one.
3. Reject a missing, non-file, or unreadable input.
4. Query only the package supplied by the caller.
5. Require non-whitespace Package, Version, Architecture, Maintainer, and
   Description values.
6. Report the missing field and exit nonzero without modifying the package.
7. Print a concise success line naming the validated package.
8. The release workflow invokes this contract after `cargo deb` and before
   copying, hashing, uploading, or publishing the package.
