#!/usr/bin/env bash

set -eo pipefail

echo "============"
rustup show active-toolchain
echo "============"
echo

if [[ ! -z "${INSTALL_PACKAGES}" ]]
then
    apk add --no-cache ${INSTALL_PACKAGES}
fi

retdir=$(pwd)
cd ./source
cargo test
cd ${retdir}