#!/bin/sh
set -e
. ./scripts/env_vars.sh
sudo -E docker compose up load_db --build
# sudo -E docker compose up --wait
