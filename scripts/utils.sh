#!/bin/sh
. ./.env

is_db_running() {
    if [ -z "$(docker-compose -f "$DOCKER_COMPOSE_FILE" ps -q db)" ]; then
        return 1 # false
    else
        return 0 # true
    fi
}

spin_db() {
    if ! is_db_running; then
        echo "Starting postgresql the database..."
        docker compose -f "$DOCKER_COMPOSE_FILE" up --wait
    fi
    echo "Database is up and running."
}

load_db() {
    spin_db
    echo "Running load_db: connect and load data to db"
    cargo run --bin load_db
}

is_asdf_installed() {
    if command -v asdf >/dev/null 2>&1; then
        return 1 # false
    else
        return 0
    fi
}

install_asdf() {
    if is_asdf_installed; then
        exit 0
    fi
    echo "Installing ASDF."
    sudo apt-get install -y curl git
    git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.13.1
    echo '. "'"$HOME"'/.asdf/asdf.sh"' >> ~/.bashrc
    echo '. "'"$HOME"'/.asdf/completions/asdf.bash"' >> ~/.bashrc
    # shellcheck disable=SC1091
    . "$HOME/.bashrc"
}

install_runner() {
    program="$1"
    version="$2"
    asdf plugin add "$program"
    asdf install "$program" "$version"
    asdf global "$program" "$version"
    asdf local "$program" "$version"
}

install_runners() {
    install_asdf
    install_runner "rust" "$RUST_VERSION"
    install_runner "python" "$PYTHON_VERSION"
}
