mod handlers;
mod structs;

use std::sync::Arc;
use axum::routing::get;
use axum::Router;
use db::db::DbPool;
use crate::handlers::{exchanges, last_min, markets};
use crate::structs::AppState;

pub fn get_app(pool: DbPool) -> Router {
    let state = Arc::new(AppState{pool});
    Router::new()
        .route("/exchanges", get(exchanges))
        .route("/markets", get(markets))
        .route("/last_min", get(last_min))
        .with_state(state)
}

