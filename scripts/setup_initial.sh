#!/bin/sh
set -e

if [ -f "$RUNNING_ENV_IS_OK_FILE" ] && [ "$RUNNING_ENV" = 'dev' ]; then
    exit 0
fi

echo "## Initial Setup"


{
    echo "export DATA_PATH='./data'"
    echo "export POSTGRES_VERSION='16'"
    echo "export RUST_VERSION='1.73.0'"
    echo "export PYTHON_VERSION='3.12.0'"
    echo "export NET_HOST=$NET_HOST"
    echo "export API_PORT='3000'"
    echo "export DB_PORT=$DB_PORT"
    echo "export POSTGRES_PASSWORD=$POSTGRES_PASSWORD"
    echo "export POSTGRES_USER=$POSTGRES_USER"
    echo "export POSTGRES_DB=$POSTGRES_DB"
    echo "export DATABASE_URL=$DATABASE_URL"
} > ./env

# shellcheck disable=SC1091
. ./env
./scripts/install_asdf.sh
./scripts/install_asdf_runner.sh "rust" "$RUST_VERSION"
./scripts/install_sqlx.sh
if [ "$RUNNING_ENV" = 'dev' ]; then
    ./scripts/install_asdf_runner.sh "python" "$PYTHON_VERSION"
    asdf install
    asdf reshim
    pip install pre-commit
    pre-commit install
    pre-commit autoupdate
    pre-commit run --all-files
fi

if [ "$RUNNING_ENV" = 'prod' ]; then
    sudo -E mkdir -p "$PROD_WD"
    sudo -E cp "$DOCKER_COMPOSE_FILE" "$PROD_WD"
    cargo build --bin load_db --release
    cargo build --bin brag-server --release
    # cp binaries to default system-wide locations
    sudo -E cp ./target/release/load_db "$PROD_WD"
    sudo -E cp ./target/release/brag-server "$PROD_WD"
    # cp units to systemd dirs
    sudo -E cp ./units/* /etc/systemd/system
fi

touch "$RUNNING_ENV_IS_OK_FILE"

echo "## Initial Setup: environment is setup"
