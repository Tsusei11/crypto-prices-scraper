use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let markets: Vec<String>;

    match env::var("MARKETS") {
        Ok(val) => markets = val.split(";").map(|s| s.to_string()).collect(),
        Err(e) => panic!("Error {}", e)
    }

    println!("{:?}", markets);
}
