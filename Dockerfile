FROM rust:alpine as builder
WORKDIR /build
COPY Cargo.* ./
COPY src ./src
RUN apk add --no-cache musl-dev \
    && cargo build --release

FROM alpine:latest
LABEL org.opencontainers.image.source https://github.com/setten-io/ecg
COPY --from=builder /build/target/release/ecg /usr/local/bin/ecg
ENTRYPOINT ["ecg"]
