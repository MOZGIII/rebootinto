#!/bin/bash
set -euo pipefail

choco install --no-progres \
  ninja \
  llvm \
  vswhere

choco install --no-progres --source python \
  meson
