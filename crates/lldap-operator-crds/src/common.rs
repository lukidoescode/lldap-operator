use kube::KubeSchema;
use serde::{Deserialize, Serialize};

pub const INSTANCE_LABEL: &str = "lldap-operator.lukidoescode.com/lldap-instance";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, KubeSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttributeValue {
    pub name: String,
    pub value: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, KubeSchema)]
#[serde(rename_all = "camelCase")]
pub struct SecretKeyRef {
    pub name: String,
    pub key: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attribute_value_round_trips_through_json() {
        let value = AttributeValue {
            name: "phone".to_owned(),
            value: vec!["+1234".to_owned()],
        };
        let json = serde_json::to_string(&value).expect("serialize");
        let back: AttributeValue = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(value, back);
    }

    #[test]
    fn attribute_value_round_trips_with_multiple_values() {
        let value = AttributeValue {
            name: "phone".to_owned(),
            value: vec!["+1234".to_owned(), "+5678".to_owned(), "+9012".to_owned()],
        };
        let json = serde_json::to_string(&value).expect("serialize");
        let back: AttributeValue = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(value, back);
    }

    #[test]
    fn instance_label_constant_value() {
        assert_eq!(
            INSTANCE_LABEL,
            "lldap-operator.lukidoescode.com/lldap-instance"
        );
    }

    #[test]
    fn secret_key_ref_uses_camel_case() {
        let value = SecretKeyRef {
            name: "secret".to_owned(),
            key: "password".to_owned(),
        };
        let json = serde_json::to_string(&value).expect("serialize");
        assert!(json.contains("\"name\""));
        assert!(json.contains("\"key\""));
    }

    #[test]
    fn secret_key_ref_round_trips() {
        let value = SecretKeyRef {
            name: "my-secret".to_owned(),
            key: "password".to_owned(),
        };
        let json = serde_json::to_string(&value).expect("serialize");
        let back: SecretKeyRef = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(value, back);
    }
}
