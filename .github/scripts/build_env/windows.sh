#!/bin/bash
set -euo pipefail

choco install --no-progress \
  ninja \
  llvm \
  vswhere \
  msys2

pacman -S mingw-w64-x86_64-gtk4
