use axum::{
    Router,
    routing::{get, post, put, delete},
};
use sqlx::PgPool;
use tower_http::services::ServeDir;

use crate::handlers;

pub fn app_config_routes() -> Router<PgPool> {
    Router::new()
        .route("/api/app-configs", get(handlers::list_app_configs))
        .route("/api/app-configs", post(handlers::create_app_config))
        .route("/api/app-configs/:id", get(handlers::get_app_config))
        .route("/api/app-configs/:id", put(handlers::update_app_config))
        .route("/api/app-configs/:id", delete(handlers::delete_app_config))
        .route("/api/app-configs/bulk-update", post(handlers::bulk_update_app_configs))
        .route("/api/app-configs/export", get(handlers::export_app_configs))
}

pub fn auth_routes() -> Router<PgPool> {
    Router::new()
        .route("/auth/google", get(handlers::google_login))
        .route("/auth/callback", get(handlers::google_callback))
}

pub fn static_routes() -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("static"))
}
