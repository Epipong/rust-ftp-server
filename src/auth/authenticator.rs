use libunftp::auth::{AuthenticationError, Authenticator, Credentials, DefaultUser};

#[derive(Debug)]
pub struct MyAuthenticator {
    username: String,
    password: String,
}

impl Default for MyAuthenticator {
    fn default() -> Self {
        let username = std::env::var("FTP_USER").expect("FTP_USER must be set.");
        let password = std::env::var("FTP_PWD").expect("FTP_PWD must be set.");
        MyAuthenticator { username, password }
    }
}

impl Authenticator<DefaultUser> for MyAuthenticator {
    fn name(&self) -> &str {
        self.username.as_str()
    }

    fn cert_auth_sufficient<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _username: &'life1 str,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = bool> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move { false })
    }

    #[doc = "Authenticate the given user with the given credentials."]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn authenticate<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        username: &'life1 str,
        creds: &'life2 Credentials,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<DefaultUser, AuthenticationError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if username == self.username && creds.password == Some(self.password.to_owned()) {
                Ok(DefaultUser)
            } else {
                Err(AuthenticationError::new("Invalid credentials"))
            }
        })
    }
}
