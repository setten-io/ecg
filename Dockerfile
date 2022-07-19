FROM rust:slim as builder
WORKDIR /build
COPY Cargo.* ./
COPY src ./src
RUN cargo install --path .

FROM debian:buster-slim
LABEL org.opencontainers.image.source https://github.com/setten-io/ecg
COPY --from=builder /usr/local/cargo/bin/ecg /usr/local/bin/ecg
ENTRYPOINT ["ecg"]
