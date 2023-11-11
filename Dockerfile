# syntax=docker/dockerfile:1

# Build stage
FROM rust:1.73.0-slim-bullseye AS builder
WORKDIR /usr/src/myapp
COPY . .

# Install necessary packages
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Build your application
RUN cargo install --path .

# Run stage
FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/neurocache-gateway /usr/local/bin/neurocache-gateway

# Install ca-certificates in runtime image
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

CMD ["neurocache-gateway"]
