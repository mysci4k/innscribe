use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, WeakEntity, Window,
    base::h_flex,
    component::{
        ActiveTheme, Sizable, TitleBar,
        button::{Button, ButtonVariants},
    },
};

use crate::{assets::AppIcon, sidebar::AppSidebar, state::AppState};

pub struct AppTitleBar {
    app_state: WeakEntity<AppState>,
    sidebar: WeakEntity<AppSidebar>,
}

impl AppTitleBar {
    pub fn new(app_state: WeakEntity<AppState>, sidebar: WeakEntity<AppSidebar>) -> Self {
        Self { app_state, sidebar }
    }
}

impl Render for AppTitleBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.app_state.clone();
        let sidebar = self.sidebar.clone();
        let is_dark = cx.theme().is_dark();

        TitleBar::new().bg(cx.theme().sidebar).child(
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
                        .tooltip("Toggle sidebar")
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
                        .tooltip("Change theme")
                        .on_click(move |_, window, cx| {
                            if let Some(state) = state.upgrade() {
                                state.update(cx, |s, cx| s.set_theme(Some(window), cx))
                            }
                        }),
                ),
        )
    }
}
