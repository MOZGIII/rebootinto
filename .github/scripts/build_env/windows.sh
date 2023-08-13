#!/bin/bash
set -euo pipefail

choco install --no-progress \
  ninja \
  llvm \
  vswhere

choco install --no-progress --source python \
  meson
