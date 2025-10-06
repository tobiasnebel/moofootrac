use axum::{
    Router,
    routing::{delete, get, post},
};

use axum_prometheus::{PrometheusMetricLayer, metrics_exporter_prometheus::PrometheusHandle};
use sea_orm::DatabaseConnection;
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

use crate::metrics::AppMetrics;

use super::handlers::{
    hello_handler::hello_handler as get_hello_handler,
    login_handler::get_login_handler,
    moofoolog_handler::{delete_moofoolog, get_moofoologs, post_moofoolog},
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
        .route("/api/hello", get(get_hello_handler))
        .route("/api/login", get(get_login_handler))
        .route("/api/moofoolog", get(get_moofoologs))
        .route("/api/moofoolog", post(post_moofoolog))
        .route("/api/moofoolog/:id", delete(delete_moofoolog))
        .route(
            "/api/metrics",
            get(|| async move { metric_handle.render() }),
        )
        // >>> serve static frontend files; doesnt redirect /app to index.html
        .nest_service(
            "/app",
            get(|request| async {
                let service = ServeDir::new("./dist");
                service
                    .fallback(ServeFile::new("./dist/index.html")) // doesnt work either
                    .oneshot(request)
                    .await
            }),
        )
        // CHATGPT, doesnt work:
        // .nest_service(
        //     "/app",
        //     ServeDir::new("./dist").fallback(ServeFile::new("./dist/index.html")),
        // )
        // <<<
        .with_state(state.clone())
        .layer(PrometheusMetricLayer::new());

    routes
}
