#!/bin/sh
set -e
program="$1"
version="$2"
. ./scripts/utils.sh

is_asdf_plugin_installed "$program"
is_program_installed="$?"
is_asdf_plugin_version_installed "$program" "$version"
is_version_installed="$?"

if "$is_program_installed" && "$is_version_installed"; then
    exit 0
fi

if ! "$is_program_installed"; then
    echo "ASDF: installing $program plugin."
    asdf plugin add "$program"
fi

if ! "$is_version_installed"; then
    echo "ASDF: installing $program v.$version ."
    asdf install "$program" "$version"
    asdf global "$program" "$version"
    asdf local "$program" "$version"
fi
echo "ASDF: $program v.$version installed."
