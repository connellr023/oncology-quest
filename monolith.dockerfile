# Build stage for the web app
FROM node:16 AS web-builder
WORKDIR /web-tmp

# Copy package.json and package-lock.json to the working directory
COPY ./oncology-quest-web/package*.json ./

# Install dependencies
RUN npm install

# Copy the current directory contents into the container
COPY ./oncology-quest-web .

# Build the app for production
RUN npm run build

# Final stage
FROM fedora:35

# Set the environment variables
ENV HOST_IP=0.0.0.0
ENV HOST_PORT=80

WORKDIR /prod

# Copy the compiled binary
# Ensure the binary is built with the following command:
# cargo build --release --features "production monolith"
COPY ./oncology-quest-api/target/release/oncology-quest-api /prod/api

# Copy the built web app from the web build stage
COPY --from=web-builder /web-tmp/dist /prod/dist

# Expose the port
EXPOSE 80

# Run the binary
CMD ["./api"]