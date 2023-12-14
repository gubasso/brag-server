#!/bin/sh
. ./utils.sh

set -e

if is_sqlx_installed; then
    exit 0
fi

echo "Installing SQLx-CLI (with cargo install)."
cargo install sqlx-cli --no-default-features --features native-tls,postgres
asdf reshim
