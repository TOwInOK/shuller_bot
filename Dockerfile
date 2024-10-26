FROM rust:alpine AS base

# Устанавливаем необходимые зависимости и cargo-chef
RUN apk add --no-cache musl-dev openssl-dev \
    && cargo install cargo-chef

# Настройка переменных окружения для оптимизации
ENV RUSTFLAGS="-C target-feature=-crt-static -C opt-level=3 -C target-cpu=native -C link-arg=-s"
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV CARGO_HTTP_MULTIPLEXING=false
ENV CARGO_INCREMENTAL=0
ENV CARGO_PROFILE_RELEASE_LTO=true
ENV CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1

FROM base AS planner
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo chef prepare --recipe-path recipe.json

FROM base AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json

ARG CARGO_BUILD_JOBS=4

# Сборка зависимостей
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo chef cook --release --recipe-path recipe.json

COPY . .
# Финальная сборка
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release --jobs ${CARGO_BUILD_JOBS}

FROM alpine:latest
WORKDIR /app

RUN apk add --no-cache ca-certificates libgcc

COPY --from=builder /app/target/release/shuller_bot .

# Создаем non-root пользователя
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

CMD ["./shuller_bot"]
