#!/bin/bash

set -e  # Stop script on error

set -x #Debuging purpose 
# Declare environment variables
DB_USER=postgres
DB_PASSWORD=password
DB_DATABASE=postgres
DB_CONTAINER_NAME=postgres_db
DATABASE_URL="postgres://$DB_USER:$DB_PASSWORD@localhost:5432/$DB_DATABASE"

# Check if Docker is installed
if ! docker --version > /dev/null 2>&1; then
    echo "Docker is not installed. Please install it."
    exit 1
fi

# Check if sqlx is installed
if ! sqlx --version > /dev/null 2>&1; then
    echo "sqlx is not installed. Please install it."
    exit 1
fi

# Start PostgreSQL Docker container
echo "Starting PostgreSQL container..."
docker run --name "$DB_CONTAINER_NAME" \
    -e POSTGRES_USER="$DB_USER" \
    -e POSTGRES_PASSWORD="$DB_PASSWORD" \
    -e POSTGRES_DB="$DB_DATABASE" \
    -p 5432:5432 \
    -d postgres

# Wait for the database to initialize
echo "Initialising database" 
sleep 10 # Adjust the sleep time if needed

# Apply migrations
echo "Applying migrations..."

export $DATABASE_URL
sqlx migrate run

echo "Database is ready!"
