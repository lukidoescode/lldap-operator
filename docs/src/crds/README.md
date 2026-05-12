# CRD Reference

The lldap-operator manages the following Custom Resource Definitions:

| Kind | Description | Status |
|------|-------------|--------|
| [`LldapUser`](lldap-user.md) | Manages lldap user accounts | Schema only |
| [`LldapGroup`](lldap-group.md) | Manages lldap groups | Schema only |
| [`LldapMembership`](lldap-membership.md) | Manages group membership relationships | Schema only |
| [`LldapAttributeSchema`](lldap-attribute-schema.md) | Manages custom attribute schemas | Schema only |

All resources use the API group
`lldap-operator.lukidoescode.com/v1alpha1`.
