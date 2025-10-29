use std::sync::Arc;
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::{Response, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use diesel::{BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use db::models::Bar1min;
use db::schema::bars_1min::dsl::bars_1min;
use db::schema::bars_1min::{exchange, id, market};
use crate::{AppState};
use crate::structs::{LastMinParams, LastMinResponse};

// Get the list of all available exchanges
pub async fn exchanges(State(state): State<Arc<AppState>>) -> Json<Vec<String>> {
    let pool = state.pool.clone();

    let result = tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().expect("Failed to get connection from pool");

        bars_1min
            .select(exchange)
            .distinct()
            .order(exchange.asc())
            .load::<String>(&mut conn)
            .expect("Error loading bars")
    }).await.expect("Error spawning exchanges endpoint task");

    Json(result)
}

// Get the list of all available markets
pub async fn markets(State(state): State<Arc<AppState>>) -> Json<Vec<String>> {
    let pool = state.pool.clone();

    let result = tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().expect("Failed to get connection from pool");

        bars_1min
            .select(market)
            .distinct()
            .order(market.asc())
            .load::<String>(&mut conn)
            .expect("Error loading bars")
    }).await.expect("Error spawning exchanges endpoint task");

    Json(result)
}

// Get the last available bar for a given market on a given exchange
pub async fn last_min(
    State(state): State<Arc<AppState>>,
    Query(params): Query<LastMinParams>
) -> Response<Body> {
    let pool = state.pool.clone();

    let result = tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().expect("Failed to get connection from pool");

        bars_1min
            .filter(
                exchange.ilike(params.exchange.unwrap_or_default())
                    .and(market.ilike(params.market.unwrap_or_default()))
            )
            .order(id.desc())
            .first::<Bar1min>(&mut conn)
    }).await.expect("Error spawning last_min endpoint task");

    match result {
        Ok(row) => {
            let bar = LastMinResponse {
                open: row.open,
                close: row.close,
                min: row.min,
                max: row.max,
            };
            Json(bar).into_response()
        },
        Err(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "Last min for a given market on a given exchange not found").into_response()
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}