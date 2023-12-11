#!/bin/sh
. ./.env
. ./scripts/utils.sh

case $1 in
    migrate)
        echo "Starting migration revert and run"
        sqlx migrate revert && \
            sqlx migrate run
        ;;
    connect_db)
        psql "$DATABASE_URL"
        ;;
    load_db)
        load_db
        ;;
    clean_db)
        sqlx migrate revert
        sudo rm -rf "$DATA_PATH" ../"$DATA_PATH"
        rm -rf "$HOME"/.local/share/brag-server
        ;;
    watch)
        load_db
        cargo watch -q -c -w src -x "run --bin brag-server"
        ;;
    dstop)
        docker compose -f "$DOCKER_COMPOSE_FILE" down
        docker kill "$(docker ps -q)"
        ;;
    dclean)
        ./run dev clean_db
        ./run dev dstop
        ;;
    dprune)
        ./run dev clean_db
        ./run dev dstop
        docker system prune -a -f && docker volume prune -f
        docker network prune -f
        ;;
    setup)
        install_runners
        asdf install
        asdf reshim
        pip install pre-commit
        pre-commit install
        pre-commit autoupdate
        pre-commit run --all-files
        ;;
    *)
        echo 'Error: Invalid input. Please enter one of the valid dev steps.'
        exit 1
        ;;
esac
