#!/bin/bash
set -euo pipefail

ARTIFACTS_DIR="$1"

# TODO: do not build the code again
gbp buildpackage \
  --git-ignore-branch \
  --no-sign \
  --no-pre-clean \
  --no-check-builddeps \
  --build=binary

mkdir -p "$ARTIFACTS_DIR"

ARTIFACTS=(
  ../*.deb
)

cp -t "$ARTIFACTS_DIR" "${ARTIFACTS[@]}"
