#!/bin/sh
set -e
# shellcheck disable=SC1091
. "$PROD_WD"/env
sys_user="$1"

if [ -z "$sys_user" ]; then
    echo "run prod must have a sys_user"
    exit 1
fi

sudo systemctl enable spin_db.service --now
sudo systemctl enable "load_db@$sys_user.service" --now
sudo systemctl enable "brag-server@$sys_user.service" --now
