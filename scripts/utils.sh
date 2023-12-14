#!/bin/sh
# shellcheck disable=SC1091
. ./env

is_db_running() {
    if [ -z "$(docker-compose -f "$DOCKER_COMPOSE_FILE" ps -q db)" ]; then
        return 1 # false
    else
        return 0 # true
    fi
}

is_asdf_installed() {
    if command -v asdf >/dev/null 2>&1; then
        return 0 # true
    else
        return 1 # false
    fi
}

is_sqlx_installed() {
    if command -v sqlx >/dev/null 2>&1; then
        return 0 # true
    else
        return 1 # false
    fi
}

is_asdf_plugin_installed() {
    program="$1"
    if asdf plugin list | grep -q "$program"; then
        return 0 #true
    else
        return 1 #false
    fi
}

is_asdf_plugin_version_installed() {
    program="$1"
    version="$2"
    if asdf current "$program" | grep -q "$version"; then
        return 0 #true
    else
        return 1 #false
    fi
}
