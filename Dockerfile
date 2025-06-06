FROM rust:1.87.0-bookworm AS base

WORKDIR /usr/src/app

RUN rustup install nightly && \
    rustup default nightly && \
    rustup target add x86_64-unknown-linux-musl && \
    rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

FROM base AS builder

COPY . .

RUN cargo build --release --package server --bin server --target x86_64-unknown-linux-musl

ENTRYPOINT ["/bin/bash", "-l", "-c"]

FROM scratch AS release

COPY --from=builder --chmod=744 /usr/src/app/target/x86_64-unknown-linux-musl/release/server /server

VOLUME /config
VOLUME /config.json
VOLUME /native_functions_list.txt

EXPOSE 6900 5121 6121 6901 6123 6124

ENTRYPOINT ["/server"]