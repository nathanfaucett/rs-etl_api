FROM rust:1.69-alpine3.17 as builder

WORKDIR /app

RUN apk add --no-cache musl-dev

COPY . .
RUN cargo install --path .

FROM alpine:3

COPY --from=builder /usr/local/cargo/bin/etl_api /usr/local/bin/etl_api

EXPOSE 8080

CMD ["etl_api"]
