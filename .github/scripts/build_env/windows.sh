#!/bin/bash
set -euo pipefail

choco install --no-progress \
  ninja \
  llvm \
  vswhere \
  msys2 \
  python3

choco install --no-progress --source python  \
  meson \
  gvsbuild

gvsbuild build gtk4
