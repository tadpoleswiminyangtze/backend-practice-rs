use crate::handler::auth::auth_handler;
use crate::handler::users::users_handler;
use crate::middleware::auth;
use crate::AppState;
use axum::{middleware, Extension, Router};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_router = Router::new()
        .nest("/auth", auth_handler())
        .nest("/users", users_handler().layer(middleware::from_fn(auth)))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));

    Router::new().nest("/api", api_router)
}
