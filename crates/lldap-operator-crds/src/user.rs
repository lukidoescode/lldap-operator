use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::{CustomResource, KubeSchema};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::{AttributeValue, SecretKeyRef};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum PasswordPolicy {
    #[default]
    Manage,
    InitialOnly,
    Ignore,
}

#[derive(CustomResource, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, KubeSchema)]
#[kube(
    group = "lldap-operator.lukidoescode.com",
    version = "v1alpha1",
    kind = "LldapUser",
    plural = "lldapusers",
    shortname = "llu",
    namespaced,
    status = "LldapUserStatus",
    derive = "PartialEq",
    printcolumn = r#"{"name":"Username","type":"string","jsonPath":".spec.username"}"#,
    printcolumn = r#"{"name":"Email","type":"string","jsonPath":".spec.email"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#,
    validation = Rule::new(
        "has(self.metadata.labels) && 'lldap-operator.lukidoescode.com/lldap-instance' in self.metadata.labels && size(self.metadata.labels['lldap-operator.lukidoescode.com/lldap-instance']) > 0"
    ).message("metadata.labels must include 'lldap-operator.lukidoescode.com/lldap-instance' with a non-empty value").reason(Reason::FieldValueRequired),
    validation = Rule::new(
        "self.spec.passwordPolicy == 'Ignore' || has(self.spec.passwordSecretRef)"
    ).message("passwordSecretRef is required unless passwordPolicy is Ignore").reason(Reason::FieldValueRequired),
)]
#[serde(rename_all = "camelCase")]
pub struct LldapUserSpec {
    #[schemars(regex(pattern = r"^[a-zA-Z0-9._-]+$"), length(min = 1, max = 64))]
    pub username: String,
    #[schemars(
        regex(pattern = r"^[^@\s]+@[^@\s]+\.[^@\s]+$"),
        length(min = 3, max = 320)
    )]
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeValue>>,
    #[serde(default)]
    pub password_policy: PasswordPolicy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password_secret_ref: Option<SecretKeyRef>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, KubeSchema)]
#[serde(rename_all = "camelCase")]
pub struct LldapUserStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

#[cfg(test)]
mod tests {
    use kube::CustomResourceExt;

    use super::*;

    fn spec_schema() -> serde_json::Value {
        let schema = serde_json::to_value(LldapUser::crd()).expect("serialize CRD");
        schema["spec"]["versions"][0]["schema"]["openAPIV3Schema"]["properties"]["spec"].clone()
    }

    fn cel_rules() -> serde_json::Value {
        let schema = serde_json::to_value(LldapUser::crd()).expect("serialize CRD");
        schema["spec"]["versions"][0]["schema"]["openAPIV3Schema"]["x-kubernetes-validations"]
            .clone()
    }

    #[test]
    fn crd_has_expected_group_version_kind() {
        let crd = LldapUser::crd();
        assert_eq!(crd.spec.group, "lldap-operator.lukidoescode.com");
        assert_eq!(crd.spec.versions[0].name, "v1alpha1");
        assert_eq!(crd.spec.names.kind, "LldapUser");
        assert_eq!(crd.spec.names.plural, "lldapusers");
    }

    #[test]
    fn spec_round_trips_through_yaml() {
        let spec = LldapUserSpec {
            username: "alice".to_owned(),
            email: "alice@example.com".to_owned(),
            display_name: Some("Alice".to_owned()),
            first_name: None,
            last_name: None,
            attributes: None,
            password_policy: PasswordPolicy::Manage,
            password_secret_ref: Some(SecretKeyRef {
                name: "alice-pw".to_owned(),
                key: "password".to_owned(),
            }),
        };
        let yaml = serde_yaml_ng::to_string(&spec).expect("serialize");
        let back: LldapUserSpec = serde_yaml_ng::from_str(&yaml).expect("deserialize");
        assert_eq!(spec, back);
    }

    #[test]
    fn spec_round_trips_with_attributes() {
        let spec = LldapUserSpec {
            username: "alice".to_owned(),
            email: "alice@example.com".to_owned(),
            display_name: None,
            first_name: None,
            last_name: None,
            attributes: Some(vec![AttributeValue {
                name: "phone".to_owned(),
                value: vec!["555-1234".to_owned(), "555-5678".to_owned()],
            }]),
            password_policy: PasswordPolicy::Manage,
            password_secret_ref: Some(SecretKeyRef {
                name: "alice-pw".to_owned(),
                key: "password".to_owned(),
            }),
        };
        let yaml = serde_yaml_ng::to_string(&spec).expect("serialize");
        let back: LldapUserSpec = serde_yaml_ng::from_str(&yaml).expect("deserialize");
        assert_eq!(spec, back);
    }

    #[test]
    fn spec_round_trips_with_all_optional_fields() {
        let spec = LldapUserSpec {
            username: "alice".to_owned(),
            email: "alice@example.com".to_owned(),
            display_name: Some("Alice Smith".to_owned()),
            first_name: Some("Alice".to_owned()),
            last_name: Some("Smith".to_owned()),
            attributes: Some(vec![AttributeValue {
                name: "department".to_owned(),
                value: vec!["Engineering".to_owned()],
            }]),
            password_policy: PasswordPolicy::Manage,
            password_secret_ref: Some(SecretKeyRef {
                name: "alice-pw".to_owned(),
                key: "password".to_owned(),
            }),
        };
        let yaml = serde_yaml_ng::to_string(&spec).expect("serialize");
        let back: LldapUserSpec = serde_yaml_ng::from_str(&yaml).expect("deserialize");
        assert_eq!(spec, back);
    }

    #[test]
    fn spec_round_trips_with_ignore_policy_no_secret() {
        let spec = LldapUserSpec {
            username: "alice".to_owned(),
            email: "alice@example.com".to_owned(),
            display_name: None,
            first_name: None,
            last_name: None,
            attributes: None,
            password_policy: PasswordPolicy::Ignore,
            password_secret_ref: None,
        };
        let yaml = serde_yaml_ng::to_string(&spec).expect("serialize");
        let back: LldapUserSpec = serde_yaml_ng::from_str(&yaml).expect("deserialize");
        assert_eq!(spec, back);
    }

    #[test]
    fn password_policy_defaults_to_manage() {
        assert_eq!(PasswordPolicy::default(), PasswordPolicy::Manage);
    }

    #[test]
    fn password_policy_all_variants_round_trip() {
        for policy in [
            PasswordPolicy::Manage,
            PasswordPolicy::InitialOnly,
            PasswordPolicy::Ignore,
        ] {
            let json = serde_json::to_string(&policy).expect("serialize");
            let back: PasswordPolicy = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(policy, back);
        }
    }

    #[test]
    fn crd_yaml_round_trips() {
        let crd = LldapUser::crd();
        let yaml = serde_yaml_ng::to_string(&crd).expect("serialize");
        let _: serde_yaml_ng::Value = serde_yaml_ng::from_str(&yaml).expect("parse");
    }

    #[test]
    fn username_schema_has_correct_pattern() {
        let schema = spec_schema();
        assert_eq!(
            schema["properties"]["username"]["pattern"],
            serde_json::Value::String(r"^[a-zA-Z0-9._-]+$".to_owned())
        );
    }

    #[test]
    fn username_schema_has_correct_length_constraints() {
        let schema = spec_schema();
        assert_eq!(schema["properties"]["username"]["minLength"], 1);
        assert_eq!(schema["properties"]["username"]["maxLength"], 64);
    }

    #[test]
    fn email_schema_has_correct_pattern() {
        let schema = spec_schema();
        assert_eq!(
            schema["properties"]["email"]["pattern"],
            serde_json::Value::String(r"^[^@\s]+@[^@\s]+\.[^@\s]+$".to_owned())
        );
    }

    #[test]
    fn email_schema_has_correct_length_constraints() {
        let schema = spec_schema();
        assert_eq!(schema["properties"]["email"]["minLength"], 3);
        assert_eq!(schema["properties"]["email"]["maxLength"], 320);
    }

    #[test]
    fn password_policy_schema_has_all_enum_variants() {
        let schema = spec_schema();
        let variants = schema["properties"]["passwordPolicy"]["enum"]
            .as_array()
            .expect("passwordPolicy enum is an array");
        let variant_strs: std::collections::HashSet<&str> =
            variants.iter().filter_map(|v| v.as_str()).collect();
        assert!(variant_strs.contains("Manage"));
        assert!(variant_strs.contains("InitialOnly"));
        assert!(variant_strs.contains("Ignore"));
    }

    #[test]
    fn crd_includes_instance_label_validation() {
        let yaml = serde_yaml_ng::to_string(&LldapUser::crd()).expect("serialize");
        assert!(yaml.contains("x-kubernetes-validations"));
        assert!(yaml.contains("lldap-operator.lukidoescode.com/lldap-instance"));
    }

    #[test]
    fn cel_rule_exact_wording_instance_label() {
        let rules = cel_rules();
        let rules = rules
            .as_array()
            .expect("x-kubernetes-validations is an array");
        let label_rule = rules
            .iter()
            .find(|r| r["message"].as_str() == Some("metadata.labels must include 'lldap-operator.lukidoescode.com/lldap-instance' with a non-empty value"))
            .expect("instance label CEL rule present");
        assert_eq!(
            label_rule["rule"].as_str().expect("rule string"),
            "has(self.metadata.labels) && 'lldap-operator.lukidoescode.com/lldap-instance' in self.metadata.labels && size(self.metadata.labels['lldap-operator.lukidoescode.com/lldap-instance']) > 0"
        );
    }

    #[test]
    fn crd_includes_password_secret_ref_validation() {
        let yaml = serde_yaml_ng::to_string(&LldapUser::crd()).expect("serialize");
        assert!(yaml.contains("passwordSecretRef is required unless passwordPolicy is Ignore"));
        assert!(yaml.contains("self.spec.passwordPolicy == 'Ignore'"));
    }

    #[test]
    fn cel_rule_exact_wording_password_secret_ref() {
        let rules = cel_rules();
        let rules = rules
            .as_array()
            .expect("x-kubernetes-validations is an array");
        let pw_rule = rules
            .iter()
            .find(|r| {
                r["message"].as_str()
                    == Some("passwordSecretRef is required unless passwordPolicy is Ignore")
            })
            .expect("password secret ref CEL rule present");
        assert_eq!(
            pw_rule["rule"].as_str().expect("rule string"),
            "self.spec.passwordPolicy == 'Ignore' || has(self.spec.passwordSecretRef)"
        );
    }

    #[test]
    fn status_struct_defaults_to_all_none() {
        let status = LldapUserStatus::default();
        assert!(status.uuid.is_none());
        assert!(status.observed_generation.is_none());
        assert!(status.password_hash.is_none());
        assert!(status.conditions.is_none());
    }
}
