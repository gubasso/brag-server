#!/bin/sh
export API_PORT='3000'
export DB_HOST='db'
export DATA_PATH='./data'
export PROD_WD='/opt/brag-server'
export RUNNING_ENV_IS_OK_FILE='.runner-dev-ok'
export DOCKER_COMPOSE_FILE='compose.yaml'
export POSTGRES_PASSWORD_GOPASS='Playground/postgre_sample'
export DB_PORT='5432'
export RUST_VERSION='1.73.0'
export PYTHON_VERSION='3.12.0'
export POSTGRES_VERSION='16'
export POSTGRES_USER='postgres'
export POSTGRES_DB='postgres'
POSTGRES_PASSWORD="$(gopass show -o "$POSTGRES_PASSWORD_GOPASS")"
export POSTGRES_PASSWORD
export DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${DB_HOST}:${DB_PORT}/${POSTGRES_DB}"
