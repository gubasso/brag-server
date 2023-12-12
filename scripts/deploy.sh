#!/bin/sh
. ./.env
. ./scripts/utils.sh

set -e

install_asdf
install_runner "rust" "$RUST_VERSION"
cargo install sqlx-cli --no-default-features --features native-tls,postgres
asdf reshim
cargo build --bin load_db --release
cargo build --bin brag-server --release
spin_db
./target/release/load_db &
./target/release/brag-server
