# Build stage - use latest Rust
FROM rust:latest AS builder

WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Copy source tree
COPY src ./src
COPY migrations ./migrations
COPY build.rs ./
COPY admin ./admin

# Get git commit SHA for version (or use "main" if not available)
ARG VERSION=main
ENV VERSION=$VERSION

# Build for release
RUN cargo build --release

# Runtime stage - use Ubuntu 24.04 for GLIBC_2.39 compatibility
FROM ubuntu:24.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    sqlite3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/egghead_service /app/egghead_service

# Copy migrations
COPY migrations ./migrations

EXPOSE 3000

ENV DATABASE_URL=sqlite:./egghead.db
ENV LISTEN_ADDR=0.0.0.0:3000

ENTRYPOINT ["/app/egghead_service"]
