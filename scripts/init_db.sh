#!/bin/bash

set -ex #Debuging and handling errors
export PATH="$HOME/.cargo/bin:$PATH"

#Declare enviroment variables
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"
SKIP_DOCKER="${SKIP_DOCKER:=false}"
DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

# check if sqlx is installed
if ! sqlx --version >/dev/null 2>&1; then
    echo "sqlx is not installed. Please install it."
    exit 1
fi

# check if psql is installed 
if ! psql --version >/dev/null 2>&1; then
    echo "Psql required but not found"
    exit 1
fi

# Check if Docker is installed
if [[ -z "${SKIP_DOCKER}" ]]; then
    if ! docker --version >/dev/null 2>&1; then
        echo "Docker required but not found"
        exit 1
    fi

    echo "docker initialisation"
         docker run \
        -e POSTGRES_USER="$DB_USER" \
        -e POSTGRES_PASSWORD="$DB_PASSWORD" \
        -e POSTGRES_DB="$DB_NAME" \
        -p "$DB_PORT":5432 \
        -d postgres \
        postgres -N 1000

fi

if [[ "$DB_PASSWORD" == "password" ]]; then
  echo "Password is correct"
else
  echo "password is wrong"
  exit 1 
fi

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    echo >&2 "Postgres is still unavailable - sleeping..."
    sleep 1
done

echo "Applying migrations..."

export DATABASE_URL
sqlx database create
sqlx migrate run
echo "Database is ready!"
