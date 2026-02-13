# Invisible Development Environment
FROM rust:1.70-slim AS rust-dev

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Rust tooling
RUN rustup component add rustfmt clippy
RUN cargo install cargo-audit cargo-deny cargo-tarpaulin

# Set working directory
WORKDIR /workspace

# Development stage
FROM rust-dev AS development

# Copy project files
COPY . .

# Build dependencies (cached layer)
RUN cargo fetch

# Expose ports for relay node
EXPOSE 8080 8443

# Default command
CMD ["cargo", "watch", "-x", "test"]

# Production build stage
FROM rust:1.70-slim AS builder

WORKDIR /build

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Build release binary
RUN cargo build --release --bin invisible-relay

# Runtime stage
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 invisible && \
    mkdir -p /var/lib/invisible && \
    chown invisible:invisible /var/lib/invisible

USER invisible
WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/invisible-relay /app/

# Expose relay ports
EXPOSE 8080 8443

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["./invisible-relay", "--health-check"]

# Run relay node
ENTRYPOINT ["./invisible-relay"]
