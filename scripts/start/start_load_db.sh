#!/bin/sh
set -e
# shellcheck disable=SC1091
. ./env_vars.sh
echo "start_load_db.sh: executing load_db"
/opt/brag-server/load_db
