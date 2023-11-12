# Build stage
FROM rust:1.73.0-slim-bullseye AS builder
WORKDIR /usr/src/myapp
COPY . .

# Install necessary packages including OpenSSL, its development headers, and libsasl2 development package
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev ca-certificates build-essential libsasl2-dev && \
    rm -rf /var/lib/apt/lists/*

# Build your application
RUN cargo install --path .

# Run stage
FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/neurocache-gateway /usr/local/bin/neurocache-gateway

# Install runtime dependencies including OpenSSL and libsasl2
RUN apt-get update && \
    apt-get install -y ca-certificates libssl1.1 libsasl2-2 && \
    rm -rf /var/lib/apt/lists/*

CMD ["neurocache-gateway"]
