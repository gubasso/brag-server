#!/bin/sh
set -e
. /opt/brag-server/env
export DATABASE_URL
export NET_HOST
export API_PORT
echo "DATABASE_URL: $DATABASE_URL"
sudo -E /opt/brag-server/brag-server
