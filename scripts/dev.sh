#!/bin/sh
set -e
# shellcheck disable=SC1091
. ./env
. ./scripts/utils.sh

case $1 in
    connect_db)
        psql "$DATABASE_URL"
        ;;
    load_db)
        if ! is_db_running; then
            echo "Starting postgresql the database..."
            sudo -E docker compose -f "$DOCKER_COMPOSE_FILE" up --wait
            echo "Starting migration revert and run"
            sqlx migrate revert
            sqlx migrate run
        fi
        echo "Database is up and running."
        echo "Running load_db: connect and load data to db"
        cargo run --bin load_db
        ;;
    clean_db)
        echo "(clean_db) Database: cleaning db data."
        sqlx migrate revert
        sudo -E rm -rf "$DATA_PATH"
        rm -rf "$HOME"/.local/share/brag-server
        ;;
    watch)
        ./run dev load_db
        cargo watch -q -c -w src -x "run --bin brag-server"
        ;;
    dstop)
        echo "(dstop) Docker: stoping containers."
        echo "DOCKER_COMPOSE_FILE: $DOCKER_COMPOSE_FILE"
        echo "DATA_PATH: $DATA_PATH"
        sudo -E docker compose -f "$DOCKER_COMPOSE_FILE" down
        sudo docker kill "$(docker ps -q)"
        ;;
    dclean)
        echo "(dclean) Docker: clean_db and dstop."
        ./run dev clean_db
        ./run dev dstop
        ;;
    dprune)
        echo "Docker: pruning all docker data."
        ./run dev clean_db
        ./run dev dstop
        sudo docker system prune -a -f && sudo docker volume prune -f
        sudo docker network prune -f
        ;;
    *)
        echo 'Error: Invalid input. Please enter one of the valid dev commands.'
        exit 1
        ;;
esac
