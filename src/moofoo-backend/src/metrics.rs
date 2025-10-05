use axum_prometheus::metrics::{Counter, counter};

#[derive(Clone)]
pub struct AppMetrics {
    pub invalid_token_counter: Counter,
    pub missing_token_counter: Counter,
    pub successful_login_counter: Counter,
}

impl AppMetrics {
    pub fn init() -> Self {
        // initialize metrics
        let invalid_token_counter = counter!("invalid_token_counter");
        let missing_token_counter = counter!("missing_token_counter");
        let successful_login_counter = counter!("successful_login_counter");

        // initialize AppMetrics
        Self {
            invalid_token_counter,
            missing_token_counter,
            successful_login_counter,
        }
    }
}
