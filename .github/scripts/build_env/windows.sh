#!/bin/bash
set -euo pipefail

choco install -y --no-progress \
  ninja \
  llvm \
  vswhere

choco install -y --no-progress --source python \
  meson
