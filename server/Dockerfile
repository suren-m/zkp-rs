# Builder
FROM rust:1.58.0-bullseye as build

WORKDIR /app
COPY . .

# build server package
RUN cargo build -p zkp-server --release

# The final image
FROM debian:bullseye-slim
LABEL org.opencontainers.image.source https://github.com/suren-m/zkp-rs

RUN mkdir -p /usr/local/bin
COPY --from=build app/target/release/zkp-server /usr/local/bin/

ENTRYPOINT ["zkp-server"]