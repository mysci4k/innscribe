use gpui::{Context, IntoElement, ParentElement, Render, Styled, WeakEntity, div};
use gpui_component::v_flex;

use crate::state::AppState;

pub struct DashboardView {
    app_state: WeakEntity<AppState>,
}

impl DashboardView {
    pub fn new(app_state: WeakEntity<AppState>, _cx: &mut Context<Self>) -> Self {
        Self { app_state }
    }
}

impl Render for DashboardView {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(div().text_3xl().child("Dashboard"))
    }
}
