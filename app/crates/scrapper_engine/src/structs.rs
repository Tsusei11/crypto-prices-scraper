use std::collections::HashMap;
use crate::MapOLHC;

use std::sync::Arc;

use bigdecimal::BigDecimal;
use diesel::RunQueryDsl;
use futures_util::lock::Mutex;
use db::db::establish_connection;
use db::models::NewBar1min;
use db::schema::bars_1min;
use exchange::structs::Orderbook;

#[derive(Clone, Debug)]
pub struct OLHC {
    pub open: BigDecimal,
    pub close: BigDecimal,
    pub min: BigDecimal,
    pub max: BigDecimal,
}

impl OLHC {
    pub fn new(open: BigDecimal) -> Self {
        OLHC {
            open: open.clone(),
            close: open.clone(),
            min: open.clone(),
            max: open,
        }
    }

    pub async fn update_map(map: Arc<Mutex<MapOLHC>>, orderbook: Orderbook) {
        let price = orderbook.ask;

        if price < BigDecimal::from(0) {
            return;
        }

        if let Some(exchange) = map.lock().await.get_mut(&orderbook.exchange) {
            if let Some(market) = exchange.get_mut(&orderbook.symbol) {
                market.close = price.clone();

                if market.min > price {
                    market.min = price.clone();
                }

                if market.max < price {
                    market.max = price.clone();
                }

            } else {
                exchange.insert(
                    orderbook.symbol.clone(),
                    OLHC::new(price),
                );
            }
        } else {
            let mut markets = HashMap::<String, OLHC>::new();
            markets.insert(
                orderbook.symbol.clone(),
                OLHC::new(price),
            );
            map.lock().await.insert(orderbook.exchange.clone(), markets);
        }
    }

    pub fn save_map(map: MapOLHC){

        for (exchange, markets) in map.iter() {
            for (market, olhc) in markets.iter() {
                olhc.save_to_db(&exchange, &market)
            }
        }

    }

    fn save_to_db(&self, exchange: &String, market: &String) {
        let bar_1min = NewBar1min {
            exchange,
            market,
            open: self.open.clone(),
            close: self.close.clone(),
            min: self.min.clone(),
            max: self.max.clone(),
        };
        let mut connection = establish_connection()
            .expect("Error establishing connection.");

        diesel::insert_into(bars_1min::table)
            .values(&bar_1min)
            .execute(&mut connection)
            .expect("Error saving bars_1min");
    }
}