# LldapUser

An `LldapUser` resource declares an lldap user account managed by the operator.

## Example

```yaml
{{#include ../../examples/lldap-user.yaml}}
```

<!-- TODO: Document spec fields, status fields, and behavior details -->

## Spec Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `username` | string | yes | LDAP username |
| `email` | string | yes | Email address |
| `displayName` | string | no | Display name |
| `firstName` | string | no | First name |
| `lastName` | string | no | Last name |
| `groups` | string[] | no | Group memberships by name |
| `passwordSecretRef` | object | yes | Reference to a Secret containing the password |
| `passwordSecretRef.name` | string | yes | Name of the Secret |
| `passwordSecretRef.key` | string | yes | Key within the Secret containing the password |

## Status

| Field | Type | Description |
|-------|------|-------------|
| `uuid` | string | User UUID assigned by lldap |
| `observedGeneration` | int64 | Last observed resource generation |
| `conditions` | Condition[] | Standard Kubernetes conditions |
