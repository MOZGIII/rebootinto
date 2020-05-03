#!/bin/bash
set -euo pipefail

ARTIFACTS_DIR="$1"

cargo wix \
  --nocapture \
  --name rebootinto \
  --install-version "0.0.1"

mkdir -p "$ARTIFACTS_DIR"

ARTIFACTS=(
  target/wix/*.msi
  target/release/rebootinto-cli.exe
  target/release/rebootinto-tui.exe
  target/release/rebootinto-iui.exe
  target/release/rebootinto-iced.exe
)

cp -t "$ARTIFACTS_DIR" "${ARTIFACTS[@]}"
