# LldapGroup

An `LldapGroup` resource declares an lldap group managed by the operator.

Each resource must carry the `lldap-operator.lukidoescode.com/lldap-instance`
label whose value selects which lldap instance the group belongs to.

## Example

```yaml
{{#include ../../examples/lldap-group.yaml}}
```

## Spec Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `displayName` | string | yes | Group display name (1–128 chars) |
| `attributes` | AttributeValue[] | no | Custom attribute values; each requires a corresponding `LldapAttributeSchema` with `target: Group` |

## Status

| Field | Type | Description |
|-------|------|-------------|
| `groupId` | integer | Group ID assigned by lldap |
| `uuid` | string | Group UUID assigned by lldap |
| `observedGeneration` | int64 | Last observed resource generation |
| `conditions` | Condition[] | Standard Kubernetes conditions |
