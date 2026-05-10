# Architecture

This chapter is for contributors who want to understand how the operator is
structured internally.

## Workspace Structure

The operator is organized as a Cargo workspace with focused crates:

| Crate | Purpose |
|-------|---------|
| `lldap-operator` | Operator binary (entry point, CLI, controller setup) |
| `lldap-operator-crds` | CRD type definitions (`CustomResource` derives) |
| `lldap-operator-traits` | Trait definitions for lldap operations |
| `lldap-operator-client` | GraphQL client for the lldap API |
| `lldap-operator-reconciler` | Reconciliation business logic |

<!-- TODO: Add reconciliation flow diagram -->
<!-- TODO: Document error handling patterns and trait boundaries -->
