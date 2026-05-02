# ============================================================
# STAGE 1 — Builder
# ============================================================
FROM rust:1.90 AS builder

WORKDIR /app

# Copy dependency files first (layer caching)
COPY Cargo.toml Cargo.lock ./

# Create dummy main to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real source code
COPY src ./src

# Build the actual binary
RUN touch src/main.rs && cargo build --release

# ============================================================
# STAGE 2 — Runtime (small image)
# ============================================================
FROM debian:bookworm-slim

WORKDIR /app

# Install SSL certificates (needed for HTTPS calls)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/rust_retrieval_engine .

EXPOSE 3000

CMD ["./rust_retrieval_engine"]
