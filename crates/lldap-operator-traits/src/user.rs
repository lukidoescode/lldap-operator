use std::future::Future;

use crate::attribute::AttributeValue;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub uuid: String,
    pub creation_date: String,
    pub attributes: Vec<AttributeValue>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdateUserInput {
    pub username: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub attributes: Option<Vec<AttributeValue>>,
}

pub trait UserClient {
    type Error: std::error::Error + Send + Sync + 'static;

    fn get_user(&self, username: &str) -> impl Future<Output = Result<User, Self::Error>> + Send;

    fn list_users(&self) -> impl Future<Output = Result<Vec<User>, Self::Error>> + Send;

    fn create_user(
        &self,
        input: &CreateUserInput,
    ) -> impl Future<Output = Result<User, Self::Error>> + Send;

    fn update_user(
        &self,
        input: &UpdateUserInput,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn delete_user(&self, username: &str) -> impl Future<Output = Result<(), Self::Error>> + Send;
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

    struct MockUserClient;

    fn sample_user() -> User {
        User {
            username: "jdoe".to_owned(),
            email: "jdoe@example.com".to_owned(),
            display_name: Some("John Doe".to_owned()),
            first_name: Some("John".to_owned()),
            last_name: Some("Doe".to_owned()),
            uuid: "550e8400-e29b-41d4-a716-446655440000".to_owned(),
            creation_date: "2025-01-01T00:00:00Z".to_owned(),
            attributes: vec![],
        }
    }

    impl UserClient for MockUserClient {
        type Error = MockError;

        async fn get_user(&self, _username: &str) -> Result<User, Self::Error> {
            Ok(sample_user())
        }

        async fn list_users(&self) -> Result<Vec<User>, Self::Error> {
            Ok(vec![sample_user()])
        }

        async fn create_user(&self, _input: &CreateUserInput) -> Result<User, Self::Error> {
            Ok(sample_user())
        }

        async fn update_user(&self, _input: &UpdateUserInput) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn delete_user(&self, _username: &str) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn get_user_returns_user() {
        let client = MockUserClient;
        let user = client.get_user("jdoe").await.expect("should succeed");
        assert_eq!(user.username, "jdoe");
        assert_eq!(user.email, "jdoe@example.com");
    }

    #[tokio::test]
    async fn list_users_returns_users() {
        let client = MockUserClient;
        let users = client.list_users().await.expect("should succeed");
        assert_eq!(users.len(), 1);
    }

    #[tokio::test]
    async fn create_user_returns_created_user() {
        let client = MockUserClient;
        let input = CreateUserInput {
            username: "jdoe".to_owned(),
            email: "jdoe@example.com".to_owned(),
            display_name: None,
            first_name: None,
            last_name: None,
        };
        let user = client.create_user(&input).await.expect("should succeed");
        assert_eq!(user.username, "jdoe");
    }

    #[tokio::test]
    async fn update_user_succeeds() {
        let client = MockUserClient;
        let input = UpdateUserInput {
            username: "jdoe".to_owned(),
            email: Some("new@example.com".to_owned()),
            display_name: None,
            first_name: None,
            last_name: None,
            attributes: None,
        };
        client.update_user(&input).await.expect("should succeed");
    }

    #[tokio::test]
    async fn delete_user_succeeds() {
        let client = MockUserClient;
        client.delete_user("jdoe").await.expect("should succeed");
    }
}
