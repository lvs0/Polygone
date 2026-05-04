FROM alpine:3.19

RUN apk add --no-cache wget

RUN addgroup -S app && adduser -S app -G app

WORKDIR /app

COPY polygone /app/polygone
COPY health_server /app/health_server
COPY --from=builder /usr/src/polygone/target/release/webui /app/webui
COPY keepalive.sh /app/keepalive.sh
COPY entrypoint.sh /app/entrypoint.sh

RUN chmod +x /app/entrypoint.sh /app/keepalive.sh

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget -qO- http://localhost:8080/health || exit 1

EXPOSE 8080
EXPOSE 9050

USER app

ENTRYPOINT ["/app/entrypoint.sh"]
