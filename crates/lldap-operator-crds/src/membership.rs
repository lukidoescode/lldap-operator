use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::{CustomResource, KubeSchema};
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, KubeSchema)]
#[kube(
    group = "lldap-operator.lukidoescode.com",
    version = "v1alpha1",
    kind = "LldapMembership",
    plural = "lldapmemberships",
    shortname = "llm",
    namespaced,
    status = "LldapMembershipStatus",
    derive = "PartialEq",
    printcolumn = r#"{"name":"User","type":"string","jsonPath":".spec.userName"}"#,
    printcolumn = r#"{"name":"Group","type":"string","jsonPath":".spec.groupName"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#,
    validation = Rule::new(
        "has(self.metadata.labels) && 'lldap-operator.lukidoescode.com/lldap-instance' in self.metadata.labels && size(self.metadata.labels['lldap-operator.lukidoescode.com/lldap-instance']) > 0"
    ).message("metadata.labels must include 'lldap-operator.lukidoescode.com/lldap-instance' with a non-empty value").reason(Reason::FieldValueRequired),
)]
#[serde(rename_all = "camelCase")]
pub struct LldapMembershipSpec {
    #[schemars(length(min = 1, max = 64))]
    pub user_name: String,
    #[schemars(length(min = 1, max = 128))]
    pub group_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, KubeSchema)]
#[serde(rename_all = "camelCase")]
pub struct LldapMembershipStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

#[cfg(test)]
mod tests {
    use kube::CustomResourceExt;

    use super::*;

    fn spec_schema() -> serde_json::Value {
        let schema = serde_json::to_value(LldapMembership::crd()).expect("serialize CRD");
        schema["spec"]["versions"][0]["schema"]["openAPIV3Schema"]["properties"]["spec"].clone()
    }

    fn cel_rules() -> serde_json::Value {
        let schema = serde_json::to_value(LldapMembership::crd()).expect("serialize CRD");
        schema["spec"]["versions"][0]["schema"]["openAPIV3Schema"]["x-kubernetes-validations"]
            .clone()
    }

    #[test]
    fn crd_has_expected_group_version_kind() {
        let crd = LldapMembership::crd();
        assert_eq!(crd.spec.group, "lldap-operator.lukidoescode.com");
        assert_eq!(crd.spec.versions[0].name, "v1alpha1");
        assert_eq!(crd.spec.names.kind, "LldapMembership");
        assert_eq!(crd.spec.names.plural, "lldapmemberships");
    }

    #[test]
    fn spec_round_trips_through_yaml() {
        let spec = LldapMembershipSpec {
            user_name: "alice".to_owned(),
            group_name: "Engineering".to_owned(),
        };
        let yaml = serde_yaml_ng::to_string(&spec).expect("serialize");
        let back: LldapMembershipSpec = serde_yaml_ng::from_str(&yaml).expect("deserialize");
        assert_eq!(spec, back);
    }

    #[test]
    fn crd_yaml_round_trips() {
        let crd = LldapMembership::crd();
        let yaml = serde_yaml_ng::to_string(&crd).expect("serialize");
        let _: serde_yaml_ng::Value = serde_yaml_ng::from_str(&yaml).expect("parse");
    }

    #[test]
    fn user_name_schema_has_correct_length_constraints() {
        let schema = spec_schema();
        assert_eq!(schema["properties"]["userName"]["minLength"], 1);
        assert_eq!(schema["properties"]["userName"]["maxLength"], 64);
    }

    #[test]
    fn group_name_schema_has_correct_length_constraints() {
        let schema = spec_schema();
        assert_eq!(schema["properties"]["groupName"]["minLength"], 1);
        assert_eq!(schema["properties"]["groupName"]["maxLength"], 128);
    }

    #[test]
    fn crd_includes_instance_label_validation() {
        let yaml = serde_yaml_ng::to_string(&LldapMembership::crd()).expect("serialize");
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
    fn status_struct_defaults_to_all_none() {
        let status = LldapMembershipStatus::default();
        assert!(status.observed_generation.is_none());
        assert!(status.conditions.is_none());
    }
}
