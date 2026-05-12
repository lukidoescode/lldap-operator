use std::future::Future;

pub trait PasswordClient {
    type Error: std::error::Error + Send + Sync + 'static;

    fn set_password(
        &self,
        username: &str,
        password: &str,
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

    struct MockPasswordClient;

    impl PasswordClient for MockPasswordClient {
        type Error = MockError;

        async fn set_password(&self, _username: &str, _password: &str) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn set_password_succeeds() {
        let client = MockPasswordClient;
        client
            .set_password("jdoe", "s3cret")
            .await
            .expect("should succeed");
    }
}
