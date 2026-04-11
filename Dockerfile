# --- Build Stage ---
FROM rust:1-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    clang \
    pkg-config \
    libssl-dev \
    git \
    cmake \
    perl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/polygone
COPY . .

# Build the server binary
RUN cargo build --release --bin polygone-server

# --- Runtime Stage ---
FROM debian:bookworm-slim

# Install CA certificates and networking tools
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from build stage
COPY --from=builder /usr/src/polygone/target/release/polygone-server /usr/local/bin/polygone-server

# Identity directory
RUN mkdir -p /data

# Render expects the app to bind to $PORT
EXPOSE 8080
EXPOSE 4001

ENTRYPOINT ["polygone-server"]
CMD ["--identity", "/data/identity.p2p"]
