use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName,
    base::v_flex,
    component::{
        ActiveTheme, Icon,
        sidebar::{
            Sidebar, SidebarCollapsible, SidebarGroup, SidebarHeader, SidebarItem, SidebarMenu,
            SidebarMenuItem,
        },
    },
    div,
    prelude::FluentBuilder,
    relative,
};

pub struct AppSidebar {
    collapsed: bool,
}

impl AppSidebar {
    pub fn new() -> Self {
        Self { collapsed: false }
    }

    fn toggle(&mut self) {
        self.collapsed = !self.collapsed
    }

    fn campaign_menu(&self) -> SidebarMenu {
        let menu = SidebarMenu::new()
            .child(SidebarMenuItem::new("Sheet").icon(IconName::ScrollText))
            .child(SidebarMenuItem::new("Combat").icon(IconName::Swords))
            .child(SidebarMenuItem::new("Notes").icon(IconName::Notebook));

        if self.collapsed { menu.pt_2() } else { menu }
    }
}

impl Render for AppSidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                            .flex_shrink_0()
                            .when(self.collapsed, |this| {
                                this.size_4().text_color(cx.theme().foreground)
                            })
                            .when(!self.collapsed, |this| {
                                this.size_8()
                                    .rounded(cx.theme().radius)
                                    .bg(cx.theme().sidebar_primary.alpha(0.9))
                                    .text_color(cx.theme().sidebar_primary_foreground)
                            })
                            .child(Icon::new(IconName::Dices)),
                    )
                    .when(!self.collapsed, |this| {
                        this.child(
                            v_flex()
                                .text_sm()
                                .flex_1()
                                .line_height(relative(1.25))
                                .overflow_hidden()
                                .text_ellipsis()
                                .child("InnScribe")
                                .child(div().child("Character Manager").text_xs()),
                        )
                    }),
            )
            .child(SidebarGroup::new("Library").child(
                SidebarMenu::new().child(SidebarMenuItem::new("Characters").icon(IconName::Users)),
            ))
            .child(SidebarGroup::new("Campaign").child(self.campaign_menu()))
            .footer(
                SidebarMenuItem::new("Settings")
                    .icon(IconName::Settings)
                    .collapsed(self.collapsed)
                    .render("app-sidebar-settings", window, cx),
            )
    }
}
