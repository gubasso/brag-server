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
    pre-commit run --all-files || true
    touch "$RUNNING_ENV_IS_OK_FILE"
fi

echo "## Initial Setup: environment is setup"
