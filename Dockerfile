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
# Copy the whole workspace (important for local dependencies)
COPY . .

# Build the server package
RUN cargo build --release -p polygone-server

# --- Runtime Stage ---
FROM python:3.11-slim-bookworm

# Install CA certificates
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from the workspace build
COPY --from=builder /usr/src/polygone/target/release/polygone-server /usr/local/bin/polygone-server

# Copy pulse and entrypoint from the server directory
COPY server/pulse.py .
COPY server/entrypoint.sh .
RUN chmod +x entrypoint.sh

# Render expects the app to bind to $PORT
EXPOSE 8080
EXPOSE 4001

ENTRYPOINT ["./entrypoint.sh"]
CMD ["--identity", "/app/identity.p2p", "--listen", "/ip4/0.0.0.0/tcp/4001"]
