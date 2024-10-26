FROM rust:alpine AS base

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
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release


FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache ca-certificates libgcc

COPY --from=builder /app/target/release/shuller_bot .
CMD ["./shuller_bot"]
