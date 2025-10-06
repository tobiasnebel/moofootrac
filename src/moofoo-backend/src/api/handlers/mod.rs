pub mod hello_handler;
pub mod login_handler;
pub mod moofoolog_handler;

pub type UserId = String;

pub struct WithResolvedUserId<T> {
    pub user_id: String,
    pub data: T,
}

impl<T> WithResolvedUserId<T> {
    pub fn with_data_and_user(user_id: UserId, data: T) -> Self {
        Self { user_id, data }
    }
}

/// that header name, yikes
pub const TOKEN_HEADER_NAME: &str = "MooFoo-Token";
