# Introduction

**lldap-operator** is a Kubernetes operator that manages
[lldap](https://github.com/lldap/lldap) users and groups as native Kubernetes
resources via Custom Resource Definitions (CRDs).

> **Note:** This project is in early development. APIs are not yet stable.

## Features

- Declare lldap users and groups as Kubernetes CRDs
- Reconcile desired state with lldap via its GraphQL API
- Password management through Kubernetes Secret references
- Standard operator patterns: status conditions, finalizers, observed generation

<!-- TODO: Expand with a getting-started overview once the operator is functional -->
