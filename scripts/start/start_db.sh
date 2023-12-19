#!/bin/sh
set -e
# shellcheck disable=SC1091
. ./env_vars.sh
echo "start_db.sh: running docker"
sudo docker compose up --wait
