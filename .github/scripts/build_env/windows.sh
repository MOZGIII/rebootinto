#!/bin/bash
set -euo pipefail

choco install \
  ninja \
  llvm \
  vswhere \
  msys2 \
  python3

choco install --source python \
  meson \
  gvsbuild

gvsbuild build gtk4
