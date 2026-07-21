use anyhow::{Result, anyhow};
use directories::ProjectDirs;
use std::{
    fs,
    path::{Path, PathBuf},
};
use toasty::{Db, models};
use toasty_driver_turso::Turso;

async fn init_turso(db_path: &Path) -> Result<Db> {
    let driver = Turso::file(db_path);
    let db = Db::builder()
        .models(models!(crate::*))
        .build(driver)
        .await?;

    // db.push_schema().await?;

    Ok(db)
}

pub async fn init(db_path: PathBuf) -> Result<Db> {
    if let Some(parent) = db_path.parent() {
        if let Err(err) = fs::create_dir_all(parent) {
            return Err(anyhow!("Failed to create database directory: {err}"));
        }
    }

    let db = init_turso(&db_path).await?;

    Ok(db)
}

const PROJECT_QUALIFIER: &str = "app";
const PROJECT_ORGANIZATION: &str = "innscribestudio";
const PROJECT_APPLICATION: &str = "innscribe";

const DATABASE_SUBDIR: &str = "database";
const DATABASE_FILENAME: &str = "innscribe.db";

pub fn path() -> PathBuf {
    let proj_dirs = ProjectDirs::from(PROJECT_QUALIFIER, PROJECT_ORGANIZATION, PROJECT_APPLICATION)
        .expect("Failed to get project directories");

    proj_dirs
        .data_local_dir()
        .join(DATABASE_SUBDIR)
        .join(DATABASE_FILENAME)
}
