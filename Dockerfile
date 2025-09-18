# syntax=docker/dockerfile:1.4
FROM node:20-bullseye

ENV DEBIAN_FRONTEND=noninteractive \
    RUSTUP_HOME=/opt/rust/rustup \
    CARGO_HOME=/opt/rust/cargo \
    PATH=/opt/rust/cargo/bin:/usr/local/bin:$PATH

RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential \
        pkg-config \
        libssl-dev \
        libudev-dev \
        libclang-dev \
        clang \
        cmake \
        python3 \
        git \
        curl \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Install Rust toolchain
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path && \
    rustup default stable && \
    rustup component add rustfmt clippy

RUN curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash

WORKDIR /workspace

# Copy dependency manifests separately for better Docker layer caching
COPY package.json package-lock.json* tsconfig.json Anchor.toml Cargo.toml Cargo.lock ./
COPY programs ./programs
COPY tests ./tests
COPY config ./config

# Prime node and rust dependencies (optional but speeds up container usage)
RUN npm install && \
    anchor build

CMD ["bash"]
