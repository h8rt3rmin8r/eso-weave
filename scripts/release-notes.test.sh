#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
generator="$repo_root/scripts/release-notes.sh"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

fail() {
  echo "FAIL: $*" >&2
  exit 1
}

assert_equals() {
  local expected="$1"
  local actual="$2"
  local name="$3"
  if [[ "$actual" != "$expected" ]]; then
    printf 'FAIL: %s\nExpected:\n%s\nActual:\n%s\n' "$name" "$expected" "$actual" >&2
    exit 1
  fi
}

assert_fails_with() {
  local expected_message="$1"
  shift
  local output
  local status

  set +e
  output="$($generator "$@" 2>&1)"
  status=$?
  set -e

  if [[ $status -eq 0 ]]; then
    fail "command unexpectedly succeeded: $*"
  fi
  if [[ "$output" != *"$expected_message"* ]]; then
    fail "expected failure containing '$expected_message', got '$output'"
  fi
}

if [[ ! -x "$generator" ]]; then
  fail "missing executable generator: $generator"
fi

valid="$tmp_dir/valid.md"
printf '%s\r\n' \
  '# Changelog' \
  '' \
  '## [1.2.3] - 2026-09-04' \
  '' \
  '### Highlights' \
  '' \
  '- First user-facing result with `inline code`.' \
  '  Continuation text stays with the first item.' \
  '- Second result links to [details](https://example.com).' \
  '  - Nested context remains part of the second item.' \
  '' \
  '### Added' \
  '' \
  '- Detailed engineering entry that must not appear.' \
  '' \
  '### Decisions' \
  '' \
  '- Internal decision that must not appear.' > "$valid"

expected="$(printf '%s\n' \
  '- First user-facing result with `inline code`.' \
  '  Continuation text stays with the first item.' \
  '- Second result links to [details](https://example.com).' \
  '  - Nested context remains part of the second item.' \
  '' \
  '[Read the full changelog for v1.2.3](https://github.com/example/project/blob/v1.2.3/CHANGELOG.md)')"
actual="$($generator 1.2.3 example/project "$valid")"
assert_equals "$expected" "$actual" "valid extraction"

level_two_boundary="$tmp_dir/level-two-boundary.md"
printf '%s\n' \
  '# Changelog' \
  '## [1.2.3]' \
  '### Highlights' \
  '- Included.' \
  '## Metadata' \
  'This level-two section is outside the release excerpt.' > "$level_two_boundary"
actual="$($generator 1.2.3 example/project "$level_two_boundary")"
expected="$(printf '%s\n' \
  '- Included.' \
  '' \
  '[Read the full changelog for v1.2.3](https://github.com/example/project/blob/v1.2.3/CHANGELOG.md)')"
assert_equals "$expected" "$actual" "unbracketed level-two boundary"

six_items="$tmp_dir/six-items.md"
{
  printf '%s\n' '# Changelog' '## [1.2.3]' '### Highlights'
  for item in 1 2 3 4 5 6; do
    printf -- '- Boundary highlight %s.\n' "$item"
  done
} > "$six_items"
$generator 1.2.3 example/project "$six_items" > /dev/null

one_hundred_twenty_words="$tmp_dir/120-words.md"
{
  printf '%s\n' '# Changelog' '## [1.2.3]' '### Highlights'
  printf -- '-'
  for _ in $(seq 1 120); do
    printf ' word'
  done
  printf '\n'
} > "$one_hundred_twenty_words"
$generator 1.2.3 example/project "$one_hundred_twenty_words" > /dev/null

unreleased="$tmp_dir/unreleased.md"
printf '%s\n' \
  '# Changelog' \
  '' \
  '## [Unreleased]' \
  '' \
  '### Highlights' \
  '' \
  '- Candidate release highlight.' \
  '' \
  '### Added' \
  '' \
  '- Detail.' > "$unreleased"
actual="$(CHANGELOG_HEADING=Unreleased $generator 1.2.3 example/project "$unreleased")"
expected="$(printf '%s\n' \
  '- Candidate release highlight.' \
  '' \
  '[Read the full changelog for v1.2.3](https://github.com/example/project/blob/v1.2.3/CHANGELOG.md)')"
assert_equals "$expected" "$actual" "Unreleased preview"

missing_version="$tmp_dir/missing-version.md"
printf '%s\n' '# Changelog' '## [9.9.9]' '### Highlights' '- Other.' > "$missing_version"
assert_fails_with "version section [1.2.3] was not found" 1.2.3 example/project "$missing_version"

missing_highlights="$tmp_dir/missing-highlights.md"
printf '%s\n' '# Changelog' '## [1.2.3]' '### Added' '- Detail.' > "$missing_highlights"
assert_fails_with "has no Highlights subsection" 1.2.3 example/project "$missing_highlights"

empty_highlights="$tmp_dir/empty-highlights.md"
printf '%s\n' '# Changelog' '## [1.2.3]' '### Highlights' '' '### Added' '- Detail.' > "$empty_highlights"
assert_fails_with "Highlights subsection is empty" 1.2.3 example/project "$empty_highlights"

empty_bullet="$tmp_dir/empty-bullet.md"
printf '%s\n' '# Changelog' '## [1.2.3]' '### Highlights' '- ' > "$empty_bullet"
assert_fails_with "each top-level bullet must contain text" 1.2.3 example/project "$empty_bullet"

mixed_empty_bullet="$tmp_dir/mixed-empty-bullet.md"
printf '%s\n' \
  '# Changelog' \
  '## [1.2.3]' \
  '### Highlights' \
  '- Real item.' \
  '- ' > "$mixed_empty_bullet"
assert_fails_with "each top-level bullet must contain text" 1.2.3 example/project "$mixed_empty_bullet"

duplicate_highlights="$tmp_dir/duplicate-highlights.md"
printf '%s\n' \
  '# Changelog' \
  '## [1.2.3]' \
  '### Highlights' \
  '- First.' \
  '### Added' \
  '- Detail.' \
  '### Highlights' \
  '- Second.' > "$duplicate_highlights"
assert_fails_with "has 2 Highlights subsections; expected exactly 1" 1.2.3 example/project "$duplicate_highlights"

invalid_shape="$tmp_dir/invalid-shape.md"
printf '%s\n' '# Changelog' '## [1.2.3]' '### Highlights' 'A paragraph is not a highlight list.' > "$invalid_shape"
assert_fails_with "must begin with a top-level '- ' bullet" 1.2.3 example/project "$invalid_shape"

unindented_continuation="$tmp_dir/unindented-continuation.md"
printf '%s\n' \
  '# Changelog' \
  '## [1.2.3]' \
  '### Highlights' \
  '- First.' \
  'This line is not part of the Markdown list.' > "$unindented_continuation"
assert_fails_with "continuation lines must be indented" 1.2.3 example/project "$unindented_continuation"

too_many="$tmp_dir/too-many.md"
{
  printf '%s\n' '# Changelog' '## [1.2.3]' '### Highlights'
  for item in 1 2 3 4 5 6 7; do
    printf -- '- Highlight %s.\n' "$item"
  done
} > "$too_many"
assert_fails_with "contains 7 top-level bullets; maximum is 6" 1.2.3 example/project "$too_many"

too_wordy="$tmp_dir/too-wordy.md"
{
  printf '%s\n' '# Changelog' '## [1.2.3]' '### Highlights'
  printf -- '-'
  for _ in $(seq 1 121); do
    printf ' word'
  done
  printf '\n'
} > "$too_wordy"
assert_fails_with "contains 121 words; maximum is 120" 1.2.3 example/project "$too_wordy"

assert_fails_with "invalid release version" v1.2.3 example/project "$valid"
assert_fails_with "invalid repository slug" 1.2.3 example "$valid"
assert_fails_with "changelog file was not found" 1.2.3 example/project "$tmp_dir/absent.md"

echo "release-notes contract tests passed"
