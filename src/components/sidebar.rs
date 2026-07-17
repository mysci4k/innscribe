use gpui::{Context, Entity, ParentElement, Render, Styled, div, prelude::FluentBuilder, px};
use gpui_component::{
    ActiveTheme, Icon, IconName, h_flex,
    sidebar::{
        Sidebar, SidebarCollapsible, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu,
        SidebarMenuItem,
    },
    v_flex,
};

use crate::state::{AppScreen, AppState};

pub struct AppSidebar {
    state: Entity<AppState>,
    collapsed: bool,
}

impl AppSidebar {
    pub fn new(state: Entity<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            state,
            collapsed: false,
        }
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        self.collapsed = !self.collapsed;

        cx.notify();
    }

    fn nav_item(
        &self,
        label: &'static str,
        target: AppScreen,
        icon: IconName,
        cx: &mut Context<Self>,
    ) -> SidebarMenuItem {
        let state = self.state.clone();
        let is_active = self.state.read(cx).current_screen == target;

        SidebarMenuItem::new(label)
            .icon(icon)
            .active(is_active)
            .on_click(move |_, _, cx| state.update(cx, |s, cx| s.navigate(target.clone(), cx)))
    }
}

impl Render for AppSidebar {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::prelude::IntoElement {
        Sidebar::new("innscribe-sidebar")
            .collapsible(SidebarCollapsible::Icon)
            .collapsed(self.collapsed)
            .w(px(240.))
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
                            .when(self.collapsed, |this| {
                                this.size_4()
                                    .bg(cx.theme().transparent)
                                    .text_color(cx.theme().foreground)
                            })
                            .child(Icon::new(IconName::GalleryVerticalEnd)),
                    )
                    .when(!self.collapsed, |this| {
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
                        .child(self.nav_item(
                            "Dashboard",
                            AppScreen::Dashboard,
                            IconName::LayoutDashboard,
                            cx,
                        ))
                        .child(self.nav_item(
                            "Test",
                            AppScreen::Test,
                            IconName::LayoutDashboard,
                            cx,
                        )),
                ),
            )
            .footer(
                SidebarFooter::new().child(
                    h_flex()
                        .gap_2()
                        .child(Icon::new(IconName::CircleUser))
                        .when(!self.collapsed, |this| this.child("John Snow")),
                ),
            )
    }
}
