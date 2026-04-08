# ============================================================
# Stage 1: Build the Rust indexer binary
# ============================================================
FROM rust:1.85-slim-bookworm AS rust-builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy workspace Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY programs/shared/Cargo.toml programs/shared/Cargo.toml
COPY programs/content-registry/Cargo.toml programs/content-registry/Cargo.toml
COPY programs/analysis-marketplace/Cargo.toml programs/analysis-marketplace/Cargo.toml
COPY programs/model-registry/Cargo.toml programs/model-registry/Cargo.toml
COPY programs/worker-registry/Cargo.toml programs/worker-registry/Cargo.toml
COPY programs/rewards/Cargo.toml programs/rewards/Cargo.toml
COPY worker/dfpn-worker/Cargo.toml worker/dfpn-worker/Cargo.toml
COPY indexer/Cargo.toml indexer/Cargo.toml

# Create dummy source files so cargo can resolve deps
RUN mkdir -p programs/shared/src && echo "pub fn dummy() {}" > programs/shared/src/lib.rs && \
    mkdir -p programs/content-registry/src && echo "pub fn dummy() {}" > programs/content-registry/src/lib.rs && \
    mkdir -p programs/analysis-marketplace/src && echo "pub fn dummy() {}" > programs/analysis-marketplace/src/lib.rs && \
    mkdir -p programs/model-registry/src && echo "pub fn dummy() {}" > programs/model-registry/src/lib.rs && \
    mkdir -p programs/worker-registry/src && echo "pub fn dummy() {}" > programs/worker-registry/src/lib.rs && \
    mkdir -p programs/rewards/src && echo "pub fn dummy() {}" > programs/rewards/src/lib.rs && \
    mkdir -p worker/dfpn-worker/src && echo "fn main() {}" > worker/dfpn-worker/src/main.rs && \
    mkdir -p indexer/src && echo "fn main() {}" > indexer/src/main.rs

# Pre-build dependencies (cached layer)
RUN cargo build --release --package dfpn-indexer 2>/dev/null || true

# Copy actual source code
COPY programs/ programs/
COPY indexer/ indexer/

# Build the indexer
RUN cargo build --release --package dfpn-indexer

# ============================================================
# Stage 2: Build the Vue.js frontend
# ============================================================
FROM node:20-alpine AS node-builder

WORKDIR /app

COPY dashboard/package.json dashboard/package-lock.json ./
RUN npm ci

COPY dashboard/ .
RUN npm run build

# ============================================================
# Stage 3: Runtime image
# ============================================================
FROM nginx:1.27-bookworm

# Install supervisord
RUN apt-get update && apt-get install -y --no-install-recommends supervisor && rm -rf /var/lib/apt/lists/*

# Copy indexer binary
COPY --from=rust-builder /build/target/release/dfpn-indexer /usr/local/bin/dfpn-indexer

# Copy frontend build
COPY --from=node-builder /app/dist /usr/share/nginx/html

# Copy config files
COPY dashboard/nginx.conf /etc/nginx/conf.d/default.conf
COPY dashboard/supervisord.conf /etc/supervisord.conf
COPY dashboard/entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh && \
    mkdir -p /data/indexes && \
    rm -f /etc/nginx/conf.d/default.conf.bak

EXPOSE 80

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s \
    CMD curl -f -s http://localhost/api/health || exit 1

ENTRYPOINT ["/entrypoint.sh"]
