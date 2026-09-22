use anyhow::Result;
use gpui_kit::Global;
use settings::model::{AppSettings, ThemeSettings};

pub struct SettingsGlobal {
    settings: AppSettings,
}

impl Global for SettingsGlobal {}

impl SettingsGlobal {
    pub fn new(settings: AppSettings) -> Self {
        Self { settings }
    }

    pub fn theme(&self) -> &ThemeSettings {
        self.settings.theme()
    }

    pub fn theme_mut(&mut self) -> &mut ThemeSettings {
        self.settings.theme_mut()
    }

    pub fn save(&self) -> Result<()> {
        self.settings.save()
    }
}
