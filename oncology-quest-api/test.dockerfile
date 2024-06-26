# Container for setting up test environment for the API
FROM rust:1.78 AS builder

WORKDIR /usr/src/oncology-quest-api

# Install PostgreSQL client
RUN apt-get update && apt-get install -y libpq-dev && apt-get install -y postgresql-client

# Install SQLx CLI
RUN cargo install sqlx-cli

# Copy over manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Copy source tree
COPY ./src ./src

# Copy migrations
COPY ./migrations ./migrations

# Copy entrypoint script
COPY ./entrypoint.sh ./entrypoint.sh
RUN chmod +x ./entrypoint.sh

# Run the entrypoint script
ENTRYPOINT ["./entrypoint.sh"]