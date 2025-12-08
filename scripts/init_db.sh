#!/usr/bin/env bash
set -x
set -eo pipefail

# --- check dependencies ---
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "❌ Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "❌ Error: sqlx is not installed."
  echo >&2 "👉 Use:"
  echo >&2 " cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
  exit 1
fi

# --- environnement  variables---
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

# --- run Docker ---
if [[ -z "${SKIP_DOCKER}" ]]
then
  echo "🚀 Starting PostgreSQL container with Docker..."
  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
fi

# --- waiting postgres to be ready ---
export PGPASSWORD="${DB_PASSWORD}"

echo "⏳ Waiting for PostgreSQL to become available..."
until docker exec -it $(docker ps -q --filter ancestor=postgres) \
  psql -U "${DB_USER}" -d "postgres" -c '\q' 2>/dev/null; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "✅ Postgres is up and running on port ${DB_PORT}!"

# --- integration and Configuration ---
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

echo "🛠️  Creating database if not exists..."
sqlx database create

echo "📦 Running database migrations..."
sqlx migrate run

>&2 echo "PostgreSQL has been migrated successfully and is ready to go!"
