# Build rust binaries
FROM messense/rust-musl-cross:x86_64-musl as builder
ENV LIBOPUS_STATIC=1
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN rustup target add x86_64-unknown-linux-gnu
WORKDIR /zeabot
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl
# Put binaries in a minimal image
# FROM scratch
# ADD https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp .
FROM alpine:latest as final
RUN apk add --no-cache yt-dlp
COPY --from=builder /zeabot/target/x86_64-unknown-linux-musl/release/zeabot /zeabot
ENTRYPOINT ["/zeabot"]

# FROM rust:alpine as builder
# ENV RUSTFLAGS="-C target-feature=+crt-static"
# WORKDIR /bot
# RUN apk update && apk add --no-cache musl-dev pcc-libs-dev pkgconfig openssl-dev cmake
# RUN --mount=type=cache,target=/usr/local/cargo/registry cargo install cargo-strip
# COPY . .
# RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=./target \
#     cargo build --release && \
#     cargo strip && \
#     mv target/release/zeabot /bot/zeabot
# FROM scratch
# COPY --from=builder "/bot/zeabot" /
# ENTRYPOINT [ "./zeabot" ]

# FROM lukemathwalker/cargo-chef:latest as chef
# WORKDIR /app
# FROM chef AS planner
# COPY ./Cargo.toml ./Cargo.lock ./
# COPY ./src ./src
# RUN cargo chef prepare
# FROM chef AS builder
# COPY --from=planner /app/recipe.json .
# RUN cargo chef cook --release
# COPY . .
# RUN cargo build --release
# RUN mv ./target/release/zeabot ./app
# FROM debian:stable-slim AS runtime
# WORKDIR /app
# COPY --from=builder /app/app /usr/local/bin/
# ENTRYPOINT ["/usr/local/bin/app"]