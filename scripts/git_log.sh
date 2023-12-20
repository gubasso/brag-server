#!/bin/bash
. ./scripts/git-log-json.sh
git_log_json I-deploy_with_systemd_units | jq -r '.[] | .subject'
