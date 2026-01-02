# GitLab CI Design

Add GitLab CI support with Rust builds, checks, tests, formatting, renovate, semantic-release, and commitlint.

## File Structure

```
.gitlab-ci.yml                              # Main CI config, workflow rules, includes
.gitlab/ci/rust.gitlab-ci.yml               # Rust jobs (fmt, clippy, build, test, docs)
.gitlab/ci/images.gitlab-ci.yml             # Docker image build jobs
images/rust/x86_64-unknown-linux-gnu/Dockerfile   # Native build image
images/rust/wasm32-wasip2/Dockerfile              # WASM build image
commitlint.config.js                        # Conventional commits config
renovate.json                               # Extends lx-industries/renovate-config
.releaserc.json                             # Semantic-release for workspace publishing
```

## Stages

1. `images` - Build custom Docker images (only when Dockerfiles change)
2. `check` - commitlint, semantic-release dry-run, fmt, clippy
3. `build` - Native build, WASM build
4. `test` - cargo test, doc-test
5. `docs` - rustdoc generation
6. `release` - semantic-release (manual/scheduled trigger)

## Workflow Rules

From rmcp-server-builder pattern:

```yaml
workflow:
  auto_cancel:
    on_new_commit: interruptible
    on_job_failure: none
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH && $CI_COMMIT_TITLE =~ /^chore\(release\):/
      when: never
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
    - if: $CI_COMMIT_TAG
```

## Docker Images

Two custom images stored in project container registry.

### images/rust/x86_64-unknown-linux-gnu/Dockerfile

```dockerfile
FROM rust:1.92.0-bookworm@sha256:...

# renovate: datasource=github-releases depName=mozilla/sccache
ARG SCCACHE_VERSION="0.12.0"

RUN curl -fsSL https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz \
    | tar xz -C /usr/local/bin --strip-components=1

RUN rustup component add rustfmt clippy
```

### images/rust/wasm32-wasip2/Dockerfile

```dockerfile
FROM rust:1.92.0-bookworm@sha256:...

# renovate: datasource=github-releases depName=mozilla/sccache
ARG SCCACHE_VERSION="0.12.0"

RUN curl -fsSL https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz \
    | tar xz -C /usr/local/bin --strip-components=1

RUN rustup component add rustfmt clippy
RUN rustup target add wasm32-wasip2
```

### Image Build Jobs

```yaml
.docker-in-docker:
  image: docker:27.5.1-dind@sha256:aa3df78ecf320f5fafdce71c659f1629e96e9de0968305fe1de670e0ca9176ce
  services:
    - docker:27.5.1-dind@sha256:aa3df78ecf320f5fafdce71c659f1629e96e9de0968305fe1de670e0ca9176ce
  variables:
    DOCKER_TLS_CERTDIR: "/certs"
    GIT_FETCH_EXTRA_FLAGS: "+refs/heads/main:refs/remotes/origin/main"
  before_script:
    - docker info

build-image:x86_64-unknown-linux-gnu:
  extends: .docker-in-docker
  stage: images
  rules:
    - changes:
        - images/rust/x86_64-unknown-linux-gnu/**/*
    - if: '$FORCE_BUILD_IMAGES == "true"'
  script:
    - RUST_VERSION=$(grep "^FROM rust:" images/rust/x86_64-unknown-linux-gnu/Dockerfile | sed -r "s/[^:]*:([0-9]+\.[0-9]+\.[0-9]+).*/\1/")
    - docker login -u gitlab-ci-token -p ${CI_JOB_TOKEN} ${CI_REGISTRY}
    - docker build -t ${CI_REGISTRY_IMAGE}/rust/x86_64-unknown-linux-gnu:${RUST_VERSION} images/rust/x86_64-unknown-linux-gnu
    - docker push ${CI_REGISTRY_IMAGE}/rust/x86_64-unknown-linux-gnu:${RUST_VERSION}

build-image:wasm32-wasip2:
  extends: .docker-in-docker
  stage: images
  rules:
    - changes:
        - images/rust/wasm32-wasip2/**/*
    - if: '$FORCE_BUILD_IMAGES == "true"'
  script:
    - RUST_VERSION=$(grep "^FROM rust:" images/rust/wasm32-wasip2/Dockerfile | sed -r "s/[^:]*:([0-9]+\.[0-9]+\.[0-9]+).*/\1/")
    - docker login -u gitlab-ci-token -p ${CI_JOB_TOKEN} ${CI_REGISTRY}
    - docker build -t ${CI_REGISTRY_IMAGE}/rust/wasm32-wasip2:${RUST_VERSION} images/rust/wasm32-wasip2
    - docker push ${CI_REGISTRY_IMAGE}/rust/wasm32-wasip2:${RUST_VERSION}
```

## Rust Jobs

### Change Detection

```yaml
.rust-changes: &rust-changes
  - "Cargo.toml"
  - "Cargo.lock"
  - "crates/**/*"
  - "tests/**/*"
  - ".cargo/**/*"
  - ".gitlab/ci/rust.gitlab-ci.yml"

.rust-template:
  variables:
    RUSTC_WRAPPER: sccache
    GIT_FETCH_EXTRA_FLAGS: "+refs/heads/main:refs/remotes/origin/main"
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
      changes: *rust-changes
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
      changes: *rust-changes
    - if: $CI_COMMIT_TAG
```

### Caching

- Cargo registry cache: pull-only, keyed by `$CI_COMMIT_REF_SLUG`
- sccache: `RUSTC_WRAPPER=sccache`, backend configured via project CI settings
- No target/ directory caching

### Jobs

**Check Stage:**
- `cargo-fmt` - `cargo fmt --all -- --check`
- `cargo-clippy` - `cargo clippy --all-targets --all-features -- -D warnings`
- `commitlint` - Validates commit messages (MR pipelines only)
- `check:semantic-release` - Dry-run of semantic-release config

**Build Stage:**
- `build:x86_64-unknown-linux-gnu` - Full workspace build with `--all-features`
- `build:wasm32-wasip2` - `cargo build -p a2a-transport-wasi -p a2a-wasm-component --target wasm32-wasip2`

**Test Stage:**
- `cargo-test` - `cargo test --all-features` (depends on native build)
- `cargo-doc-test` - `cargo test --doc --all-features`

**Docs Stage:**
- `cargo-doc` - `cargo doc --all-features --no-deps`, artifacts expire in 1 week

**Release Stage:**
- `semantic-release` - Manual/scheduled trigger with `RELEASE_ENABLED=true`

## Commitlint

```javascript
// commitlint.config.js
module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'header-max-length': [2, 'always', 150],
  },
};
```

## Renovate

```json
{
  "$schema": "https://docs.renovatebot.com/renovate-schema.json",
  "extends": ["local>lx-industries/renovate-config"]
}
```

## Semantic Release

Based on rmcp-openapi workspace pattern:

- Targets `main` branch
- Commit analyzer with conventional commits
- Changelog generation
- `semantic-release-cargo prepare` and `publish` commands
- Git commits: `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, `crates/*/Cargo.toml`
- GitLab release creation

### Publishing Order

Crates published in dependency order:

1. `a2a-types` (no internal deps)
2. `a2a-transport` (no internal deps)
3. `a2a-transport-wasi` (depends on a2a-transport)
4. `a2a-client` (depends on a2a-types, a2a-transport)
5. `a2a-server` (depends on a2a-types, a2a-transport)
6. `a2a-wasm-component` (depends on all above)

## Bootstrap Process

1. First pipeline run: trigger with `FORCE_BUILD_IMAGES=true` to build Docker images
2. After images are pushed, update CI config with SHA256 digests
3. Subsequent pipelines use pinned `image@sha256:...` references
