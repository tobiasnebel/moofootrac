pub mod login_handler;
pub mod hello_handler;
pub mod moofoolog_handler;

pub type UserName = String;

pub struct WithResolvedUserName<T> {
    pub user_name: String,
    pub data: T,
}

impl<T> WithResolvedUserName<T> {
    pub fn with_data_and_user(user_name: UserName, data: T) -> Self {
        Self { user_name, data }
    }
}
