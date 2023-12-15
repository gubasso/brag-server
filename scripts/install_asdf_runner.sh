#!/bin/sh
set -e
program="$1"
version="$2"
. ./scripts/utils.sh

echo "ASDF install runner: $program v.$version."

if is_asdf_plugin_installed "$program" && is_asdf_plugin_version_installed "$program" "$version"; then
    echo "ASDF install runner: exiting."
    exit 0
fi

if ! is_asdf_plugin_installed "$program"; then
    echo "ASDF: installing $program plugin."
    asdf plugin add "$program"
fi

if ! is_asdf_plugin_version_installed "$program" "$version"; then
    echo "ASDF: installing $program v.$version ."
    asdf install "$program" "$version"
    asdf global "$program" "$version"
    asdf local "$program" "$version"
fi
echo "ASDF: $program v.$version installed."
