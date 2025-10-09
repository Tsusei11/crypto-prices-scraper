use std::env;
use diesel::{Connection, PgConnection};
use anyhow::Result;

pub fn establish_connection() -> Result<PgConnection> {
    let dsn = env::var("DATABASE_URL")?;

    Ok(PgConnection::establish(&dsn)?)
}