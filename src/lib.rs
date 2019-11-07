#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

pub mod reg;

use diesel::prelude::*;
use dotenv::dotenv;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::env;

pub fn establish_connection() -> Result<SqliteConnection, ExitFailure> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").with_context(|_| "DATABASE_URL must be set")?;
    let db_connection = SqliteConnection::establish(&database_url)
        .with_context(|_| format!("Error connecting to {}", database_url))?;
    Ok(db_connection)
}
