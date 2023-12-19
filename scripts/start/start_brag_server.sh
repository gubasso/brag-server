#!/bin/sh
set -e
# shellcheck disable=SC1091
. ./env_vars.sh
echo "start_brag_server.sh: executing brag-server"
/opt/brag-server/brag-server
