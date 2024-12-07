ARG RUST_VERSION=1.81.0
ARG APP_NAME=udpredirect

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
# Install host build dependencies.
RUN apt update
RUN apt-get install -y clang lld musl-dev git llvm libelf-dev libpcap-dev \
    build-essential libc6-dev-i386 libbpf-dev m4
ENV RUST_BACKTRACE=1
RUN rustup component add rustfmt
WORKDIR /app
 
FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG APP_NAME
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin udpredirect


FROM ubuntu:22.04 AS prod

RUN apt update
RUN apt-get install -y clang lld musl-dev git llvm libelf-dev libpcap-dev \
    build-essential libc6-dev-i386 libbpf-dev m4 netcat-traditional vim ethtool iproute2
ENV RUST_BACKTRACE=1

# Copy the executable from the "build" stage.
COPY --from=builder /app/target/release/udpredirect /bin/udpredirect

ENV RUST_LOG=debug
CMD ["/bin/udpredirect"]
