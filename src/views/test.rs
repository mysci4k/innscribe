use gpui::{Context, IntoElement, ParentElement, Render, Styled, WeakEntity, div};
use gpui_component::v_flex;

use crate::state::AppState;

pub struct TestView {
    app_state: WeakEntity<AppState>,
}

impl TestView {
    pub fn new(app_state: WeakEntity<AppState>, _cx: &mut Context<Self>) -> Self {
        Self { app_state }
    }
}

impl Render for TestView {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .p_8()
            .child(div().text_3xl().child("Test"))
    }
}
