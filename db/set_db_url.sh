#!/bin/bash

if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi;

export DB_URL="${DB_DRIVER}://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}?sslmode=disable";

"$@"
