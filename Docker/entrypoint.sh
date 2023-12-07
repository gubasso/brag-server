#!/bin/bash
# Read text from secret file
DATABASE_URL="$(cat /run/secrets/database_url)"
export DATABASE_URL
# Call the original entrypoint script
exec "$@"
