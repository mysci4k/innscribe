use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window};
use gpui_component::h_flex;

use crate::{components::AppSidebar, state::AppState};

pub struct MainApp {
    app_state: Entity<AppState>,
    sidebar: Entity<AppSidebar>,
}

impl MainApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let app_state = cx.new(|cx| AppState::new(cx));
        let sidebar = cx.new(|cx| AppSidebar::new(app_state.clone(), cx));

        Self { app_state, sidebar }
    }
}

impl Render for MainApp {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        h_flex().size_full().child(self.sidebar.clone())
    }
}
