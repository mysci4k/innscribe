use anyhow::{Result, anyhow};
use directories::ProjectDirs;
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

const PROJECT_QUALIFIER: &str = "app";
const PROJECT_ORGANIZATION: &str = "innscribestudio";
const PROJECT_APPLICATION: &str = "innscribe";

const DATABASE_SUBDIR: &str = "database";
const DATABASE_FILENAME: &str = "innscribe.db";

pub fn default_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from(PROJECT_QUALIFIER, PROJECT_ORGANIZATION, PROJECT_APPLICATION)
        .expect("Failed to get project directories");

    proj_dirs
        .data_local_dir()
        .join(DATABASE_SUBDIR)
        .join(DATABASE_FILENAME)
}

static MIGRATIONS: MigrationSet = embed_migrations!("../../toasty");

async fn apply_migrations(db: &Db) -> Result<()> {
    let report = MIGRATIONS.apply(db).await?;

    info!("Applied {} migrations", report.applied());
    info!("Skipped {} migrations", report.skipped());

    Ok(())
}
