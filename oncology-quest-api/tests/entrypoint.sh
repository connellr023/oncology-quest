#!/bin/bash

set -e
cd /usr/src/oncology-quest-api

# Wait for the database to be ready
until PGPASSWORD=password psql -h "db" -p "5432" -U "postgres" -c '\q'; do
  echo "Postgres is unavailable - sleeping"
  sleep 1
done

echo "Postgres is up - executing command"

# Run migrations
sqlx migrate run --database-url "postgres://postgres:password@db:5432/oncology-quest"

# Run tests
cargo test --no-run

# Start the application
exec cargo run --release