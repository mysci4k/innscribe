use gpui::Context;

#[derive(Debug, Clone, PartialEq)]
pub enum AppScreen {
    Dashboard,
    Test,
}

pub struct AppState {
    pub current_screen: AppScreen,
}

impl AppState {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            current_screen: AppScreen::Dashboard,
        }
    }

    pub fn navigate(&mut self, screen: AppScreen, cx: &mut Context<Self>) {
        self.current_screen = screen;

        cx.notify();
    }
}
