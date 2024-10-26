ARG TARGETARCH
FROM alpine:latest AS base

# Установка только необходимых зависимостей
RUN apk add --no-cache musl-dev openssl-dev pkgconfig curl gcc \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable \
    && ~/.cargo/bin/cargo install cargo-chef --locked --version 0.1.68

ENV PATH="/root/.cargo/bin:${PATH}"
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV CARGO_HTTP_MULTIPLEXING=false

# Первый этап - Подготовка рецепта
FROM base AS planner
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo generate-lockfile && \
    cargo chef prepare --recipe-path recipe.json

# Сборка для AMD64
FROM base AS builder-amd64
WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl
ENV RUST_TARGET=x86_64-unknown-linux-musl

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --target ${RUST_TARGET} --recipe-path recipe.json
COPY . .
RUN cargo build --release --target ${RUST_TARGET}

# Сборка для ARM64
FROM base AS builder-arm64
WORKDIR /app
RUN rustup target add aarch64-unknown-linux-musl
ENV RUST_TARGET=aarch64-unknown-linux-musl

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --target ${RUST_TARGET} --recipe-path recipe.json
COPY . .
RUN cargo build --release --target ${RUST_TARGET}

# Финальный образ для AMD64
FROM alpine:latest AS final-amd64
WORKDIR /app
RUN apk add --no-cache ca-certificates openssl
COPY --from=builder-amd64 /app/target/x86_64-unknown-linux-musl/release/shuller_bot .
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser
CMD ["./shuller_bot"]

# Финальный образ для ARM64
FROM alpine:latest AS final-arm64
WORKDIR /app
RUN apk add --no-cache ca-certificates openssl
COPY --from=builder-arm64 /app/target/aarch64-unknown-linux-musl/release/shuller_bot .
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser
CMD ["./shuller_bot"]

FROM final-${TARGETARCH}
