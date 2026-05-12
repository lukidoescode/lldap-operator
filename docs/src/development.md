# Development Guide

This chapter is for contributors who want to build and test the operator
locally.

## Prerequisites

- Rust 1.88+
- Docker
- [kind](https://kind.sigs.k8s.io/) or [k3d](https://k3d.io/)
- [Helm](https://helm.sh/) v3
- kubectl

## Local Setup

```bash
./scripts/local-dev-setup.sh
```

This creates a kind cluster, deploys an lldap test instance, and installs the
CRDs.

## Building

```bash
cargo build --workspace
```

## Running Tests

```bash
cargo test --workspace
```

## Code Quality

Before submitting changes, ensure the following pass:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

<!-- TODO: Document changeset workflow and contribution process -->
