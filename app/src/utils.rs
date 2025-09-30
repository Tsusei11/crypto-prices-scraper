pub use utils::*;

pub mod utils {
    use std::env;
    use anyhow::Result;

    pub fn load_markets() -> Result<Vec<String>> {
        Ok(
            env::var("MARKETS")?.split(";").map(|s| {s.to_string()}).collect::<Vec<String>>()
        )
    }
}