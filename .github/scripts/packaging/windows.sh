#!/bin/bash
set -euo pipefail

ARTIFACTS_DIR="$1"

set -x

wix build wix/main.wxs \
  -ext WixToolset.UI.wixext \
  -o target/wix/rebootinto.msi

mkdir -p "$ARTIFACTS_DIR"

ARTIFACTS=(
  target/wix/*.msi
  target/release/rebootinto-cli.exe
  target/release/rebootinto-tui.exe
  target/release/rebootinto-iui.exe
  target/release/rebootinto-iced.exe
  target/release/rebootinto-gtk.exe
)

cp -t "$ARTIFACTS_DIR" "${ARTIFACTS[@]}"
