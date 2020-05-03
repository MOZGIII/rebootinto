#!/bin/bash
set -euo pipefail

sudo apt-get -qq update

sudo apt-get install -y \
  libgtk-3-dev \
  python3 \
  python3-pip \
  python3-setuptools \
  python3-wheel \
  ninja-build \
  llvm

sudo pip3 install meson
