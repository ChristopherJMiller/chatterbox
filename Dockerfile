FROM docker.io/rust:alpine as BUILDER

WORKDIR /rust

RUN apk add --update build-base

ADD . .

RUN --mount=type=cache,target=/app/target cargo install --locked --root install --path .

FROM gcr.io/distroless/cc

COPY --from=BUILDER /usr/lib /usr/lib
COPY --from=BUILDER /lib /lib
COPY --from=BUILDER /app/install /app/install

CMD ["/app/install/bin/chatterbox"]