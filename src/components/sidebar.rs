use gpui::{
    Context, ParentElement, Render, Styled, WeakEntity, Window, div, prelude::FluentBuilder, px,
};
use gpui_component::{
    ActiveTheme, Icon,
    sidebar::{
        Sidebar, SidebarCollapsible, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
    },
    v_flex,
};

use crate::{
    assets::AppIcon,
    state::{AppScreen, AppState},
};

const MOBILE_BREAKPOINT: f32 = 768.;

pub struct AppSidebar {
    state: WeakEntity<AppState>,
    collapsed: bool,
    mobile_open: bool,
}

impl AppSidebar {
    pub fn new(state: WeakEntity<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            state,
            collapsed: false,
            mobile_open: false,
        }
    }

    pub fn toggle(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let is_mobile = window.viewport_size().width < px(MOBILE_BREAKPOINT);
        if is_mobile {
            self.mobile_open = !self.mobile_open;
            self.collapsed = false;
        } else {
            self.collapsed = !self.collapsed;
            self.mobile_open = false;
        }

        cx.notify();
    }

    fn nav_item(
        &self,
        label: &'static str,
        target: AppScreen,
        icon: AppIcon,
        cx: &mut Context<Self>,
    ) -> SidebarMenuItem {
        let state = self.state.clone();
        let is_active = self
            .state
            .upgrade()
            .map(|s| s.read(cx).current_screen == target)
            .unwrap_or(false);

        SidebarMenuItem::new(label)
            .icon(icon)
            .active(is_active)
            .on_click(move |_, _, cx| {
                if let Some(state) = state.upgrade() {
                    state.update(cx, |s, cx| s.navigate(target.clone(), cx))
                }
            })
    }
}

impl Render for AppSidebar {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::prelude::IntoElement {
        let viewport = window.viewport_size();
        let is_mobile = viewport.width < px(MOBILE_BREAKPOINT);
        let collapsed = if is_mobile {
            !self.mobile_open
        } else {
            self.collapsed
        };

        Sidebar::new("app-sidebar")
            .collapsible(if is_mobile {
                SidebarCollapsible::Offcanvas
            } else {
                SidebarCollapsible::Icon
            })
            .collapsed(collapsed)
            .w(if is_mobile { px(180.) } else { px(240.) })
            .header(
                SidebarHeader::new()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .size_8()
                            .flex_shrink_0()
                            .rounded(cx.theme().radius)
                            .bg(cx.theme().sidebar_primary)
                            .text_color(cx.theme().sidebar_primary_foreground)
                            .when(collapsed, |this| {
                                this.size_4()
                                    .bg(cx.theme().transparent)
                                    .text_color(cx.theme().foreground)
                            })
                            .child(Icon::new(AppIcon::Dices)),
                    )
                    .when(!collapsed, |this| {
                        this.child(
                            v_flex()
                                .flex_1()
                                .overflow_hidden()
                                .child("Acme Inc")
                                .child(div().text_xs().child("Enterprise")),
                        )
                    }),
            )
            .child(
                SidebarGroup::new("Navigation").child(
                    SidebarMenu::new()
                        .child(self.nav_item("Dashboard", AppScreen::Dashboard, AppIcon::Map, cx))
                        .child(self.nav_item("Test", AppScreen::Test, AppIcon::Map, cx)),
                ),
            )
    }
}
