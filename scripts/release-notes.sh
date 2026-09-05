#!/usr/bin/env bash
# Generates a compact GitHub release body from a changelog Highlights subsection.
set -euo pipefail

version="${1:-}"
repository="${2:-${GITHUB_REPOSITORY:-h8rt3rmin8r/eso-weave}}"
changelog_file="${3:-CHANGELOG.md}"
heading="${CHANGELOG_HEADING:-$version}"

if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+([.-][0-9A-Za-z.-]+)?$ ]]; then
  echo "release-notes: invalid release version '$version'; expected X.Y.Z without a leading v" >&2
  exit 2
fi

if [[ ! "$repository" =~ ^[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+$ ]]; then
  echo "release-notes: invalid repository slug '$repository'; expected owner/repository" >&2
  exit 2
fi

if [[ "$heading" != "Unreleased" && ! "$heading" =~ ^[0-9]+\.[0-9]+\.[0-9]+([.-][0-9A-Za-z.-]+)?$ ]]; then
  echo "release-notes: invalid changelog heading '$heading'" >&2
  exit 2
fi

if [[ ! -f "$changelog_file" ]]; then
  echo "release-notes: changelog file was not found: $changelog_file" >&2
  exit 2
fi

if ! awk -v want="$heading" '
  {
    line = $0
    sub(/\r$/, "", line)
  }
  line ~ /^## \[/ {
    key = line
    sub(/^## \[/, "", key)
    sub(/\].*$/, "", key)
    if (key == want) {
      found = 1
    }
  }
  END { exit(found ? 0 : 1) }
' "$changelog_file"; then
  echo "release-notes: version section [$heading] was not found in $changelog_file" >&2
  exit 3
fi

highlight_heading_count="$(awk -v want="$heading" '
  {
    line = $0
    sub(/\r$/, "", line)
  }
  line ~ /^## \[/ {
    if (in_version) {
      exit
    }
    key = line
    sub(/^## \[/, "", key)
    sub(/\].*$/, "", key)
    in_version = (key == want)
    next
  }
  in_version && line == "### Highlights" { count++ }
  END { print count + 0 }
' "$changelog_file")"

if [[ "$highlight_heading_count" -eq 0 ]]; then
  echo "release-notes: version section [$heading] has no Highlights subsection" >&2
  exit 4
fi
if [[ "$highlight_heading_count" -ne 1 ]]; then
  echo "release-notes: version section [$heading] has $highlight_heading_count Highlights subsections; expected exactly 1" >&2
  exit 4
fi

highlights="$(awk -v want="$heading" '
  {
    line = $0
    sub(/\r$/, "", line)
  }
  line ~ /^## \[/ {
    if (in_version) {
      exit
    }
    key = line
    sub(/^## \[/, "", key)
    sub(/\].*$/, "", key)
    in_version = (key == want)
    next
  }
  in_version && line == "### Highlights" {
    capture = 1
    next
  }
  in_version && capture && line ~ /^### / { exit }
  in_version && capture { lines[++count] = line }
  END {
    first = 1
    while (first <= count && lines[first] ~ /^[[:space:]]*$/) {
      first++
    }
    last = count
    while (last >= first && lines[last] ~ /^[[:space:]]*$/) {
      last--
    }
    for (i = first; i <= last; i++) {
      print lines[i]
    }
  }
' "$changelog_file")"

if [[ -z "$(printf '%s' "$highlights" | tr -d '[:space:]')" ]]; then
  echo "release-notes: version section [$heading] Highlights subsection is empty" >&2
  exit 5
fi

first_line="${highlights%%$'\n'*}"
if [[ ! "$first_line" =~ ^-\  ]]; then
  echo "release-notes: Highlights must begin with a top-level '- ' bullet" >&2
  exit 5
fi

if ! printf '%s\n' "$highlights" | awk '
  /^[[:space:]]*$/ { next }
  /^- / { next }
  /^[[:space:]]/ { next }
  { exit 1 }
'; then
  echo "release-notes: Highlights continuation lines must be indented under a top-level bullet" >&2
  exit 5
fi

bullet_count="$(printf '%s\n' "$highlights" | grep -c '^- ' || true)"
if [[ "$bullet_count" -gt 6 ]]; then
  echo "release-notes: Highlights contains $bullet_count top-level bullets; maximum is 6" >&2
  exit 6
fi

word_count="$(printf '%s\n' "$highlights" | sed -E 's/^[[:space:]]*-[[:space:]]+//' | wc -w | tr -d '[:space:]')"
if [[ "$word_count" -gt 120 ]]; then
  echo "release-notes: Highlights contains $word_count words; maximum is 120" >&2
  exit 6
fi

printf '%s\n\n' "$highlights"
printf '[Read the full changelog for v%s](https://github.com/%s/blob/v%s/CHANGELOG.md)\n' \
  "$version" "$repository" "$version"
