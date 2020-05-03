#!/bin/bash
set -euo pipefail

# Build upon the build environment.
.github/scripts/build_env/windows.sh

choco install wixtoolset
cargo install -f cargo-wix
