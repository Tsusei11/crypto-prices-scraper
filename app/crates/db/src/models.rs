use crate::schema::bars_1min;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Bar1min {
    pub id: i32,
    pub exchange: String,
    pub market: String,
    pub timestamp: NaiveDateTime,
    pub open: BigDecimal,
    pub close: BigDecimal,
    pub min: BigDecimal,
    pub max: BigDecimal,
}

#[derive(Insertable)]
#[diesel(table_name = bars_1min)]
pub struct NewBar1min<'a> {
    pub exchange: &'a str,
    pub market: &'a str,
    pub open: BigDecimal,
    pub close: BigDecimal,
    pub min: BigDecimal,
    pub max: BigDecimal,
}

impl<'a> NewBar1min<'a> {
    pub fn new(exchange: &'a str,
               market: &'a str,
               open: BigDecimal,
               close: BigDecimal,
               min: BigDecimal,
               max: BigDecimal) -> NewBar1min<'a> {

        NewBar1min {
            exchange,
            market,
            open,
            close,
            min,
            max,
        }
    }
}