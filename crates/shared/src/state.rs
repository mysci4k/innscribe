use gpui_kit::Context;

#[derive(Debug, PartialEq)]
pub enum AppView {
    CharacterList,
}

pub struct AppState {
    current_view: AppView,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_view: AppView::CharacterList,
        }
    }

    pub fn current_view(&self) -> &AppView {
        &self.current_view
    }

    pub fn navigate(&mut self, view: AppView, cx: &mut Context<Self>) {
        if self.current_view == view {
            return;
        }

        self.current_view = view;

        cx.notify();
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
