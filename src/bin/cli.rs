use anyhow::Result;
use innscribe::database;
use toasty_cli::{Config, ToastyCli};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    let db = database::init(database::path()).await?;

    let cli = ToastyCli::with_config(db, config);
    cli.parse_and_run().await?;

    Ok(())
}
