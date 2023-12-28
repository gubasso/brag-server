#!/bin/sh
DATABASE_URL="$(cat /run/secrets/database_url)"
export DATABASE_URL
exec ./load_db
