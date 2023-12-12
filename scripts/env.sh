#!/bin/sh
# set_env_vars.sh
# TODO
# # Set your environment variables
# export VAR1="value1"
# export VAR2="value2"
# # Add more variables as needed
#
# # Export all variables to a file
# declare -p VAR1 VAR2 > /path/to/env_file

export DEPLOY="dev"
export BRAG_SERVER_DIR="/opt/brag-server"
export DATA_PATH="./data"

export POSTGRES_VERSION='16'
export RUST_VERSION=1.73.0
export PYTHON_VERSION=3.12.0
export CADDY_VERSION=2-alpine

export NET_HOST='127.0.0.1'
export DB_PORT='5432'
export API_PORT='3000'


POSTGRES_PASSWORD="$(gopass show -o Playground/postgre_sample)"
export POSTGRES_PASSWORD
export POSTGRES_USER='postgres'
export POSTGRES_DB='postgres'
export DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${NET_HOST}:${DB_PORT}/${POSTGRES_DB}"


case "$DEPLOY" in
    prod)
        SITE_ADDRESS="gubasso.xyz"
        DOCKER_COMPOSE_WD='Docker/compose.yaml'
        DOCKER_COMPOSE_FILE='Docker/compose.yaml'
        ;;
    dev)
        SITE_ADDRESS="127.0.0.1"
        DOCKER_COMPOSE_FILE='Docker/compose.yaml'
        ;;
    *)
        echo "Error: DEPLOY must be 'prod' or 'dev'"
        exit 1
        ;;
esac

export SITE_ADDRESS
export DOCKER_COMPOSE_FILE='Docker/compose.yaml'
