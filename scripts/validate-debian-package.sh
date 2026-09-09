#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 1 ]]; then
  printf 'usage: %s <package.deb>\n' "${0##*/}" >&2
  exit 2
fi

package="$1"
if [[ ! -f "$package" || ! -r "$package" ]]; then
  printf 'Debian package is missing or unreadable: %s\n' "$package" >&2
  exit 1
fi

if ! command -v dpkg-deb >/dev/null 2>&1; then
  printf 'dpkg-deb is required to validate Debian package metadata\n' >&2
  exit 1
fi

required_fields=(Package Version Architecture Maintainer Description)
for field in "${required_fields[@]}"; do
  if ! value="$(dpkg-deb -f "$package" "$field")"; then
    printf 'Unable to read Debian control field %s from %s\n' "$field" "$package" >&2
    exit 1
  fi
  if [[ -z "${value//[[:space:]]/}" ]]; then
    printf 'Debian control field %s is missing or empty in %s\n' "$field" "$package" >&2
    exit 1
  fi
done

printf 'Debian package metadata is complete: %s\n' "$package"
