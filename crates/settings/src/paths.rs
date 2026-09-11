use directories::ProjectDirs;
use std::path::PathBuf;

const PROJECT_QUALIFIER: &str = "app";
const PROJECT_ORGANIZATION: &str = "innscribestudio";
const PROJECT_APPLICATION: &str = "innscribe";

const DATABASE_SUBDIR: &str = "database";
const DATABASE_FILENAME: &str = "innscribe.db";

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from(PROJECT_QUALIFIER, PROJECT_ORGANIZATION, PROJECT_APPLICATION)
        .expect("Failed to get project directories")
}

pub fn data_dir() -> PathBuf {
    project_dirs().data_local_dir().to_path_buf()
}

pub fn database_path() -> PathBuf {
    data_dir().join(DATABASE_SUBDIR).join(DATABASE_FILENAME)
}
