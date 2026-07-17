use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window};
use gpui_component::{h_flex, v_flex};

use crate::{
    components::{AppSidebar, AppTitleBar},
    state::{AppScreen, AppState},
    views::{DashboardView, TestView},
};

pub struct MainApp {
    app_state: Entity<AppState>,
    sidebar: Entity<AppSidebar>,
    title_bar: Entity<AppTitleBar>,
}

impl MainApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let app_state = cx.new(|cx| AppState::new(cx));
        let sidebar = cx.new(|cx| AppSidebar::new(app_state.clone(), cx));
        let title_bar = cx.new(|_| AppTitleBar::new(sidebar.clone()));

        Self {
            app_state,
            sidebar,
            title_bar,
        }
    }

    fn render_view(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.app_state.clone();

        match self.app_state.read(cx).current_screen.clone() {
            AppScreen::Dashboard => cx
                .new(|cx| DashboardView::new(state.downgrade(), cx))
                .into_any_element(),
            AppScreen::Test => cx
                .new(|cx| TestView::new(state.downgrade(), cx))
                .into_any_element(),
        }
    }
}

impl Render for MainApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex().size_full().child(self.sidebar.clone()).child(
            v_flex()
                .size_full()
                .flex_1()
                .child(self.title_bar.clone())
                .child(self.render_view(cx)),
        )
    }
}
