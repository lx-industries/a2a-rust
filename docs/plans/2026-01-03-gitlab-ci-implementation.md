# GitLab CI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add GitLab CI with Rust builds, WASM builds, linting, testing, commitlint, renovate, and semantic-release for workspace publishing.

**Architecture:** Self-contained CI configuration (no shared templates) with custom Docker images containing sccache. Native builds for all crates, WASM builds for wasm32-wasip2 target. Semantic-release handles workspace publishing to crates.io.

**Tech Stack:** GitLab CI, Docker, Rust 1.92.0, sccache, semantic-release, commitlint, Renovate

---

## Task 1: Create Native Build Dockerfile

**Files:**
- Create: `images/rust/x86_64-unknown-linux-gnu/Dockerfile`

**Step 1: Create directory structure**

```bash
mkdir -p images/rust/x86_64-unknown-linux-gnu
```

**Step 2: Write Dockerfile**

```dockerfile
FROM rust:1.92.0-bookworm@sha256:3d0d1a335e1d1220d416a1f38f29925d40ec9929d3c83e07a263adf30a7e4aa3

# renovate: datasource=github-releases depName=mozilla/sccache
ARG SCCACHE_VERSION="0.12.0"

RUN curl -fsSL https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz \
    | tar xz --strip-components=1 -C /usr/local/bin sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl/sccache

RUN rustup component add rustfmt clippy
```

**Step 3: Verify Dockerfile syntax**

```bash
docker build --help > /dev/null && echo "Docker available"
```

**Step 4: Commit**

```bash
git add images/rust/x86_64-unknown-linux-gnu/Dockerfile
git commit -m "build: add native Rust Docker image with sccache"
```

---

## Task 2: Create WASM Build Dockerfile

**Files:**
- Create: `images/rust/wasm32-wasip2/Dockerfile`

**Step 1: Create directory structure**

```bash
mkdir -p images/rust/wasm32-wasip2
```

**Step 2: Write Dockerfile**

```dockerfile
FROM rust:1.92.0-bookworm@sha256:3d0d1a335e1d1220d416a1f38f29925d40ec9929d3c83e07a263adf30a7e4aa3

# renovate: datasource=github-releases depName=mozilla/sccache
ARG SCCACHE_VERSION="0.12.0"

RUN curl -fsSL https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz \
    | tar xz --strip-components=1 -C /usr/local/bin sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl/sccache

RUN rustup component add rustfmt clippy
RUN rustup target add wasm32-wasip2
```

**Step 3: Commit**

```bash
git add images/rust/wasm32-wasip2/Dockerfile
git commit -m "build: add WASM Rust Docker image with wasm32-wasip2 target"
```

---

## Task 3: Create Image Build CI Configuration

**Files:**
- Create: `.gitlab/ci/images.gitlab-ci.yml`

**Step 1: Create directory structure**

```bash
mkdir -p .gitlab/ci
```

**Step 2: Write images CI config**

```yaml
# Docker image build jobs for CI runners

.docker-in-docker:
  image: docker:27.5.1-dind@sha256:aa3df78ecf320f5fafdce71c659f1629e96e9de0968305fe1de670e0ca9176ce
  services:
    - docker:27.5.1-dind@sha256:aa3df78ecf320f5fafdce71c659f1629e96e9de0968305fe1de670e0ca9176ce
  variables:
    DOCKER_TLS_CERTDIR: "/certs"
    GIT_FETCH_EXTRA_FLAGS: "+refs/heads/main:refs/remotes/origin/main"
  before_script:
    - docker info
  interruptible: true

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

**Step 3: Commit**

```bash
git add .gitlab/ci/images.gitlab-ci.yml
git commit -m "ci: add Docker image build jobs"
```

---

## Task 4: Create Rust CI Configuration

**Files:**
- Create: `.gitlab/ci/rust.gitlab-ci.yml`

**Step 1: Write Rust CI config**

```yaml
# Rust-specific CI configuration

# File patterns that trigger Rust jobs
.rust-changes: &rust-changes
  - "Cargo.toml"
  - "Cargo.lock"
  - "crates/**/*"
  - "tests/**/*"
  - ".cargo/**/*"
  - ".gitlab/ci/rust.gitlab-ci.yml"

# Cargo registry cache (pull-only for dependencies)
.cargo-registry-cache: &cargo-registry-cache
  key: $CI_COMMIT_REF_SLUG
  paths:
    - ".cargo/.crates.toml"
    - ".cargo/.crates2.json"
    - ".cargo/bin/"
    - ".cargo/registry/index/"
    - ".cargo/registry/cache/"
    - ".cargo/registry/src/"
    - ".cargo/git/db/"
  policy: pull

# Base template for all Rust jobs
.rust-template:
  # TODO: Update with actual registry image after first build
  image: rust:1.92.0@sha256:910b9dc6597a3ef16458dc1d20520714d3526ddb038749b8d87334798064d672
  variables:
    CARGO_HOME: ".cargo"
    RUSTC_WRAPPER: sccache
    GIT_FETCH_EXTRA_FLAGS: "+refs/heads/main:refs/remotes/origin/main"
  rules:
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
      changes: *rust-changes
    - if: "$CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH"
      changes: *rust-changes
    - if: "$CI_COMMIT_TAG"
  before_script:
    - rustc --version
    - cargo --version
    - sccache --version || true
  cache:
    - <<: *cargo-registry-cache
  interruptible: true

# Build jobs
build:x86_64-unknown-linux-gnu:
  extends: .rust-template
  stage: build
  script:
    - cargo build --workspace --all-features --verbose
    - sccache --show-stats || true

build:wasm32-wasip2:
  extends: .rust-template
  # TODO: Update with actual registry image after first build
  image: rust:1.92.0@sha256:910b9dc6597a3ef16458dc1d20520714d3526ddb038749b8d87334798064d672
  stage: build
  before_script:
    - rustup target add wasm32-wasip2
    - !reference [.rust-template, before_script]
  script:
    - cargo build -p a2a-transport-wasi -p a2a-wasm-component --target wasm32-wasip2 --verbose
    - sccache --show-stats || true

# Test jobs
cargo-test:
  extends: .rust-template
  stage: test
  script:
    - cargo test --workspace --all-features --verbose
    - sccache --show-stats || true
  needs:
    - build:x86_64-unknown-linux-gnu

cargo-doc-test:
  extends: .rust-template
  stage: test
  script:
    - cargo test --workspace --doc --all-features
    - sccache --show-stats || true
  needs:
    - build:x86_64-unknown-linux-gnu

# Lint jobs
cargo-clippy:
  extends: .rust-template
  stage: check
  before_script:
    - !reference [.rust-template, before_script]
    - cargo clippy --version
  script:
    - cargo clippy --workspace --all-targets --all-features -- -D warnings
    - sccache --show-stats || true

cargo-fmt:
  extends: .rust-template
  stage: check
  before_script:
    - !reference [.rust-template, before_script]
    - cargo fmt --version
  script:
    - cargo fmt --all -- --check

# Docs job
cargo-doc:
  extends: .rust-template
  stage: docs
  script:
    - cargo doc --workspace --all-features --no-deps
    - sccache --show-stats || true
  needs:
    - build:x86_64-unknown-linux-gnu
  artifacts:
    paths:
      - target/doc
    expire_in: 1 week
```

**Step 2: Commit**

```bash
git add .gitlab/ci/rust.gitlab-ci.yml
git commit -m "ci: add Rust build, test, and lint jobs"
```

---

## Task 5: Create Main GitLab CI Configuration

**Files:**
- Create: `.gitlab-ci.yml`

**Step 1: Write main CI config**

```yaml
# GitLab CI/CD configuration for a2a-rust

include:
  - local: '.gitlab/ci/images.gitlab-ci.yml'
  - local: '.gitlab/ci/rust.gitlab-ci.yml'

# Global variables for package versions
variables:
  # renovate: datasource=npm depName=@commitlint/cli
  COMMITLINT_CLI_VERSION: "19.8.1"
  # renovate: datasource=npm depName=@commitlint/config-conventional
  COMMITLINT_CONFIG_VERSION: "19.8.1"
  # renovate: datasource=npm depName=semantic-release
  SEMANTIC_RELEASE_VERSION: "24.2.6"
  # renovate: datasource=npm depName=@semantic-release/changelog
  SEMANTIC_RELEASE_CHANGELOG_VERSION: "6.0.3"
  # renovate: datasource=npm depName=@semantic-release/git
  SEMANTIC_RELEASE_GIT_VERSION: "10.0.1"
  # renovate: datasource=npm depName=@semantic-release/gitlab
  SEMANTIC_RELEASE_GITLAB_VERSION: "13.2.3"
  # renovate: datasource=npm depName=@semantic-release/exec
  SEMANTIC_RELEASE_EXEC_VERSION: "7.0.3"
  # renovate: datasource=github-releases depName=semantic-release-cargo/semantic-release-cargo
  SEMANTIC_RELEASE_CARGO_VERSION: "2.4.44"
  # renovate: datasource=npm depName=conventional-changelog-conventionalcommits
  CONVENTIONAL_CHANGELOG_CONVENTIONALCOMMITS_VERSION: "8.1.0"

# Workflow rules for automatic pipeline management
workflow:
  auto_cancel:
    on_new_commit: interruptible
    on_job_failure: none
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
    - if: $CI_COMMIT_TAG
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH && $CI_COMMIT_TITLE =~ /^chore\(release\):/
      when: never
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH

stages:
  - images
  - check
  - build
  - test
  - docs
  - release

# Commitlint job - validates commit messages
commitlint:
  image: node:22.16.0@sha256:252e85de9f1b74e3940f291f5dd9c3718b49f32e0b49dbc4f93fbf8e3df44772
  stage: check
  rules:
    - if: '$CI_PIPELINE_SOURCE == "push"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
    - if: "$CI_COMMIT_TAG"
      when: never
  variables:
    GIT_FETCH_EXTRA_FLAGS: "+refs/heads/main:refs/remotes/origin/main"
  before_script:
    - npm install -g @commitlint/cli@${COMMITLINT_CLI_VERSION} @commitlint/config-conventional@${COMMITLINT_CONFIG_VERSION}
  script:
    - commitlint --from=$(git merge-base origin/main HEAD) --to=$CI_COMMIT_SHA
  interruptible: true

# Semantic release template
.semantic-release-template: &semantic-release-template
  image: rust:1.92.0@sha256:910b9dc6597a3ef16458dc1d20520714d3526ddb038749b8d87334798064d672
  before_script:
    - apt-get update -qq && apt-get install -y -qq curl
    - curl -fsSL https://nodejs.org/dist/v22.16.0/node-v22.16.0-linux-x64.tar.xz | tar -xJ -C /usr/local --strip-components=1
    - node --version
    - npm --version
    - npm install -g semantic-release@${SEMANTIC_RELEASE_VERSION} @semantic-release/changelog@${SEMANTIC_RELEASE_CHANGELOG_VERSION} @semantic-release/git@${SEMANTIC_RELEASE_GIT_VERSION} @semantic-release/gitlab@${SEMANTIC_RELEASE_GITLAB_VERSION} @semantic-release/exec@${SEMANTIC_RELEASE_EXEC_VERSION} conventional-changelog-conventionalcommits@${CONVENTIONAL_CHANGELOG_CONVENTIONALCOMMITS_VERSION}
    - curl -L https://github.com/semantic-release-cargo/semantic-release-cargo/releases/download/v${SEMANTIC_RELEASE_CARGO_VERSION}/semantic-release-cargo-x86_64-unknown-linux-musl -o /usr/local/bin/semantic-release-cargo
    - chmod +x /usr/local/bin/semantic-release-cargo
    - semantic-release-cargo --version
  interruptible: true

# Semantic release dry-run for validation
check:semantic-release:
  <<: *semantic-release-template
  stage: check
  script:
    - semantic-release --dry-run --no-ci

# Semantic release job - creates releases
semantic-release:
  <<: *semantic-release-template
  stage: release
  rules:
    - if: '$CI_PIPELINE_SOURCE == "schedule" && $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH && $RELEASE_ENABLED == "true"'
    - if: '$CI_PIPELINE_SOURCE == "web" && $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH && $RELEASE_ENABLED == "true"'
  variables:
    GIT_FETCH_EXTRA_FLAGS: "+refs/heads/main:refs/remotes/origin/main"
  before_script:
    - !reference [.semantic-release-template, before_script]
    - git config user.name "${GITLAB_USER_NAME}"
    - git config user.email "${GITLAB_USER_EMAIL}"
  script:
    - semantic-release
  interruptible: false
```

**Step 2: Commit**

```bash
git add .gitlab-ci.yml
git commit -m "ci: add main GitLab CI configuration with semantic-release"
```

---

## Task 6: Create Commitlint Configuration

**Files:**
- Create: `commitlint.config.js`

**Step 1: Write commitlint config**

```javascript
module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'header-max-length': [2, 'always', 150],
  },
};
```

**Step 2: Commit**

```bash
git add commitlint.config.js
git commit -m "chore: add commitlint configuration"
```

---

## Task 7: Create Renovate Configuration

**Files:**
- Create: `renovate.json`

**Step 1: Write renovate config**

```json
{
  "$schema": "https://docs.renovatebot.com/renovate-schema.json",
  "extends": ["local>lx-industries/renovate-config"]
}
```

**Step 2: Commit**

```bash
git add renovate.json
git commit -m "chore: add Renovate configuration"
```

---

## Task 8: Create Semantic Release Configuration

**Files:**
- Create: `.releaserc.json`

**Step 1: Write semantic-release config**

```json
{
  "branches": ["main"],
  "plugins": [
    [
      "@semantic-release/commit-analyzer",
      {
        "preset": "angular",
        "releaseRules": [
          { "breaking": true, "release": "minor" },
          { "type": "docs", "scope": "README", "release": "patch" },
          { "type": "fix", "release": "patch" },
          { "type": "refactor", "release": "patch" },
          { "type": "chore", "scope": "deps", "release": "patch" },
          { "type": "feat", "release": "minor" }
        ],
        "parserOpts": {
          "noteKeywords": ["BREAKING CHANGE", "BREAKING CHANGES"]
        }
      }
    ],
    [
      "@semantic-release/release-notes-generator",
      {
        "preset": "conventionalcommits",
        "presetConfig": {
          "types": [
            { "type": "feat", "section": "Features", "hidden": false },
            { "type": "fix", "section": "Bug Fixes", "hidden": false },
            { "type": "chore", "scope": "deps", "section": "Miscellaneous Chores", "hidden": false }
          ]
        }
      }
    ],
    "@semantic-release/changelog",
    [
      "@semantic-release/exec",
      {
        "prepareCmd": "semantic-release-cargo prepare ${nextRelease.version}",
        "publishCmd": "semantic-release-cargo publish"
      }
    ],
    [
      "@semantic-release/git",
      {
        "assets": [
          "CHANGELOG.md",
          "Cargo.toml",
          "Cargo.lock",
          "crates/*/Cargo.toml"
        ],
        "message": "chore(release): ${nextRelease.version}\n\n${nextRelease.notes}"
      }
    ],
    [
      "@semantic-release/gitlab",
      {
        "assets": []
      }
    ]
  ]
}
```

**Step 2: Commit**

```bash
git add .releaserc.json
git commit -m "chore: add semantic-release configuration for workspace publishing"
```

---

## Task 9: Update .gitignore

**Files:**
- Modify: `.gitignore`

**Step 1: Add node_modules to gitignore**

Append to `.gitignore`:

```
node_modules/
```

**Step 2: Commit**

```bash
git add .gitignore
git commit -m "chore: add node_modules to gitignore"
```

---

## Task 10: Verify CI Configuration Locally

**Step 1: Validate YAML syntax**

```bash
python3 -c "import yaml; yaml.safe_load(open('.gitlab-ci.yml'))" && echo "Main CI YAML valid"
python3 -c "import yaml; yaml.safe_load(open('.gitlab/ci/rust.gitlab-ci.yml'))" && echo "Rust CI YAML valid"
python3 -c "import yaml; yaml.safe_load(open('.gitlab/ci/images.gitlab-ci.yml'))" && echo "Images CI YAML valid"
```

Expected: All three print "valid" messages.

**Step 2: Validate JSON syntax**

```bash
python3 -c "import json; json.load(open('.releaserc.json'))" && echo "Release config JSON valid"
python3 -c "import json; json.load(open('renovate.json'))" && echo "Renovate JSON valid"
```

Expected: Both print "valid" messages.

**Step 3: Verify Dockerfile can be parsed**

```bash
grep "^FROM rust:" images/rust/x86_64-unknown-linux-gnu/Dockerfile
grep "^FROM rust:" images/rust/wasm32-wasip2/Dockerfile
```

Expected: Both show `FROM rust:1.92.0-bookworm@sha256:...`

---

## Task 11: Final Verification and Summary Commit

**Step 1: Run cargo check to ensure project still builds**

```bash
cargo check --workspace
```

Expected: No errors.

**Step 2: Review all changes**

```bash
git log --oneline -10
```

Expected: See commits for each task.

**Step 3: Verify file structure**

```bash
ls -la .gitlab-ci.yml .gitlab/ci/ images/rust/ commitlint.config.js renovate.json .releaserc.json
```

Expected: All files present.

---

## Post-Implementation Notes

### Bootstrap Process

After pushing to GitLab:

1. Trigger first pipeline with `FORCE_BUILD_IMAGES=true` variable to build Docker images
2. After images are pushed, note the SHA256 digests from the registry
3. Update `.gitlab/ci/rust.gitlab-ci.yml` to use `${CI_REGISTRY_IMAGE}/rust/<target>:<version>@sha256:<digest>` instead of public `rust:` image
4. Commit the digest updates

### sccache Configuration

Ensure these variables are set in GitLab CI/CD Settings > Variables:

- `SCCACHE_GCS_BUCKET` or `SCCACHE_S3_BUCKET` (depending on backend)
- `SCCACHE_GCS_KEY_PREFIX` or equivalent
- Any authentication credentials required by your sccache backend

### crates.io Publishing

For semantic-release to publish to crates.io, set in GitLab CI/CD Settings > Variables:

- `CARGO_REGISTRY_TOKEN` - Your crates.io API token
