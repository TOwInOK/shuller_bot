FROM alpine:latest AS base

ENV PATH="/root/.cargo/bin:${PATH}"
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV CARGO_HTTP_MULTIPLEXING=false
ENV RUST_TARGET=x86_64-unknown-linux-musl
ENV RUSTFLAGS="-C target-feature=+crt-static"
ENV CC="gcc"
ENV AR="ar"
ENV CFLAGS="-I/usr/include"
ENV LDFLAGS="-L/usr/lib"
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV OPENSSL_STATIC=1
ENV OPENSSL_DIR=/usr

RUN apk add --no-cache \
build-base \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    gcc \
    curl \
    perl \
    make \
    && curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs | sh -s -- -y \
        --profile minimal \
        --default-toolchain stable \
        --target ${RUST_TARGET} \
    && ~/.cargo/bin/cargo install cargo-chef --locked

# Planning stage
FROM base AS planner
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo generate-lockfile && \
    cargo chef prepare --recipe-path recipe.json

# Builder stage
FROM base AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --target ${RUST_TARGET} --recipe-path recipe.json
COPY . .
RUN cargo build --release --target ${RUST_TARGET}

# Final stage
FROM alpine:latest
WORKDIR /app
RUN apk add --no-cache ca-certificates openssl
# Changed path to include the target architecture
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/shuller_bot .
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser
