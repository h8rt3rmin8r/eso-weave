#!/usr/bin/env bash
# Installs the Linux system libraries required to build the ESO Weave GUI and its
# input and pixel-bus backends. This is the single source shared by CI and
# developer machines so the two cannot drift (see docs/releasing.md).
#
# Run with elevated privileges, as the pinned release pipeline does
# (`sudo scripts/linux-build-deps.sh`). Targets Debian/Ubuntu (apt).
#
# Do not modify without a dated decision recorded in CHANGELOG.md.
set -euo pipefail

export DEBIAN_FRONTEND=noninteractive

apt-get update

# Windowing and GL for the eframe/glow GUI; X11/XCB, xkbcommon, and Wayland for
# windowing and the X11 pixel-bus sampler; udev/evdev for the input backend.
apt-get install -y --no-install-recommends \
  pkg-config \
  libgl1-mesa-dev \
  libx11-dev \
  libxcb1-dev \
  libxkbcommon-dev \
  libxkbcommon-x11-dev \
  libwayland-dev \
  libxcursor-dev \
  libxrandr-dev \
  libxi-dev \
  libudev-dev \
  libevdev-dev
