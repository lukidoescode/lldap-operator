# lldap-operator

[![CI](https://github.com/lukidoescode/lldap-operator/actions/workflows/ci.yml/badge.svg)](https://github.com/lukidoescode/lldap-operator/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Docs](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://lukidoescode.github.io/lldap-operator/)

A Kubernetes operator that manages [lldap](https://github.com/lldap/lldap)
users and groups as native Kubernetes resources via Custom Resource Definitions
(CRDs).

> **Note:** This project is in early development. APIs are not stable.

## Features

- Declare lldap users and groups as Kubernetes CRDs
- Reconcile desired state with lldap via its GraphQL API
- Password management through Kubernetes Secret references
- Standard operator patterns: status conditions, finalizers, observed generation

## Installation

Install using Helm:

```bash
helm repo add lldap-operator https://lukidoescode.github.io/lldap-operator
helm install lldap-operator lldap-operator/lldap-operator
```

## Usage

### Creating a group

```yaml
apiVersion: lldap-operator.lukidoescode.com/v1alpha1
kind: LldapGroup
metadata:
  name: my-group
spec:
  displayName: "My Group"
```

### Creating a user

```yaml
apiVersion: lldap-operator.lukidoescode.com/v1alpha1
kind: LldapUser
metadata:
  name: my-user
spec:
  username: "my.user"
  email: "my.user@example.com"
  displayName: "My User"
  firstName: "My"
  lastName: "User"
  groups:
    - "my-group"
  passwordSecretRef:
    name: my-user-password
    key: password
```

## Architecture

The operator is built in Rust using [kube-rs](https://github.com/kube-rs/kube)
and organized as a Cargo workspace:

| Crate | Purpose |
|-------|---------|
| `lldap-operator` | Operator binary |
| `lldap-operator-crds` | CRD type definitions |
| `lldap-operator-traits` | Trait definitions for lldap operations |
| `lldap-operator-client` | GraphQL client for lldap |
| `lldap-operator-reconciler` | Reconciliation business logic |

## Development

### Prerequisites

- Rust 1.85+
- Docker
- [kind](https://kind.sigs.k8s.io/)
- [Helm](https://helm.sh/)
- kubectl

### Local setup

```bash
./scripts/local-dev-setup.sh
```

This creates a `kind` cluster, deploys an lldap test instance, and installs the
CRDs.

## License

[MIT](LICENSE)
