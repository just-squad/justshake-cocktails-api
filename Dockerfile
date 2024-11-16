# Step 1: Compute a recipe file
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Step 2: Compute a recipe file
FROM chef as planner
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

# Step 4: Build the binary
FROM chef as builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN rustup target add x86_64-unknown-linux-musl
ENV TARGET_CC=x86_64-linux-musl-gcc
RUN apt-get update && apt-get install -y \
  gcc-aarch64-linux-gnu musl-tools musl-dev libssl-dev perl cmake make \
  && rm -rf /var/lib/apt/lists/*

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json --features vendored-openssl

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl --features vendored-openssl

# Step 5: Create the final image with binary and deps
FROM alpine:3.17
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/justshake-cocktails-api .
ENTRYPOINT ["./justshake-cocktails-api"]

