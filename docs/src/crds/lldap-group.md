# LldapGroup

An `LldapGroup` resource declares an lldap group managed by the operator.

## Example

```yaml
{{#include ../../examples/lldap-group.yaml}}
```

<!-- TODO: Document behavior details and interaction with LldapUser group references -->

## Spec Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `displayName` | string | yes | Group display name |

## Status

| Field | Type | Description |
|-------|------|-------------|
| `groupId` | integer | Group ID assigned by lldap |
| `uuid` | string | Group UUID assigned by lldap |
| `observedGeneration` | int64 | Last observed resource generation |
| `conditions` | Condition[] | Standard Kubernetes conditions |
