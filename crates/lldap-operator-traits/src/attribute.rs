use std::future::Future;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttributeType {
    String,
    Integer,
    JpegPhoto,
    DateTime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttributeSchema {
    pub name: String,
    pub attribute_type: AttributeType,
    pub is_list: bool,
    pub is_visible: bool,
    pub is_editable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddAttributeInput {
    pub name: String,
    pub attribute_type: AttributeType,
    pub is_list: bool,
    pub is_visible: bool,
    pub is_editable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttributeValue {
    pub name: String,
    pub value: Vec<String>,
}

pub trait AttributeSchemaClient {
    type Error: std::error::Error + Send + Sync + 'static;

    fn list_user_attribute_schema(
        &self,
    ) -> impl Future<Output = Result<Vec<AttributeSchema>, Self::Error>> + Send;

    fn list_group_attribute_schema(
        &self,
    ) -> impl Future<Output = Result<Vec<AttributeSchema>, Self::Error>> + Send;

    fn add_user_attribute(
        &self,
        input: &AddAttributeInput,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn add_group_attribute(
        &self,
        input: &AddAttributeInput,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn delete_user_attribute(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn delete_group_attribute(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::*;

    #[derive(Debug)]
    struct MockError;

    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("mock error")
        }
    }

    impl std::error::Error for MockError {}

    struct MockAttributeSchemaClient;

    impl AttributeSchemaClient for MockAttributeSchemaClient {
        type Error = MockError;

        async fn list_user_attribute_schema(&self) -> Result<Vec<AttributeSchema>, Self::Error> {
            Ok(vec![AttributeSchema {
                name: "phone".to_owned(),
                attribute_type: AttributeType::String,
                is_list: false,
                is_visible: true,
                is_editable: true,
            }])
        }

        async fn list_group_attribute_schema(&self) -> Result<Vec<AttributeSchema>, Self::Error> {
            Ok(vec![])
        }

        async fn add_user_attribute(&self, _input: &AddAttributeInput) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn add_group_attribute(&self, _input: &AddAttributeInput) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn delete_user_attribute(&self, _name: &str) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn delete_group_attribute(&self, _name: &str) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn list_user_attribute_schema_returns_schemas() {
        let client = MockAttributeSchemaClient;
        let schemas = client
            .list_user_attribute_schema()
            .await
            .expect("should succeed");
        assert_eq!(schemas.len(), 1);
        assert_eq!(schemas[0].name, "phone");
        assert_eq!(schemas[0].attribute_type, AttributeType::String);
    }

    #[tokio::test]
    async fn list_group_attribute_schema_returns_empty() {
        let client = MockAttributeSchemaClient;
        let schemas = client
            .list_group_attribute_schema()
            .await
            .expect("should succeed");
        assert!(schemas.is_empty());
    }

    #[tokio::test]
    async fn add_user_attribute_succeeds() {
        let client = MockAttributeSchemaClient;
        let input = AddAttributeInput {
            name: "department".to_owned(),
            attribute_type: AttributeType::String,
            is_list: false,
            is_visible: true,
            is_editable: true,
        };
        client
            .add_user_attribute(&input)
            .await
            .expect("should succeed");
    }

    #[tokio::test]
    async fn add_group_attribute_succeeds() {
        let client = MockAttributeSchemaClient;
        let input = AddAttributeInput {
            name: "location".to_owned(),
            attribute_type: AttributeType::String,
            is_list: false,
            is_visible: true,
            is_editable: false,
        };
        client
            .add_group_attribute(&input)
            .await
            .expect("should succeed");
    }

    #[tokio::test]
    async fn delete_user_attribute_succeeds() {
        let client = MockAttributeSchemaClient;
        client
            .delete_user_attribute("phone")
            .await
            .expect("should succeed");
    }

    #[tokio::test]
    async fn delete_group_attribute_succeeds() {
        let client = MockAttributeSchemaClient;
        client
            .delete_group_attribute("location")
            .await
            .expect("should succeed");
    }
}
