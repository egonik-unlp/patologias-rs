#!/usr/bin/env bash
# Regenerates the favicon set in public/ from the SVG sources in branding/.
# Needs Inkscape 1.x and ImageMagick 7. See branding/README.md.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
src="$root/branding"
out="$root/public"

for tool in inkscape magick; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    echo "error: '$tool' not found. Install Inkscape 1.x and ImageMagick 7 to export brand assets." >&2
    exit 1
  fi
done

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

render() { # <svg> <width> <png>
  inkscape "$1" --export-type=png --export-width="$2" --export-filename="$3" >/dev/null 2>&1
}

# favicon.ico: 16/32/48 packed into one file
for size in 16 32 48; do
  render "$src/mark.svg" "$size" "$tmp/mark-$size.png"
done
magick "$tmp/mark-16.png" "$tmp/mark-32.png" "$tmp/mark-48.png" "$out/favicon.ico"

# Touch icon and manifest icons
# iOS paints transparent corners black, so the touch icon uses the full-bleed variant.
render "$src/mark-maskable.svg" 180 "$out/apple-touch-icon.png"
render "$src/mark.svg" 192 "$out/icon-192.png"
render "$src/mark.svg" 512 "$out/icon-512.png"
render "$src/mark-maskable.svg" 512 "$out/icon-maskable-512.png"

# SVG favicon is the source itself
cp "$src/mark.svg" "$out/favicon.svg"

echo "Brand assets written to $out"
