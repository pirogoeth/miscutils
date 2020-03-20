#!/usr/bin/env bash

set -eo pipefail

echo "============"
rustup show active-toolchain
echo "============"
echo

retdir=$(pwd)
mkdir ./output
cd ./source
cargo build --target-dir ../output/ "${BUILD_ARGS}"
cd ${retdir}