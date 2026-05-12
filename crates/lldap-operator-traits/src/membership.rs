use std::future::Future;

pub trait MembershipClient {
    type Error: std::error::Error + Send + Sync + 'static;

    fn add_user_to_group(
        &self,
        username: &str,
        group_id: i64,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn remove_user_from_group(
        &self,
        username: &str,
        group_id: i64,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn check_membership(
        &self,
        username: &str,
        group_id: i64,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
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

    struct MockMembershipClient;

    impl MembershipClient for MockMembershipClient {
        type Error = MockError;

        async fn add_user_to_group(
            &self,
            _username: &str,
            _group_id: i64,
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn remove_user_from_group(
            &self,
            _username: &str,
            _group_id: i64,
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn check_membership(
            &self,
            _username: &str,
            _group_id: i64,
        ) -> Result<bool, Self::Error> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn add_user_to_group_succeeds() {
        let client = MockMembershipClient;
        client
            .add_user_to_group("jdoe", 1)
            .await
            .expect("should succeed");
    }

    #[tokio::test]
    async fn remove_user_from_group_succeeds() {
        let client = MockMembershipClient;
        client
            .remove_user_from_group("jdoe", 1)
            .await
            .expect("should succeed");
    }

    #[tokio::test]
    async fn check_membership_returns_true() {
        let client = MockMembershipClient;
        let is_member = client
            .check_membership("jdoe", 1)
            .await
            .expect("should succeed");
        assert!(is_member);
    }
}
