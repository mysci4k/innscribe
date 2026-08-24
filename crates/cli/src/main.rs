use anyhow::Result;
use toasty_cli::{Config, ToastyCli};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    let db_path = persistence::connection::default_path();
    let db = persistence::connection::connect(&db_path, character::model_set()).await?;

    let cli = ToastyCli::with_config(db, config);
    cli.parse_and_run().await?;

    Ok(())
}
