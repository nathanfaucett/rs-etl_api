FROM rust:1.70-alpine3.17 as builder

WORKDIR /app

RUN apk add --no-cache musl-dev

COPY . .
RUN cargo install --path .

FROM alpine:3

COPY --from=builder /usr/local/cargo/bin/etl /usr/local/bin/etl

EXPOSE 8080

CMD ["etl"]
