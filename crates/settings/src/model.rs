use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, io::ErrorKind, path::Path};
use tracing::{info, warn};

use crate::paths;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppSettings {
    #[serde(default)]
    theme: ThemeSettings,
}

impl AppSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_or_init() -> Self {
        let path = paths::settings_path();
        match Self::load_from(&path) {
            Ok(settings) => {
                info!("Loaded application settings");
                settings
            }
            Err(err) => {
                warn!("Falling back to default settings: {}", err);
                Self::default()
            }
        }
    }

    fn load_from(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        match fs::read_to_string(path) {
            Ok(content) => toml::from_str(&content)
                .with_context(|| format!("Failed to parse settings at \"{}\"", path.display())),
            Err(err) if err.kind() == ErrorKind::NotFound => {
                let settings = Self::default();
                settings.save()?;

                info!("Created default settings at \"{}\"", path.display());
                Ok(settings)
            }
            Err(err) => Err(err)
                .with_context(|| format!("Failed to read settings at \"{}\"", path.display())),
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = paths::settings_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "Failed to create settings directory at \"{}\"",
                    parent.display()
                )
            })?
        }

        let body = toml::to_string(self).context("Failed to serialize settings")?;
        fs::write(&path, body)
            .with_context(|| format!("Failed to write settings at \"{}\"", path.display()))?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThemePreference {
    #[default]
    Light,
    Dark,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ThemeSettings {
    #[serde(default)]
    mode: ThemePreference,
}
