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
# TODO: new steps
# cp binaries to default system-wide locations
#   - /usr/local/bin
# cp units to systemd dirs
# generate and cp env file to systemd

# TODO: this will go to systemd units
# spin_db
# ./target/release/load_db &
# ./target/release/brag-server
