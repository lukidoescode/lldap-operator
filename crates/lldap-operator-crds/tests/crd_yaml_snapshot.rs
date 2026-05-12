use std::fs;
use std::path::PathBuf;

use kube::CustomResourceExt;
use lldap_operator_crds::{LldapAttributeSchema, LldapGroup, LldapMembership, LldapUser};

fn snapshot_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../charts/lldap-operator/crds")
        .join(name)
}

fn assert_matches<T: CustomResourceExt>(name: &str) {
    let generated = serde_yaml_ng::to_string(&T::crd()).expect("serialize");
    let committed = fs::read_to_string(snapshot_path(name)).expect("read committed CRD");
    assert_eq!(
        generated, committed,
        "CRD YAML drift for {name} — run `cargo run -p lldap-operator-crds --bin crdgen`",
    );
}

#[test]
fn lldap_user_yaml_matches_committed() {
    assert_matches::<LldapUser>("lldap-user.yaml");
}

#[test]
fn lldap_group_yaml_matches_committed() {
    assert_matches::<LldapGroup>("lldap-group.yaml");
}

#[test]
fn lldap_membership_yaml_matches_committed() {
    assert_matches::<LldapMembership>("lldap-membership.yaml");
}

#[test]
fn lldap_attribute_schema_yaml_matches_committed() {
    assert_matches::<LldapAttributeSchema>("lldap-attribute-schema.yaml");
}
