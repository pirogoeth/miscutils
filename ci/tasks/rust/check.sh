#!/usr/bin/env bash

set -eo pipefail

echo "============"
rustup show active-toolchain
echo "============"
echo

retdir=$(pwd)
cd ./source
cargo check
cd ${retdir}