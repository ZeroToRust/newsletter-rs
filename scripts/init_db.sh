#!/bin/bash
set -ex # hanldling errors and Debuging purpose 
# Declare environment variables

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"
SKIP_DOCKER=${SKIP_DOCKER:=false}
DATABASE_URL="postgres://$DB_USER:$DB_PASSWORD@localhost:5432/$DB_DATABASE"

# Check if Docker is installed
if [ "$SKIP_DOCKER" = "false" ]; then
    if ! docker --version > /dev/null 2>&1; then
        echo "Docker required but not found"
        exit 1
    fi

    echo "docker initialisation"
        docker run --name "$DB_NAME"_container \
        -e POSTGRES_USER="$DB_USER" \
        -e POSTGRES_PASSWORD="$DB_PASSWORD" \
        -e POSTGRES_DB="$DB_NAME" \
        -p "$DB_PORT":5432 \
        -d postgres
# Wait for the database to initialize
 fi 
# else check if psql is intalled
if  ! psql --version > /dev/null 2>&1; then 
         echo "Psql required but not found"
         exit 1
     fi

if ! psql -U $DB_USER -h $DB_HOST -d postgres -tc "SELECT 1 FROM pg_database WHERE datname = '$DB_NAME';" | grep -q 1; then
    psql -U $DB_USER -h $DB_HOST -d postgres -c "CREATE DATABASE $DB_NAME;"
fi

# Check if sqlx is installed
if ! sqlx --version > /dev/null 2>&1; then
    echo "sqlx is not installed. Please install it."
    exit 1
fi

# Start PostgreSQL Docker contain

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping..."
    sleep 1
done

echo "Applying migrations..."

sqlx database create 
export $DATABASE_URL
sqlx migrate run
echo "Database is ready!"
