#!/bin/sh
. ./.env
. ./scripts/utils.sh

case $1 in
    deploy)
        ./scripts/deploy.sh
        ;;
    *)
        echo 'Run Prod Error: Invalid input. Please enter one of the valid steps.'
        exit 1
        ;;
esac
