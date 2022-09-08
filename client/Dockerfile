# Step 1: Compute a recipe file
FROM rust:1.63-bullseye as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Step 2: Cache project dependencies
FROM rust:1.63-bullseye as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Step 3: Build the binary
FROM rust:1.63-bullseye as builder
WORKDIR /app
COPY . .
# Copy over the cached dependencies from above
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build -p zkp-client --release

# Step 4:
# Output Image
FROM debian:bullseye-slim as runtime
LABEL org.opencontainers.image.source https://github.com/suren-m/zkp-rs
COPY --from=builder /app/target/release/zkp-client /usr/local/bin/
ENTRYPOINT ["zkp-client"]


