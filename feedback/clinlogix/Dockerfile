# ---------- Build stage ----------
FROM rust:1-bookworm AS builder
WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/clinlogix /usr/local/bin/clinlogix

ENTRYPOINT ["clinlogix"]
