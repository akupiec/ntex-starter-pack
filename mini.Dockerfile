FROM rust:alpine as builder

RUN apk add alpine-sdk musl-dev build-base upx curl

WORKDIR /app

## Copy source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

## Build release binary
RUN cargo build --release
## Pack release binary with UPX (optional)
RUN upx --best --lzma /app/target/release/my-rest-api

FROM scratch

COPY --from=builder /app/target/release/my-rest-api /app

EXPOSE 8080/tcp

ENTRYPOINT ["/app"]
