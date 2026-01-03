# renovate: datasource=docker depName=rust
FROM rust:1.92.0-bookworm@sha256:3d0d1a335e1d1220d416a1f38f29925d40ec9929d3c83e07a263adf30a7e4aa3

# renovate: datasource=github-releases depName=mozilla/sccache
ARG SCCACHE_VERSION="0.12.0"

RUN curl -fsSL https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz \
    | tar xz --strip-components=1 -C /usr/local/bin sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl/sccache

RUN rustup component add rustfmt clippy
RUN rustup target add wasm32-wasip2
