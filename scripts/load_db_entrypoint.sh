#!/bin/sh
DATABASE_URL="$(cat /run/secrets/database_url)"
export DATABASE_URL
echo "from entrypoint"
echo "Executing load_db..."
exec ./load_db
