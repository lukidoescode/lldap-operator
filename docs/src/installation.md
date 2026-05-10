# Installation

<!-- TODO: Expand each section once the Helm chart is published -->

## Prerequisites

- A running Kubernetes cluster (v1.26+)
- [Helm](https://helm.sh/) v3
- An accessible [lldap](https://github.com/lldap/lldap) instance

## Quick Start

```bash
helm repo add lldap-operator \
  https://lukidoescode.github.io/lldap-operator
helm install lldap-operator \
  lldap-operator/lldap-operator
```

## Configuration

See the [Configuration Reference](configuration.md) for all available Helm
values.
