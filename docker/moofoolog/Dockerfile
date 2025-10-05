# Builder
FROM rust:1.88 as builder
RUN apt-get update && apt-get install -y \
    musl-tools \
    pkg-config \
    build-essential \
    libssl-dev \
WORKDIR /usr/src/moofoolog
COPY . .
# RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add x86_64-unknown-linux-gnu
# ENV RUSTFLAGS="-C target-feature=-crt-static"
# RUN cargo build --release --target x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Final image
FROM scratch
COPY --from=builder /usr/src/moofoolog/target/x86_64-unknown-linux-musl/release/moofoolog /usr/local/bin/moofoolog
ENTRYPOINT ["/usr/local/bin/moofoolog"]
