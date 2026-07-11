#!/usr/bin/env bash
# Prints the body of the "## [<heading>]" section of CHANGELOG.md.
#
# The argument is the bracketed heading text, for example "Unreleased" or a
# version like "0.1.0". Matching is on the bracketed text only, so a heading with
# a trailing date ("## [0.1.0] - 2026-07-11") still matches "0.1.0". The output is
# the lines between that heading and the next "## " heading, excluding both
# heading lines. An absent section prints nothing.
#
# Used by the pinned release pipeline (.github/workflows/release.yml) for the
# verify gate and to assemble release notes. Do not modify without a dated
# decision recorded in CHANGELOG.md.
set -euo pipefail

heading="${1:?usage: changelog-section.sh <heading>}"

awk -v want="$heading" '
  /^## \[/ {
    key = $0
    sub(/^## \[/, "", key)
    sub(/\].*$/, "", key)
    if (key == want) {
      in_section = 1
      next
    }
    if (in_section) {
      exit
    }
  }
  in_section { print }
' CHANGELOG.md
