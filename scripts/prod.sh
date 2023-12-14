#!/bin/sh
set -e
# shellcheck disable=SC1091
sudo -E . "$PROD_WD"/env
sys_user="$1"

sudo systemctl enable spin_db.service --now
sudo systemctl enable "load_db@$sys_user.service" --now
sudo systemctl enable "brag-server@$sys_user.service" --now
