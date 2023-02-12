#!/bin/bash
set -euo pipefail

choco install \
  ninja \
  llvm \
  vswhere

choco install --source python \
  meson

pipx install gvsbuild
gvsbuild build gtk4
