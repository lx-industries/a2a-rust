FROM rust:1.92.0-bookworm@sha256:e90e846de4124376164ddfbaab4b0774c7bdeef5e738866295e5a90a34a307a2

# renovate: datasource=github-releases depName=mozilla/sccache
ARG SCCACHE_VERSION="0.12.0"

RUN curl -fsSL https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz \
    | tar xz --strip-components=1 -C /usr/local/bin sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl/sccache

RUN rustup component add rustfmt clippy
