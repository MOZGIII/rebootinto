#!/bin/bash
set -euo pipefail

# Build upon the build environment.
.github/scripts/build_env/windows.sh

dotnet tool install --global wix --version 4.0.0-rc.1
wix extension add --global WixToolset.UI.wixext/4.0.0-rc.1
