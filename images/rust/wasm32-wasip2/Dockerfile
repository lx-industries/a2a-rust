FROM rust:1.92.0@sha256:6ca5ad23231207874325a751b9df584d51cd42c066c74c6963c264e3233c3e8e

# renovate: datasource=github-releases depName=mozilla/sccache
ARG SCCACHE_VERSION="0.13.0"

RUN curl -fsSL https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz \
    | tar xz --strip-components=1 -C /usr/local/bin sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl/sccache

RUN rustup component add rustfmt clippy
RUN rustup target add wasm32-wasip2
