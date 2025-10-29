use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use db::db::DbPool;

pub struct AppState {
    pub pool: DbPool
}

#[derive(Deserialize)]
pub struct LastMinParams {
    pub exchange: Option<String>,
    pub market: Option<String>,
}

#[derive(Serialize)]
pub struct LastMinResponse {
    pub open: BigDecimal,
    pub close: BigDecimal,
    pub min: BigDecimal,
    pub max: BigDecimal,
}