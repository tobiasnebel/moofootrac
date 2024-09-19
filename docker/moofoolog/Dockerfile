####################################################################################################
## Builder RS
####################################################################################################
FROM rust:latest as builder-rs
# RUN apt-get update && apt-get install -y \
#     musl-tools \
#     pkg-config \
#     build-essential \
#     libssl-dev \
#     ca-certificates
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    openjdk-21-jdk \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/moofoolog
COPY . .
# RUN rustup target add x86_64-unknown-linux-musl # musl fails due to ssl-lib issue
RUN rustup target add x86_64-unknown-linux-gnu
# ENV RUSTFLAGS="-C target-feature=-crt-static" # dunno
# RUN cargo build --release --target x86_64-unknown-linux-musl # musl fails due to ssl-lib issue
RUN cargo build --release --target x86_64-unknown-linux-gnu


####################################################################################################
## Builder - NPM
####################################################################################################
FROM node:latest AS builder-npm

RUN update-ca-certificates

WORKDIR /moofoolog
COPY ../../frontend .

# Append token for 'npm.pkg.github.com' to .npmrc file.
# RUN echo "//npm.pkg.github.com/:_authToken=${NPM_AUTH_TOKEN_ARG}" >> ./.npmrc

RUN npm install
# NOTE: for now we assume that TS models have been built from openapi spec 
#       during previous cargo build (or that the required models exists in SCR)
RUN npm run build


####################################################################################################
## Final image
####################################################################################################
FROM debian:stable-slim AS result

WORKDIR /moofoolog

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates wget \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# this allow to automagically link built package to the repository
LABEL org.opencontainers.image.source https://github.com/nebeltobias/moofoolog

EXPOSE 9011/tcp
RUN mkdir ./dist
COPY --from=builder-npm /moofoolog/dist ./dist

# rust backend
COPY --from=builder-rs /usr/src/moofoolog/config/default.yml /moofoolog/config/
COPY --from=builder-rs /usr/src/moofoolog/target/x86_64-unknown-linux-gnu/release/moofoolog /moofoolog/

ENTRYPOINT ["/moofoolog/moofoolog"]
