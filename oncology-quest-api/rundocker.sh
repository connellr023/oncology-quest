#!/bin/bash

# Change to the directory where the script is located
cd "$(dirname "$0")"

# Start the docker containers for testing
docker-compose -f docker-compose.test.yml up --build

# Stop the docker containers
docker-compose down