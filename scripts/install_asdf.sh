#!/bin/sh
. ./scripts/utils.sh

set -e

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
