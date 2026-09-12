use gpui_kit::Context;
use settings::model::AppSettings;

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

    pub fn navigate(&mut self, screen: AppScreen, cx: &mut Context<Self>) {
        if self.current_screen == screen {
            return;
        }

        self.current_screen = screen;

        cx.notify();
    }
}
