#!/bin/bash

set -e
cd /usr/src/oncology-quest-api-integration-tests

# Wait for the server to be ready
until wget -qO- "http://api:8000/api/healthcheck" | grep -q "OK"; do
  echo "Server is unavailable - Sleeping..."
  sleep 1
done

echo "Server is up - Continuing..."

# Run the tests
cargo test