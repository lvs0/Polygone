FROM rust:1.95-alpine AS builder

RUN apk add --no-cache musl-dev cmake make gcc perl pkgconfig openssl-dev openssl-libs-static
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /build
COPY . .

ENV OPENSSL_STATIC=1
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_VENDORED=true

RUN cargo build --workspace --release --target x86_64-unknown-linux-musl

FROM alpine:latest

RUN apk add --no-cache ca-certificates tzdata
RUN addgroup -S app && adduser -S app -G app

WORKDIR /app

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/polygone /app/polygone

# Script d'auto-réveil intégré
COPY <<'KEEPALIVE' /app/keepalive.sh
#!/bin/sh
while true; do
  sleep 840  # 14 minutes
  wget -qO- "http://localhost:${PORT:-8080}/health" > /dev/null 2>&1
done
KEEPALIVE

RUN chmod +x /app/keepalive.sh

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3   CMD wget -qO- http://localhost:${PORT:-8080}/health || exit 1

EXPOSE 8080

USER app

# Entrypoint : lance le keepalive en arrière-plan et polygone en avant-plan
ENTRYPOINT ["/bin/sh", "-c", "/app/keepalive.sh & exec /app/polygone start"]
