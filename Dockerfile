FROM rust:1.94-slim-bookworm as builder

WORKDIR /app

# Системные зависимости
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Копируем Cargo файлы
COPY Cargo.toml Cargo.lock ./

# Копируем .sqlx/ — очень важно!
COPY .sqlx ./.sqlx

# Копируем исходники
COPY src ./src

# Важно: включаем offline-режим именно здесь
ENV SQLX_OFFLINE=true

RUN cargo build --release

# ---------- runtime stage ----------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/mybooks .

EXPOSE 8080

CMD ["./mybooks"]