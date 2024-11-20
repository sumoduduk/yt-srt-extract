# Stage 1: Chef stage to prepare the recipe
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Stage 2: Planner stage to prepare the recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Builder stage to build the application
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --bin yt-srt-extract

# Stage 4: Runtime stage to run the application
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/yt-srt-extract /usr/local/bin

# Set the binary as the entry point
ENTRYPOINT ["/usr/local/bin/yt-srt-extract"]
