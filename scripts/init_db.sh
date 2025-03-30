#!/bin/bash

set -e  # Stop script on error

# Load environment variables
source .env

# Create the database (if not exists)
echo "Creating database..."

if psql -U "$PGUSER" -h "$PGHOST" -d postgres -tc "SELECT 1 FROM  pg_database WHERE datname = '$PGDATABASE';"| grep -q 1;  then  
       echo "oops it seems as if your database have already been created"
else 
    psql -U "$PGUSER" -h "$PGHOST" -d postgres -c "CREATE DATABASE $PGDATABASE;"
fi
# Apply migrations
echo "Applying migrations..."
sqlx migrate run

echo "Database is ready!"