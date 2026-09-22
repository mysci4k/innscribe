use directories::ProjectDirs;
use std::path::PathBuf;

const PROJECT_QUALIFIER: &str = "app";
const PROJECT_ORGANIZATION: &str = "Mysci4kStudio";
const PROJECT_APPLICATION: &str = "InnScribe";

const SETTINGS_FILENAME: &str = "settings.toml";

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from(PROJECT_QUALIFIER, PROJECT_ORGANIZATION, PROJECT_APPLICATION)
        .expect("Failed to get project directories")
}

fn data_dir() -> PathBuf {
    project_dirs().data_local_dir().to_path_buf()
}

pub fn settings_path() -> PathBuf {
    data_dir().join(SETTINGS_FILENAME)
}
