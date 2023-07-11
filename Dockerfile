FROM docker.io/rust:1-slim as BUILDER

RUN apt-get update && apt-get install -y pkg-config libpq-dev openssl libssl-dev libclang-dev llvm

WORKDIR /app

ADD . .

RUN --mount=type=cache,target=/app/target cargo install --debug --locked --root install --path .

FROM gcr.io/distroless/cc

COPY --from=BUILDER /usr/lib /usr/lib
COPY --from=BUILDER /lib /lib
COPY --from=BUILDER /app/install /app/install

CMD ["/app/install/bin/chatterbox"]