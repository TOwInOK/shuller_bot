# -- Этап сборки бинарника --
FROM rust:1.80.1-alpine AS base

RUN apk add --no-cache musl-dev openssl-dev

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN cargo install sccache
RUN cargo install cargo-chef

ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

FROM base AS planner
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

FROM base AS builder
WORKDIR /app
# set up sccache
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json
# copy project
COPY . .
# build
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release

# -- Этап итоговой сборки --

# Используем минимальный Alpine образ для конечного контейнера
FROM alpine:latest

# Устанавливаем рабочую директорию внутри контейнера
WORKDIR /app

# Устанавливаем необходимые зависимости для запуска
RUN apk add --no-cache ca-certificates libgcc

# Копируем собранный бинарный файл из стадии сборки
COPY --from=builder /app/target/release/shuller_bot .
# Указываем команду запуска
CMD ["./shuller_bot"]