# FILE: Dockerfile
# ----------------------------------------------------
# MultiStaged Rust Backend Builder for Podman
# ----------------------------------------------------

# ====================================================
# 1.  Builder Stage...
# ====================================================
FROM rust1.89 AS builder

# Workdir + installation
WORKDIR /app
RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev sqlite3 \
    libsqlite3-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Cargo dependency installation
COPY Cargo.toml Cargo.lock ./
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release

# seting proper files + building
RUN rm -rf ./src
COPY . .
RUN cargo build --release

# ====================================================
# 2.  Runtime Stage...
# ====================================================

FROM debain:bookworm-slim
WORKDIR /app

RUN apt get-update && apt get-install -y \
    ca-certificates sqlite3 libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

# pacakge name in cargo.toml = backend
COPY --from=builder /app/target/release/backend /app/backend

