# syntax=docker/dockerfile:1

# Build stage
FROM rust:1.73.0-slim-bullseye AS builder
WORKDIR /usr/src/myapp
COPY . .

# Install pkg-config and OpenSSL development files
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# This command builds your application
RUN cargo install --path .

# Run stage
FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/neurocache-gateway /usr/local/bin/neurocache-gateway

# The command to run your application
CMD ["neurocache-gateway"]
