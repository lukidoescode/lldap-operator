# LldapAttributeSchema

An `LldapAttributeSchema` resource declares a custom attribute on either the
user or group entity in lldap.

Each resource must carry the `lldap-operator.lukidoescode.com/lldap-instance`
label whose value selects which lldap instance owns the attribute schema.

lldap does not support mutating an attribute schema after creation: the
attribute can only be added or deleted. Updates to a schema's type or flags
require deleting and recreating the resource. Validation enforcing this
constraint is tracked separately.

## Example

```yaml
{{#include ../../examples/lldap-attribute-schema.yaml}}
```

## Spec Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attributeName` | string | yes | Attribute identifier (1–64 chars, `[a-zA-Z][a-zA-Z0-9_]*`) |
| `attributeType` | string | yes | One of `String`, `Integer`, `JpegPhoto`, `DateTime` |
| `target` | string | yes | One of `User`, `Group` |
| `isList` | bool | no | Whether the attribute holds multiple values |
| `isVisible` | bool | no | Whether the attribute is visible to LDAP clients |
| `isEditable` | bool | no | Whether the attribute is editable through the lldap UI |

## Status

| Field | Type | Description |
|-------|------|-------------|
| `observedGeneration` | int64 | Last observed resource generation |
| `conditions` | Condition[] | Standard Kubernetes conditions |
