#!/bin/sh
. ./.env

is_db_running() {
    if [ -z "$(docker-compose -f "$DOCKER_COMPOSE_FILE" ps -q db)" ]; then
        return 1 # false
    else
        return 0 # true
    fi
}

migrate() {
    echo "Starting migration revert and run"
    sqlx migrate revert && \
        sqlx migrate run
}

spin_db() {
    if ! is_db_running; then
        echo "Starting postgresql the database..."
        sudo docker compose -f "$DOCKER_COMPOSE_FILE" up --wait
        migrate
    fi
    echo "Database is up and running."
}

is_asdf_installed() {
    if command -v asdf >/dev/null 2>&1; then
        return 0 # true
    else
        return 1 # false
    fi
}

install_asdf() {
    if ! is_asdf_installed; then
        echo "Installing ASDF."
        sudo apt-get install -y curl git
        git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.13.1
        echo '. "'"$HOME"'/.asdf/asdf.sh"' >> ~/.bashrc
        echo '. "'"$HOME"'/.asdf/completions/asdf.bash"' >> ~/.bashrc
        # shellcheck disable=SC1091
        . "$HOME/.bashrc"
    fi
    echo "ASDF: is installed and ready."
}

install_runner() {
    program="$1"
    version="$2"
    echo "ASDF: installing $program v.$version ."
    asdf plugin add "$program"
    asdf install "$program" "$version"
    asdf global "$program" "$version"
    asdf local "$program" "$version"
    echo "ASDF: $program v.$version installed."
}

install_runners() {
    install_asdf
    install_runner "rust" "$RUST_VERSION"
    install_runner "python" "$PYTHON_VERSION"
}
