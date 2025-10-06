use axum_prometheus::metrics::{Counter, counter};

#[derive(Clone)]
pub struct AppMetrics {
    pub invalid_token_counter: Counter,
    pub missing_token_counter: Counter,
    pub valid_token_counter: Counter,
    pub login_fail_counter: Counter,
}

impl AppMetrics {
    pub fn init() -> Self {
        // initialize metrics
        let invalid_token_counter = counter!("invalid_token_counter");
        let missing_token_counter = counter!("missing_token_counter");
        let valid_token_counter = counter!("valid_token_counter");
        let login_fail_counter = counter!("login_fail_counter");

        // initialize AppMetrics
        Self {
            invalid_token_counter,
            missing_token_counter,
            valid_token_counter,
            login_fail_counter,
        }
    }
}
