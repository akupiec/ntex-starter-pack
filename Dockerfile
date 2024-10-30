FROM rust:alpine as builder

RUN apk add --no-cache alpine-sdk musl-dev build-base upx curl openssl-dev sqlite
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo install sqlx-cli --features native-tls,sqlite

WORKDIR /app

## Copy source code
COPY Cargo.toml Cargo.lock .env ./
COPY src ./src
COPY migrations ./migrations

## Create database
RUN sqlx database create
RUN sqlx migrate run

## Build release binary
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo build --release
## Pack release binary with UPX (optional)
RUN upx --best --lzma /app/target/release/my-rest-api

FROM alpine

RUN apk add --no-cache libgcc

COPY --from=builder /app/target/release/my-rest-api /app

HEALTHCHECK --start-period=1s CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1
EXPOSE 8080/tcp

ENTRYPOINT ["/app"]
