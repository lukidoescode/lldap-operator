use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::{CustomResource, KubeSchema};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum AttributeKind {
    String,
    Integer,
    JpegPhoto,
    DateTime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum AttributeTarget {
    User,
    Group,
}

#[derive(CustomResource, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, KubeSchema)]
#[kube(
    group = "lldap-operator.lukidoescode.com",
    version = "v1alpha1",
    kind = "LldapAttributeSchema",
    plural = "lldapattributeschemas",
    shortname = "llas",
    namespaced,
    status = "LldapAttributeSchemaStatus",
    derive = "PartialEq",
    printcolumn = r#"{"name":"Name","type":"string","jsonPath":".spec.attributeName"}"#,
    printcolumn = r#"{"name":"Type","type":"string","jsonPath":".spec.attributeType"}"#,
    printcolumn = r#"{"name":"Target","type":"string","jsonPath":".spec.target"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#,
    validation = Rule::new(
        "has(self.metadata.labels) && 'lldap-operator.lukidoescode.com/lldap-instance' in self.metadata.labels && size(self.metadata.labels['lldap-operator.lukidoescode.com/lldap-instance']) > 0"
    ).message("metadata.labels must include 'lldap-operator.lukidoescode.com/lldap-instance' with a non-empty value").reason(Reason::FieldValueRequired),
)]
#[serde(rename_all = "camelCase")]
pub struct LldapAttributeSchemaSpec {
    #[schemars(regex(pattern = r"^[a-zA-Z][a-zA-Z0-9_]*$"), length(min = 1, max = 64))]
    pub attribute_name: String,
    pub attribute_type: AttributeKind,
    pub target: AttributeTarget,
    #[serde(default)]
    pub is_list: bool,
    #[serde(default)]
    pub is_visible: bool,
    #[serde(default)]
    pub is_editable: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, KubeSchema)]
#[serde(rename_all = "camelCase")]
pub struct LldapAttributeSchemaStatus {
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
        let schema = serde_json::to_value(LldapAttributeSchema::crd()).expect("serialize CRD");
        schema["spec"]["versions"][0]["schema"]["openAPIV3Schema"]["properties"]["spec"].clone()
    }

    fn cel_rules() -> serde_json::Value {
        let schema = serde_json::to_value(LldapAttributeSchema::crd()).expect("serialize CRD");
        schema["spec"]["versions"][0]["schema"]["openAPIV3Schema"]["x-kubernetes-validations"]
            .clone()
    }

    #[test]
    fn crd_has_expected_group_version_kind() {
        let crd = LldapAttributeSchema::crd();
        assert_eq!(crd.spec.group, "lldap-operator.lukidoescode.com");
        assert_eq!(crd.spec.versions[0].name, "v1alpha1");
        assert_eq!(crd.spec.names.kind, "LldapAttributeSchema");
        assert_eq!(crd.spec.names.plural, "lldapattributeschemas");
    }

    #[test]
    fn attribute_kind_serializes_pascal_case() {
        assert_eq!(
            serde_json::to_string(&AttributeKind::String).expect("serialize"),
            "\"String\""
        );
        assert_eq!(
            serde_json::to_string(&AttributeKind::JpegPhoto).expect("serialize"),
            "\"JpegPhoto\""
        );
        assert_eq!(
            serde_json::to_string(&AttributeKind::DateTime).expect("serialize"),
            "\"DateTime\""
        );
    }

    #[test]
    fn attribute_kind_all_variants_round_trip() {
        let cases = [
            (AttributeKind::String, "\"String\""),
            (AttributeKind::Integer, "\"Integer\""),
            (AttributeKind::JpegPhoto, "\"JpegPhoto\""),
            (AttributeKind::DateTime, "\"DateTime\""),
        ];
        for (variant, expected_json) in cases {
            let json = serde_json::to_string(&variant).expect("serialize");
            assert_eq!(json, expected_json);
            let back: AttributeKind = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(variant, back);
        }
    }

    #[test]
    fn attribute_target_serializes_pascal_case() {
        assert_eq!(
            serde_json::to_string(&AttributeTarget::User).expect("serialize"),
            "\"User\""
        );
        assert_eq!(
            serde_json::to_string(&AttributeTarget::Group).expect("serialize"),
            "\"Group\""
        );
    }

    #[test]
    fn attribute_target_all_variants_round_trip() {
        let cases = [
            (AttributeTarget::User, "\"User\""),
            (AttributeTarget::Group, "\"Group\""),
        ];
        for (variant, expected_json) in cases {
            let json = serde_json::to_string(&variant).expect("serialize");
            assert_eq!(json, expected_json);
            let back: AttributeTarget = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(variant, back);
        }
    }

    #[test]
    fn spec_round_trips_through_yaml() {
        let spec = LldapAttributeSchemaSpec {
            attribute_name: "department".to_owned(),
            attribute_type: AttributeKind::String,
            target: AttributeTarget::User,
            is_list: false,
            is_visible: true,
            is_editable: true,
        };
        let yaml = serde_yaml_ng::to_string(&spec).expect("serialize");
        let back: LldapAttributeSchemaSpec = serde_yaml_ng::from_str(&yaml).expect("deserialize");
        assert_eq!(spec, back);
    }

    #[test]
    fn crd_yaml_round_trips() {
        let crd = LldapAttributeSchema::crd();
        let yaml = serde_yaml_ng::to_string(&crd).expect("serialize");
        let _: serde_yaml_ng::Value = serde_yaml_ng::from_str(&yaml).expect("parse");
    }

    #[test]
    fn attribute_name_schema_has_correct_pattern() {
        let schema = spec_schema();
        assert_eq!(
            schema["properties"]["attributeName"]["pattern"],
            serde_json::Value::String(r"^[a-zA-Z][a-zA-Z0-9_]*$".to_owned())
        );
    }

    #[test]
    fn attribute_name_schema_has_correct_length_constraints() {
        let schema = spec_schema();
        assert_eq!(schema["properties"]["attributeName"]["minLength"], 1);
        assert_eq!(schema["properties"]["attributeName"]["maxLength"], 64);
    }

    #[test]
    fn attribute_kind_schema_has_all_enum_variants() {
        let schema = spec_schema();
        let variants = schema["properties"]["attributeType"]["enum"]
            .as_array()
            .expect("attributeType enum is an array");
        let variant_strs: std::collections::HashSet<&str> =
            variants.iter().filter_map(|v| v.as_str()).collect();
        assert!(variant_strs.contains("String"));
        assert!(variant_strs.contains("Integer"));
        assert!(variant_strs.contains("JpegPhoto"));
        assert!(variant_strs.contains("DateTime"));
    }

    #[test]
    fn attribute_target_schema_has_all_enum_variants() {
        let schema = spec_schema();
        let variants = schema["properties"]["target"]["enum"]
            .as_array()
            .expect("target enum is an array");
        let variant_strs: std::collections::HashSet<&str> =
            variants.iter().filter_map(|v| v.as_str()).collect();
        assert!(variant_strs.contains("User"));
        assert!(variant_strs.contains("Group"));
    }

    #[test]
    fn boolean_fields_default_to_false() {
        let yaml = "attributeName: x\nattributeType: String\ntarget: User\n";
        let spec: LldapAttributeSchemaSpec = serde_yaml_ng::from_str(yaml).expect("deserialize");
        assert!(!spec.is_list);
        assert!(!spec.is_visible);
        assert!(!spec.is_editable);
    }

    #[test]
    fn crd_includes_instance_label_validation() {
        let yaml = serde_yaml_ng::to_string(&LldapAttributeSchema::crd()).expect("serialize");
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
        let status = LldapAttributeSchemaStatus::default();
        assert!(status.observed_generation.is_none());
        assert!(status.conditions.is_none());
    }
}
