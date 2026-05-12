# LldapUser

An `LldapUser` resource declares an lldap user account managed by the operator.

Each resource must carry the `lldap-operator.lukidoescode.com/lldap-instance`
label whose value selects which lldap instance the user belongs to. The operator
ignores resources without this label.

## Example

```yaml
{{#include ../../examples/lldap-user.yaml}}
```

## Spec Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `username` | string | yes | LDAP username (1–64 chars, `[a-zA-Z0-9._-]`) |
| `email` | string | yes | Email address |
| `displayName` | string | no | Display name |
| `firstName` | string | no | First name |
| `lastName` | string | no | Last name |
| `attributes` | AttributeValue[] | no | Custom attribute values; each requires a corresponding `LldapAttributeSchema` |
| `passwordPolicy` | string | no | One of `Manage` (default), `InitialOnly`, `Ignore` |
| `passwordSecretRef` | object | no | Reference to a Secret containing the password |
| `passwordSecretRef.name` | string | yes (when set) | Name of the Secret |
| `passwordSecretRef.key` | string | yes (when set) | Key within the Secret containing the password |

Group memberships are not declared on `LldapUser`. Use `LldapMembership` to
attach users to groups.

## Status

| Field | Type | Description |
|-------|------|-------------|
| `uuid` | string | User UUID assigned by lldap |
| `passwordHash` | string | Hash of the last reconciled password, used to detect Secret changes |
| `observedGeneration` | int64 | Last observed resource generation |
| `conditions` | Condition[] | Standard Kubernetes conditions |
