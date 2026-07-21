use gpui::{Context, WeakEntity};

use crate::database::Database;

#[derive(Debug, Clone, PartialEq)]
pub enum AppScreen {
    Dashboard,
    Test,
}

pub struct AppState {
    pub current_screen: AppScreen,
    pub database: WeakEntity<Database>,
}

impl AppState {
    pub fn new(database: WeakEntity<Database>, _cx: &mut Context<Self>) -> Self {
        Self {
            current_screen: AppScreen::Dashboard,
            database,
        }
    }

    pub fn navigate(&mut self, screen: AppScreen, cx: &mut Context<Self>) {
        self.current_screen = screen;

        cx.notify();
    }
}
