# FILE: Dockerfile
# ----------------------------------------------------
# MultiStaged Rust Backend Builder for Podman
# ----------------------------------------------------

# ====================================================
# 1.  Builder Stage...
# ====================================================
FROM rust:1.89-bookworm AS builder

# Workdir 
WORKDIR /app
ENV CARGO_TARGET_DIR=/app/target

# Rust Building Dependency of C
RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev sqlite3 \
    libsqlite3-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ENV DATABASE_URL=sqlite:///app/database/dev.db

# Cargo build
COPY . .
RUN cargo build --release

# ====================================================
# 2.  Runtime Stage...
# ====================================================

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates sqlite3 libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

# pacakge name in cargo.toml = backend
COPY --from=builder /app/target/release/backend /app/backend

EXPOSE 3000
CMD ["./backend"]

