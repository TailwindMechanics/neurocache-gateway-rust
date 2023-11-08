# syntax=docker/dockerfile:1

# Build stage
FROM rust:1.73.0-slim-bullseye AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

# Run stage
FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/neurocache-gateway /usr/local/bin/neurocache-gateway
CMD ["neurocache-gateway"]
