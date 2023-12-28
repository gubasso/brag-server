#!/bin/sh
DATABASE_URL="$(cat /run/secrets/database_url)"
export DATABASE_URL
echo "from brag-server entrypoint"
echo "before exec ./brag-server"
echo "NET_HOST: $NET_HOST"
echo "API_PORT: $API_PORT"
exec ./brag-server
