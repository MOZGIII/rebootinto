#!/bin/bash
set -euo pipefail

# Build upon the build environment.
.github/scripts/build_env/linux.sh

sudo apt-get install -y \
  git-buildpackage \
  equivs \
  devscripts

(
  EQUIVS_TMP="$(mktemp -d)"
  trap 'rm -rf "$EQUIVS_TMP"' EXIT

  REPO_ROOT="$(pwd)"
  cd "$EQUIVS_TMP"
  equivs-build "$REPO_ROOT/.github/scripts/packaging_env/linux/equivs/cargo"
  sudo dpkg -i cargo_1.0_all.deb
)

mk-build-deps \
  --install \
  --root-cmd sudo \
  --remove

git clean -f
