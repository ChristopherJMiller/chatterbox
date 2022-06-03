FROM docker.io/rust:alpine as BUILDER

WORKDIR /rust

RUN apk add --update build-base

ADD . .

RUN --mount=type=cache,target=/rust/target cargo run

FROM docker.io/nginx:1.21.3-alpine

COPY --from=BUILDER /rust/out /usr/share/nginx/html
ADD nginx.conf /etc/nginx/nginx.conf
