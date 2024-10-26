FROM rust:alpine AS base

# Установка только cargo-chef
RUN apk add --no-cache musl-dev openssl-dev \
    && cargo install cargo-chef

# Оптимизации для cargo
ENV RUSTFLAGS="-C target-feature=-crt-static -C opt-level=3 -C target-cpu=native -C link-arg=-s"
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV CARGO_HTTP_MULTIPLEXING=false

# Первый этап - Подготовка рецепта
FROM base AS planner
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
# Создаем пустую main.rs чтобы cargo chef мог проанализировать зависимости
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs
RUN cargo chef prepare --recipe-path recipe.json

# Второй этап - Сборка зависимостей
FROM base AS cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
# Собираем только зависимости
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo chef cook --release --recipe-path recipe.json

# Третий этап - Финальная сборка
FROM base AS builder
WORKDIR /app
# Копируем собранные зависимости
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo/registry /usr/local/cargo/registry
COPY --from=cacher /usr/local/cargo/git /usr/local/cargo/git
# Копируем исходный код
COPY . .
# Собираем только наш код
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release --offline

# Финальный образ
FROM alpine:latest
WORKDIR /app

RUN apk add --no-cache ca-certificates libgcc

COPY --from=builder /app/target/release/shuller_bot .

# Создаем non-root пользователя
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

CMD ["./shuller_bot"]
