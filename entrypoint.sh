#!/bin/bash
# Read password from secret file
POSTGRES_PASSWORD="$(cat /run/secrets/postgres_password)"
export POSTGRES_PASSWORD
# Call the original entrypoint script
exec docker-entrypoint.sh postgres
