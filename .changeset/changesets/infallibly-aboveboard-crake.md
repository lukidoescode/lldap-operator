---
lldap-operator-helm-chart: patch
lldap-operator-crds: minor
---
Define `LldapUser`, `LldapGroup`, `LldapMembership`, and `LldapAttributeSchema` CRDs generated from Rust types, with CEL validation requiring the `lldap-operator.lukidoescode.com/lldap-instance` label, conditional `passwordSecretRef` on `LldapUser`, and email format checking
