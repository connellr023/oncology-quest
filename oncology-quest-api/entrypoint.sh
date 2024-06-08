#!/bin/bash

set -e
cd /usr/src/oncology-quest-api

# Wait for the database to be ready
until PGPASSWORD=password psql -h "db" -p "5432" -U "postgres" -c '\q'; do
  echo "Postgres is unavailable - Sleeping..."
  sleep 1
done

echo "Postgres is up - Continuing..."

# Run migrations
sqlx migrate run --database-url "postgres://postgres:password@db:5432/oncology-quest"
echo "Migrations complete"

# Insert a test admin user
# Username: admin
# Password: complexpass123
psql "postgres://postgres:password@db:5432/oncology-quest" \
  -c "INSERT INTO users (username, name, email, password_reset_timestamp, is_admin, salt, password) VALUES ('admin', 'Admin Account', 'testadmin@test.com', '2024-06-07 13:06:49.566422-06', true, 8999838332277863429, '\$2b\$12\$3X1QEyLQ.BT8DGBZeG5nqOSwku7cNRVsjbNdNT/byOpHTEJPB5M5y');"

psql "postgres://postgres:password@db:5432/oncology-quest" \
  -c "SELECT * FROM users;"

echo "Admin user created"

# Run unit tests
cargo test

# Start the application
exec cargo run