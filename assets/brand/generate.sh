#!/usr/bin/env bash
#
# Regenerates every rasterized ESO Weave brand asset from the SVG masters in this
# directory. The committed rasters are the source of truth for the build; this
# script exists so they can be reproduced exactly.
#
# Requirements: ImageMagick 7 (the "magick" command) and the bundled Inter fonts
# under fonts/. SVG masters: eso-weave-mark.svg (badged icon) and
# eso-weave-glyph.svg (carets only, transparent, for lockups on ink).
#
# Notes:
#   - The weave over/under is baked into the SVG as an overlaid segment (no
#     clipPath), so ImageMagick's built-in SVG renderer produces correct output.
#   - SVG pieces are pre-rendered to PNG before compositing so a canvas "-size"
#     never bleeds into SVG rasterization (which would stretch and blur the mark).
#
set -euo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$DIR/../.." && pwd)"
MARK="$DIR/eso-weave-mark.svg"
GLYPH="$DIR/eso-weave-glyph.svg"
F="$DIR/fonts"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

# 1. Application, installer, shortcut and exe icon: multi-resolution ICO.
magick -background none "$MARK" -define icon:auto-resize=256,128,64,48,32,16 "$ROOT/assets/icon.ico"

# 2. Window icon consumed by the app at runtime (egui IconData).
magick -background none "$MARK" -resize 256x256 -depth 8 "$DIR/window-icon-256.png"

# 3. Linux package and AppImage icons.
magick -background none "$MARK" -resize 256x256 -depth 8 "$ROOT/packaging/linux/eso-weave.png"
magick -background none "$MARK" -resize 256x256 -depth 8 "$ROOT/packaging/appimage/AppDir/eso-weave.png"

# 4. Logo variants (transparent and on white).
magick -background none "$MARK" -resize 1024x1024 -depth 8 "$ROOT/assets/eso-weave-logo-clear.png"
magick "$MARK" -background white -flatten -resize 1024x1024 -depth 8 "$ROOT/assets/eso-weave-logo-white.png"

# 5. Wordmark lockup (glyph + "ESO" light + "Weave" gold, Inter SemiBold).
magick -background none -fill '#E6EDF3' -font "$F/Inter-SemiBold.ttf" -pointsize 300 -kerning 6 label:'ESO' -trim +repage "$TMP/eso.png"
magick -background none -fill '#F2B03C' -font "$F/Inter-SemiBold.ttf" -pointsize 300 -kerning 6 label:'Weave' -trim +repage "$TMP/weave.png"
magick -background none "$GLYPH" -resize 420x420 "$TMP/glyph420.png"
magick -size 70x10 xc:none "$TMP/sp1.png"
magick -size 44x10 xc:none "$TMP/sp2.png"
magick "$TMP/glyph420.png" "$TMP/sp1.png" "$TMP/eso.png" "$TMP/sp2.png" "$TMP/weave.png" -background none -gravity center +append "$TMP/lockup.png"

# 6. README banner (2000x668).
magick -size 2000x668 gradient:'#101823'-'#0B0E12' "$TMP/bg.png"
magick "$TMP/bg.png" \( "$TMP/lockup.png" -resize 1560x \) -gravity center -composite -depth 8 "$ROOT/assets/eso-weave-banner.png"

# 7. GitHub social-share image (1280x640) with tagline.
magick -size 1280x640 gradient:'#101823'-'#0A0D11' "$TMP/sbg.png"
magick -background none -fill '#8B97A7' -font "$F/Inter-Medium.ttf" -pointsize 40 -kerning 2 label:'Desktop companion for The Elder Scrolls Online' -trim +repage "$TMP/tag.png"
magick "$TMP/sbg.png" \
  \( "$TMP/lockup.png" -resize 900x \) -gravity center -geometry +0-40 -composite \
  \( "$TMP/tag.png" \) -gravity center -geometry +0+150 -composite \
  -depth 8 "$ROOT/assets/eso-weave-social.png"

# 8. Installer wizard art (light background so the default WiX dark text stays
#    readable): dialog (493x312) left ink art strip, banner (493x58) mark at right.
magick -background none "$GLYPH" -resize 112x112 "$TMP/glyph112.png"
magick -background none "$MARK" -resize 44x44 "$TMP/mark44.png"
magick -size 493x312 xc:white \
  \( -size 165x312 xc:'#0E1116' \) -gravity west -composite \
  \( -size 3x312 xc:'#F2B03C' \) -gravity west -geometry +165+0 -composite \
  "$TMP/glyph112.png" -gravity west -geometry +28+0 -composite \
  -alpha off -type truecolor -define bmp:format=bmp3 "$ROOT/packaging/windows/dialog.bmp"
magick -size 493x58 xc:white \
  \( -size 493x2 xc:'#F2B03C' \) -gravity south -composite \
  "$TMP/mark44.png" -gravity east -geometry +16+0 -composite \
  -alpha off -type truecolor -define bmp:format=bmp3 "$ROOT/packaging/windows/banner.bmp"

echo "Brand assets regenerated from SVG masters."
