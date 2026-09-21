use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, base::h_flex,
};
use shared::components::AppSidebar;

pub struct MainApp {
    sidebar: Entity<AppSidebar>,
}

impl MainApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let sidebar = cx.new(|_| AppSidebar::new());

        Self { sidebar }
    }
}

impl Render for MainApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .items_stretch()
            .child(self.sidebar.clone())
    }
}
