use thiserror::Error;

use async_graphql::{ErrorExtensions, FieldError};

#[derive(Debug, Error)]
pub enum GraphQLError {
    #[error("User not found")]
    UserNotFound
}

impl ErrorExtensions for GraphQLError {
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            GraphQLError::UserNotFound => e.set("code", "USER_NOT_FOUND"),
        })
    }
}
