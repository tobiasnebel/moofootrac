use axum::{
    Router,
    routing::{get, post},
};

use axum_prometheus::{PrometheusMetricLayer, metrics_exporter_prometheus::PrometheusHandle};
use sea_orm::DatabaseConnection;

use crate::metrics::AppMetrics;

use super::handlers::{
    hello_handler::hello_handler as get_hello_handler,
    moofoolog_handler::{get_moofoologs, post_moofoolog},
};

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub metrics: AppMetrics,
}

// Router definition
pub fn router(state: AppState, metric_handle: PrometheusHandle) -> Router {
    // routes which should be always publicly accessible
    let routes: Router = Router::new()
        .route("/hello", get(get_hello_handler))
        .route("/moofoolog", get(get_moofoologs))
        .route("/moofoolog", post(post_moofoolog))
        .with_state(state.clone())
        .layer(PrometheusMetricLayer::new());

    routes
}
