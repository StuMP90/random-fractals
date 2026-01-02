#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APPDIR="$PROJECT_ROOT/target/appimage/AppDir"
OUTDIR="$PROJECT_ROOT/target/appimage/out"

APP_NAME="random-fractals-gui"

mkdir -p "$APPDIR/usr/bin" \
         "$APPDIR/usr/share/applications" \
         "$APPDIR/usr/share/icons/hicolor/scalable/apps" \
         "$OUTDIR"

cargo build --release

cp "$PROJECT_ROOT/target/release/$APP_NAME" "$APPDIR/usr/bin/$APP_NAME"
cp "$PROJECT_ROOT/packaging/linux/$APP_NAME.desktop" "$APPDIR/$APP_NAME.desktop"
cp "$PROJECT_ROOT/packaging/linux/$APP_NAME.desktop" "$APPDIR/usr/share/applications/$APP_NAME.desktop"

cp "$PROJECT_ROOT/packaging/linux/$APP_NAME.svg" "$APPDIR/$APP_NAME.svg"
cp "$PROJECT_ROOT/packaging/linux/$APP_NAME.svg" "$APPDIR/usr/share/icons/hicolor/scalable/apps/$APP_NAME.svg"
cp "$PROJECT_ROOT/packaging/linux/AppRun" "$APPDIR/AppRun"
chmod +x "$APPDIR/AppRun" "$APPDIR/usr/bin/$APP_NAME"

if [[ ! -f "$OUTDIR/appimagetool" ]]; then
  echo "Downloading appimagetool..."
  curl -L "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage" -o "$OUTDIR/appimagetool"
  chmod +x "$OUTDIR/appimagetool"
fi

"$OUTDIR/appimagetool" "$APPDIR" "$OUTDIR/RandomFractals-x86_64.AppImage"

echo "AppImage written to: $OUTDIR/RandomFractals-x86_64.AppImage"
