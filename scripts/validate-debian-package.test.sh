#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
validator="$repo_root/scripts/validate-debian-package.sh"
fixture_root="$(mktemp -d)"
trap 'rm -rf "$fixture_root"' EXIT

expected_maintainer='maintainer = "h8rt3rmin8r <46768484+h8rt3rmin8r@users.noreply.github.com>"'
if ! grep -Fqx "$expected_maintainer" "$repo_root/Cargo.toml"; then
  printf 'Cargo.toml does not contain the required Debian maintainer\n' >&2
  exit 1
fi

mkdir -p "$fixture_root/bin"
touch "$fixture_root/package.deb"

cat > "$fixture_root/bin/dpkg-deb" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail

if [[ "${FAIL_QUERY:-0}" == "1" ]]; then
  exit 2
fi

field="${3:-}"
if [[ "$field" == "${MISSING_FIELD:-}" ]]; then
  exit 0
fi
if [[ "$field" == "${WHITESPACE_FIELD:-}" ]]; then
  printf '   \n'
  exit 0
fi

case "$field" in
  Package) printf 'eso-weave\n' ;;
  Version) printf '0.15.0-1\n' ;;
  Architecture) printf 'amd64\n' ;;
  Maintainer) printf 'h8rt3rmin8r <46768484+h8rt3rmin8r@users.noreply.github.com>\n' ;;
  Description) printf 'Desktop companion application for The Elder Scrolls Online.\n' ;;
  *) exit 3 ;;
esac
STUB
chmod +x "$fixture_root/bin/dpkg-deb"

expect_pass() {
  if ! PATH="$fixture_root/bin:$PATH" "$@" >/dev/null 2>&1; then
    printf 'expected success: %s\n' "$*" >&2
    exit 1
  fi
}

expect_fail() {
  if PATH="$fixture_root/bin:$PATH" "$@" >/dev/null 2>&1; then
    printf 'expected failure: %s\n' "$*" >&2
    exit 1
  fi
}

expect_pass "$validator" "$fixture_root/package.deb"
expect_fail "$validator"
expect_fail "$validator" "$fixture_root/package.deb" extra
expect_fail "$validator" "$fixture_root/missing.deb"

for field in Package Version Architecture Maintainer Description; do
  expect_fail env MISSING_FIELD="$field" "$validator" "$fixture_root/package.deb"
  expect_fail env WHITESPACE_FIELD="$field" "$validator" "$fixture_root/package.deb"
done

expect_fail env FAIL_QUERY=1 "$validator" "$fixture_root/package.deb"

if command -v /usr/bin/dpkg-deb >/dev/null 2>&1; then
  mkdir -p "$fixture_root/real-valid/DEBIAN" "$fixture_root/real-valid/usr/share/eso-weave"
  cat > "$fixture_root/real-valid/DEBIAN/control" <<'CONTROL'
Package: eso-weave
Version: 0.15.0-1
Section: utils
Priority: optional
Architecture: amd64
Maintainer: h8rt3rmin8r <46768484+h8rt3rmin8r@users.noreply.github.com>
Description: Desktop companion application for The Elder Scrolls Online.
CONTROL
  /usr/bin/dpkg-deb --build "$fixture_root/real-valid" "$fixture_root/real-valid.deb" >/dev/null
  /usr/bin/env PATH="/usr/bin:/bin" "$validator" "$fixture_root/real-valid.deb" >/dev/null

  mkdir -p "$fixture_root/real-missing/DEBIAN" "$fixture_root/real-missing/usr/share/eso-weave"
  cat > "$fixture_root/real-missing/DEBIAN/control" <<'CONTROL'
Package: eso-weave
Version: 0.15.0-1
Section: utils
Priority: optional
Architecture: amd64
Description: Desktop companion application for The Elder Scrolls Online.
CONTROL
  /usr/bin/dpkg-deb --build "$fixture_root/real-missing" "$fixture_root/real-missing.deb" >/dev/null 2>&1
  if /usr/bin/env PATH="/usr/bin:/bin" "$validator" "$fixture_root/real-missing.deb" >/dev/null 2>&1; then
    printf 'expected real package without Maintainer to fail\n' >&2
    exit 1
  fi
fi

printf 'Debian package validator tests passed\n'
