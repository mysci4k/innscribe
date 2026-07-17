use gpui::{Context, Entity, IntoElement, ParentElement, Render, Styled, Window};
use gpui_component::{
    IconName, Sizable, TitleBar,
    button::{Button, ButtonVariants},
    h_flex,
};

use crate::components::AppSidebar;

pub struct AppTitleBar {
    sidebar: Entity<AppSidebar>,
}

impl AppTitleBar {
    pub fn new(sidebar: Entity<AppSidebar>) -> Self {
        Self { sidebar }
    }
}

impl Render for AppTitleBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar = self.sidebar.clone();

        TitleBar::new().child(
            h_flex().items_center().child(
                Button::new("toggle-sidebar")
                    .small()
                    .ghost()
                    .icon(IconName::PanelLeft)
                    .on_click(move |_, _, cx| sidebar.update(cx, |s, cx| s.toggle(cx))),
            ),
        )
    }
}
