pub use utils::*;

pub mod utils {
    use std::env;
    use anyhow::Result;

    pub fn load_markets(exchange_name: &str) -> Result<Vec<String>> {
        let key = format!("MARKETS_{}", exchange_name.to_uppercase());
        Ok(
            env::var(key)?.split(";").map(|s| {s.to_string()}).collect::<Vec<String>>()
        )
    }
}