# Build rust binaries
FROM messense/rust-musl-cross:x86_64-musl as chef
ENV LIBOPUS_STATIC=1
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN rustup target add x86_64-unknown-linux-gnu \
    cargo install cargo-chef
WORKDIR /zeabot

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /zeabot/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Put binaries in a minimal image
FROM alpine:latest as final
RUN apk add --no-cache yt-dlp
COPY --from=builder /zeabot/target/x86_64-unknown-linux-musl/release/zeabot /zeabot
ENTRYPOINT ["/zeabot"]
