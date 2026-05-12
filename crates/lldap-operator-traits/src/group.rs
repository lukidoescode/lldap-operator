use std::future::Future;

use crate::attribute::AttributeValue;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group {
    pub id: i64,
    pub display_name: String,
    pub uuid: String,
    pub attributes: Vec<AttributeValue>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateGroupInput {
    pub display_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdateGroupInput {
    pub id: i64,
    pub display_name: Option<String>,
    pub attributes: Option<Vec<AttributeValue>>,
}

pub trait GroupClient {
    type Error: std::error::Error + Send + Sync + 'static;

    fn get_group(&self, group_id: i64) -> impl Future<Output = Result<Group, Self::Error>> + Send;

    fn get_group_by_display_name(
        &self,
        display_name: &str,
    ) -> impl Future<Output = Result<Group, Self::Error>> + Send;

    fn list_groups(&self) -> impl Future<Output = Result<Vec<Group>, Self::Error>> + Send;

    fn create_group(
        &self,
        input: &CreateGroupInput,
    ) -> impl Future<Output = Result<Group, Self::Error>> + Send;

    fn update_group(
        &self,
        input: &UpdateGroupInput,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn delete_group(&self, group_id: i64) -> impl Future<Output = Result<(), Self::Error>> + Send;
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

    struct MockGroupClient;

    fn sample_group() -> Group {
        Group {
            id: 1,
            display_name: "engineers".to_owned(),
            uuid: "660e8400-e29b-41d4-a716-446655440000".to_owned(),
            attributes: vec![],
        }
    }

    impl GroupClient for MockGroupClient {
        type Error = MockError;

        async fn get_group(&self, _group_id: i64) -> Result<Group, Self::Error> {
            Ok(sample_group())
        }

        async fn get_group_by_display_name(
            &self,
            _display_name: &str,
        ) -> Result<Group, Self::Error> {
            Ok(sample_group())
        }

        async fn list_groups(&self) -> Result<Vec<Group>, Self::Error> {
            Ok(vec![sample_group()])
        }

        async fn create_group(&self, _input: &CreateGroupInput) -> Result<Group, Self::Error> {
            Ok(sample_group())
        }

        async fn update_group(&self, _input: &UpdateGroupInput) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn delete_group(&self, _group_id: i64) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn get_group_returns_group() {
        let client = MockGroupClient;
        let group = client.get_group(1).await.expect("should succeed");
        assert_eq!(group.id, 1);
        assert_eq!(group.display_name, "engineers");
    }

    #[tokio::test]
    async fn get_group_by_display_name_returns_group() {
        let client = MockGroupClient;
        let group = client
            .get_group_by_display_name("engineers")
            .await
            .expect("should succeed");
        assert_eq!(group.display_name, "engineers");
    }

    #[tokio::test]
    async fn list_groups_returns_groups() {
        let client = MockGroupClient;
        let groups = client.list_groups().await.expect("should succeed");
        assert_eq!(groups.len(), 1);
    }

    #[tokio::test]
    async fn create_group_returns_created_group() {
        let client = MockGroupClient;
        let input = CreateGroupInput {
            display_name: "engineers".to_owned(),
        };
        let group = client.create_group(&input).await.expect("should succeed");
        assert_eq!(group.display_name, "engineers");
    }

    #[tokio::test]
    async fn update_group_succeeds() {
        let client = MockGroupClient;
        let input = UpdateGroupInput {
            id: 1,
            display_name: Some("senior-engineers".to_owned()),
            attributes: None,
        };
        client.update_group(&input).await.expect("should succeed");
    }

    #[tokio::test]
    async fn delete_group_succeeds() {
        let client = MockGroupClient;
        client.delete_group(1).await.expect("should succeed");
    }
}
