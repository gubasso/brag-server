#!/bin/sh
case "$RUNNING_ENV" in
    dev)
        NET_HOST='127.0.0.1'
        ;;
    prod)
        # NET_HOST='gubasso.xyz'
        NET_HOST='127.0.0.1'
        ;;
    *)
        echo 'Error: Invalid input. Please enter "dev" or "prod"'
        exit 1
        ;;
esac
export PROD_WD='/opt/brag-server'
export NET_HOST
export RUNNING_ENV_IS_OK_FILE='.runner-dev-ok'
export POSTGRES_PASSWORD_GOPASS='Playground/postgre_sample'
export DB_PORT='5432'
export POSTGRES_USER='postgres'
export POSTGRES_DB='postgres'
POSTGRES_PASSWORD="$(gopass show -o "$POSTGRES_PASSWORD_GOPASS")"
export POSTGRES_PASSWORD
export DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${NET_HOST}:${DB_PORT}/${POSTGRES_DB}"
