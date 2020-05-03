#!/bin/bash
set -euo pipefail

choco install \
  ninja \
  llvm \
  vswhere

choco install --source python \
  meson
