use gpui::Context;

#[derive(Debug, Clone, PartialEq)]
pub enum AppScreen {
    Dashboard,
}

pub struct AppState {
    current_screen: AppScreen,
}

impl AppState {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            current_screen: AppScreen::Dashboard,
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
