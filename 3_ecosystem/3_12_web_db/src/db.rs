use std::env::var;

use anyhow::Result;
use sqlx::{SqliteConnection, Connection};

pub async fn connect_to_db() -> Result<SqliteConnection> {
    let conn_string = var("DATABASE_URL")?;

    Ok(SqliteConnection::connect(&conn_string).await?)
}