ARG RUST_VERSION=1.81.0
ARG APP_NAME=udpredirect

FROM rust:${RUST_VERSION} AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apt update
RUN apt-get install -y clang lld musl-dev git llvm libelf-dev libpcap-dev \
    build-essential libc6-dev-i386 libbpf-dev m4 netcat-traditional vim ethtool iproute2
ENV RUST_BACKTRACE=1
RUN rustup component add rustfmt

COPY ./src src
COPY Cargo.toml Cargo.toml
COPY build.rs build.rs
COPY Cargo.lock Cargo.lock

RUN cargo build --locked --release

ENV RUST_LOG=debug
