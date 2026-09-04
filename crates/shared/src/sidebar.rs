use gpui_kit::{
    Context, FontWeight, IntoElement, ParentElement, Render, Styled, WeakEntity, Window,
    base::v_flex,
    component::{
        ActiveTheme, Icon,
        sidebar::{
            Sidebar, SidebarCollapsible, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
        },
    },
    div,
    prelude::FluentBuilder,
};

use crate::{
    assets::AppIcon,
    state::{AppScreen, AppState},
};

pub struct AppSidebar {
    state: WeakEntity<AppState>,
    collapsed: bool,
}

impl AppSidebar {
    pub fn new(state: WeakEntity<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            state,
            collapsed: false,
        }
    }

    pub fn toggle(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.collapsed = !self.collapsed;

        cx.notify();
    }

    pub fn is_collapsed(&self) -> bool {
        self.collapsed
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
            .is_some_and(|s| *s.read(cx).screen() == target);

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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Sidebar::new("app-sidebar")
            .collapsible(SidebarCollapsible::Icon)
            .collapsed(self.collapsed)
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
                            .child(Icon::new(AppIcon::Dices)),
                    )
                    .when(!self.collapsed, |this| {
                        this.child(
                            v_flex()
                                .flex_1()
                                .overflow_hidden()
                                .text_lg()
                                .font_weight(FontWeight::SEMIBOLD)
                                .child("InnScribe"),
                        )
                    }),
            )
            .child(
                SidebarGroup::new("General").child(SidebarMenu::new().child(self.nav_item(
                    "Dashboard",
                    AppScreen::Dashboard,
                    AppIcon::Map,
                    cx,
                ))),
            )
    }
}
