# Stage 1: Build Rust backend
FROM rust:1-slim-bookworm AS rust-builder
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
RUN cargo build --release -p notes-server-axum

# Stage 2: Build Vue frontend
FROM node:22-slim AS node-builder
WORKDIR /build
COPY app/package.json app/package-lock.json ./
RUN npm ci
COPY app/ ./
# Skip vue-tsc type-check in CI/Docker — use plain vite build
RUN npx vite build

# Stage 3: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=rust-builder /build/target/release/notes-server ./
COPY --from=node-builder /build/dist ./dist

VOLUME ["/data"]
EXPOSE 8080

ENV NYX_ROOT=/data
ENV STATIC_DIR=/app/dist
ENV PORT=8080
ENV AUTH_MODE=local
ENV SERVER_NAME="Nyx Notes"

CMD ["./notes-server"]
