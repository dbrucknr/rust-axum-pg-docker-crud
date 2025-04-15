#!/bin/sh
set -e

# Set default host and port if not provided
DB_HOST=${DATABASE_HOST:-postgres-database}
DB_PORT=${DATABASE_PORT:-5432}

echo "Waiting for PostgreSQL at ${DB_HOST}:${DB_PORT} to be ready..."
while ! nc -z $DB_HOST $DB_PORT; do
    sleep 1
    echo "Still waiting for PostgreSQL..."
done

echo "PostgreSQL is up - running migrations now."
# diesel setup
# diesel migration generate main
diesel migration run

# Start the application using watchexec
exec watchexec --restart --watch src --exts rs cargo run