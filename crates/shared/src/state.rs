use gpui_kit::{
    Context, Window,
    component::{Theme, ThemeMode},
};
use settings::model::{AppSettings, ThemePreference};
use tracing::warn;

#[derive(Debug, Clone, PartialEq)]
pub enum AppScreen {
    Dashboard,
}

pub struct AppState {
    current_screen: AppScreen,
    settings: AppSettings,
}

impl AppState {
    pub fn new(settings: AppSettings) -> Self {
        Self {
            current_screen: AppScreen::Dashboard,
            settings,
        }
    }

    pub fn screen(&self) -> &AppScreen {
        &self.current_screen
    }

    pub fn settings(&self) -> &AppSettings {
        &self.settings
    }

    pub fn navigate(&mut self, screen: AppScreen, cx: &mut Context<Self>) {
        if self.current_screen == screen {
            return;
        }

        self.current_screen = screen;

        cx.notify();
    }

    pub fn set_theme(&mut self, window: Option<&mut Window>, cx: &mut Context<Self>) {
        let next_mode = if self.settings.theme().is_dark() {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        };

        self.settings
            .theme_mut()
            .set_mode(ThemePreference::from_dark(next_mode.is_dark()));
        Theme::change(next_mode, window, cx);

        if let Err(err) = self.settings.save() {
            warn!("Failed to save settings: {}", err)
        }

        cx.notify();
    }
}
