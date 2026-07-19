use gpui::{Context, IntoElement, ParentElement, Render, Styled, WeakEntity, Window};
use gpui_component::{
    ActiveTheme, Sizable, Theme, ThemeMode, TitleBar,
    button::{Button, ButtonVariants},
    h_flex,
};

use crate::{assets::AppIcon, components::AppSidebar};

pub struct AppTitleBar {
    sidebar: WeakEntity<AppSidebar>,
}

impl AppTitleBar {
    pub fn new(sidebar: WeakEntity<AppSidebar>) -> Self {
        Self { sidebar }
    }
}

impl Render for AppTitleBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar = self.sidebar.clone();
        let is_dark = cx.theme().is_dark();

        TitleBar::new()
            .bg(cx.theme().sidebar_primary_foreground)
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .size_full()
                    .pr_2()
                    .child(
                        Button::new("toggle-sidebar")
                            .small()
                            .ghost()
                            .icon(AppIcon::PanelLeft)
                            .on_click(move |_, window, cx| {
                                if let Some(sidebar) = sidebar.upgrade() {
                                    sidebar.update(cx, |s, cx| s.toggle(window, cx))
                                }
                            }),
                    )
                    .child(
                        Button::new("toggle-theme")
                            .small()
                            .ghost()
                            .icon(if is_dark { AppIcon::Sun } else { AppIcon::Moon })
                            .on_click(move |_, window, cx| {
                                let next_mode = if cx.theme().is_dark() {
                                    ThemeMode::Light
                                } else {
                                    ThemeMode::Dark
                                };

                                Theme::change(next_mode, Some(window), cx);
                            }),
                    ),
            )
    }
}
