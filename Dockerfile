FROM rust:slim as builder
WORKDIR /build
COPY Cargo.* ./
COPY src ./src
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/ecg /usr/local/bin/ecg
ENTRYPOINT ["ecg"]
