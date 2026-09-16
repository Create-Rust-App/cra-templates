# syntax=docker/dockerfile:1

# Matches the pinned toolchain in rust-toolchain.toml.
FROM rust:1.82-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN useradd --create-home --shell /usr/sbin/nologin app \
  && apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*
USER app
WORKDIR /app
COPY --from=builder /app/target/release/axum-starter ./axum-starter
ENV HOST=0.0.0.0 PORT=8080 API_PREFIX=/api/v1
EXPOSE 8080
ENTRYPOINT ["./axum-starter"]
