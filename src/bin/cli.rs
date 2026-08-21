#[path = "../database/connection.rs"]
mod database;

use anyhow::Result;
use toasty_cli::{Config, ToastyCli};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    let db_path = database::default_path();
    let db = database::connect(db_path).await?;

    let cli = ToastyCli::with_config(db, config);
    cli.parse_and_run().await?;

    Ok(())
}
