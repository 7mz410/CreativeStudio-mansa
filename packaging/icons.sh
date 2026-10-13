#!/usr/bin/env bash
# Regenerate every app and tray icon from the Creative Studio star (docs/studio/mansa.svg).
#
# - App icon (Dock, taskbar, Start menu, Linux menus, installers): the star on a navy tile, which
#   studio/fork/brand_files.py writes to assets/app-icon/artcraft-launcher.svg. Run that first.
# - Tray: the black star as the macOS menu-bar template; the red star elsewhere.
#
# Needs: resvg (brew install resvg / cargo install resvg). The .ico and .icns are packed by
# `cargo xtask ico` / `cargo xtask icns`, so this runs on any OS. The outputs are committed, so
# packaging (and the Rust build, which include_bytes!s some of them) never needs these tools.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
SVG="$DIR/artcraft-launcher.svg"
STUDIO="$ROOT/docs/studio/mansa.svg"
ID="ai.storyteller.artcraft-launcher"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

command -v resvg >/dev/null || { echo "error: resvg not found (brew install resvg)" >&2; exit 1; }
[ -f "$SVG" ] || { echo "error: $SVG missing: run studio/fork/brand_files.py first" >&2; exit 1; }

render() { resvg -w "$2" -h "$2" "$1" "$3" </dev/null; }

render "$SVG" 1024 "$DIR/artcraft-launcher-1024.png"

# Linux hicolor theme.
for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/apps"
  render "$SVG" "$s" "$DIR/hicolor/${s}x${s}/apps/$ID.png"
done
mkdir -p "$DIR/hicolor/scalable/apps"
cp "$SVG" "$DIR/hicolor/scalable/apps/$ID.svg"

# Windows .ico.
ICO_PNGS=()
for s in 16 20 24 32 40 48 64 128 256; do
  render "$SVG" "$s" "$TMP/ico-$s.png"
  ICO_PNGS+=("$TMP/ico-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- ico "$DIR/artcraft-launcher.ico" "${ICO_PNGS[@]}")

# macOS .icns.
ICNS_PNGS=()
for s in 16 32 64 128 256 512 1024; do
  render "$SVG" "$s" "$TMP/icns-$s.png"
  ICNS_PNGS+=("$TMP/icns-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- icns "$DIR/artcraft-launcher.icns" "${ICNS_PNGS[@]}")

# Tray: the star alone, black for the macOS menu bar (a template image), red elsewhere.
mkdir -p "$ROOT/assets/tray"
sed 's/#eb1b3c/#000000/' "$STUDIO" >"$TMP/mansa-black.svg"
render "$TMP/mansa-black.svg" 44 "$ROOT/assets/tray/mansa-44.png"
render "$STUDIO" 64 "$ROOT/assets/tray/mansa-64.png"
