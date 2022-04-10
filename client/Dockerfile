# Builder
FROM rust:1.58.0-bullseye as build

WORKDIR /zkp-client
COPY . .

# build client package
RUN cargo build -p zkp-client --release

# The final image
FROM debian:bullseye-slim
LABEL org.opencontainers.image.source https://github.com/suren-m/zkp-rs

RUN mkdir -p /usr/local/bin
COPY --from=build zkp-client/target/release/zkp-client /usr/local/bin

ENTRYPOINT [zkp-client]