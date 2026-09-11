use anyhow::{Result, anyhow};
use settings::paths;
use std::{
    fs,
    path::{Path, PathBuf},
};
use toasty::{Db, ModelSet, embed_migrations, migration::MigrationSet};
use toasty_driver_turso::Turso;
use tracing::info;

pub async fn connect(db_path: impl AsRef<Path>, models: ModelSet) -> Result<Db> {
    let db_path = db_path.as_ref();
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| anyhow!("Failed to create database directory: {}", err))?
    }

    let driver = Turso::file(db_path);
    let db = Db::builder().models(models).build(driver).await?;

    apply_migrations(&db).await?;

    Ok(db)
}

pub fn default_path() -> PathBuf {
    paths::database_path()
}

static MIGRATIONS: MigrationSet = embed_migrations!("../../toasty");

async fn apply_migrations(db: &Db) -> Result<()> {
    let report = MIGRATIONS.apply(db).await?;

    info!("Applied {} migrations", report.applied());
    info!("Skipped {} migrations", report.skipped());

    Ok(())
}
