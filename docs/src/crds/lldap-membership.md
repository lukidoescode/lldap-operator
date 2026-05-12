# LldapMembership

An `LldapMembership` resource binds an lldap user to an lldap group.

Each resource must carry the `lldap-operator.lukidoescode.com/lldap-instance`
label whose value selects which lldap instance the membership applies to.

The `userName` and `groupName` fields reference the lldap-side identifiers
(the `spec.username` of an `LldapUser` and the `spec.displayName` of an
`LldapGroup`), not Kubernetes `metadata.name` values.

## Example

```yaml
{{#include ../../examples/lldap-membership.yaml}}
```

## Spec Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `userName` | string | yes | lldap username (1–64 chars) |
| `groupName` | string | yes | lldap group display name (1–128 chars) |

## Status

| Field | Type | Description |
|-------|------|-------------|
| `observedGeneration` | int64 | Last observed resource generation |
| `conditions` | Condition[] | Standard Kubernetes conditions |
