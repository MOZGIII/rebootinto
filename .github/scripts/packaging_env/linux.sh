#!/bin/bash
set -euo pipefail

# Build upon the build environment.
.github/scripts/build_env/linux.sh

sudo apt-get install -y \
  git-buildpackage \
  equivs \
  devscripts

mk-build-deps \
  --install \
  --root-cmd sudo \
  --remove

git clean -f
