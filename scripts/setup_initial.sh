#!/bin/sh
set -e

if [ -f "$RUNNING_ENV_IS_OK_FILE" ] && [ "$RUNNING_ENV" = 'dev' ]; then
    exit 0
fi

echo "## Initial Setup"

./scripts/install_asdf.sh
./scripts/install_asdf_runner.sh "rust" "$RUST_VERSION"
./scripts/install_sqlx.sh
if [ "$RUNNING_ENV" = 'dev' ]; then
    ./scripts/install_asdf_runner.sh "python" "$PYTHON_VERSION"
    sudo cp ./scripts/commit-json /usr/local/bin/commit-json
    asdf install
    asdf reshim
    pip install pre-commit
    pre-commit install
    pre-commit autoupdate
    # pre-commit run --all-files
fi

if [ "$RUNNING_ENV" = 'prod' ]; then
    echo "Setting up production."
    echo "mkdir $PROD_WD."
    sudo mkdir -p "$PROD_WD"
    echo "Copying $DOCKER_COMPOSE_FILE"
    sudo cp "$DOCKER_COMPOSE_FILE" "$PROD_WD"
    echo "Copying env_vars.sh"
    sudo cp ./scripts/env_vars.sh "$PROD_WD"
    echo "Building binaries"
    cargo build --bin load_db --release
    cargo build --bin brag-server --release
    echo "Copying binaries to $PROD_WD"
    # cp binaries to default system-wide locations
    sudo cp ./target/release/load_db "$PROD_WD"
    sudo cp ./target/release/brag-server "$PROD_WD"
    echo "Copying brag-server-toml to $PROD_WD"
    sudo cp brag-server.toml "$PROD_WD"
    # cp units to systemd dirs
    echo "Copying sytemd units to /etc/systemd/system"
    sudo cp ./units/* /etc/systemd/system
    echo "Copying start scripts $PROD_WD"
    sudo cp ./scripts/start/* "$PROD_WD"
fi

touch "$RUNNING_ENV_IS_OK_FILE"

echo "## Initial Setup: environment is setup"
