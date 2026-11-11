use gpui_kit::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, StatefulInteractiveElement,
    Styled, Window,
    assets::IconName,
    base::{Placement, h_flex, v_flex},
    component::{
        ActiveTheme, Icon, Sizable,
        button::{Button, ButtonVariants},
        sidebar::{
            Sidebar, SidebarCollapsible, SidebarGroup, SidebarItem, SidebarMenu, SidebarMenuItem,
        },
    },
    div,
    prelude::FluentBuilder,
    px, relative,
};

pub struct AppSidebar {
    collapsed: bool,
    logo_hovered: bool,
}

impl AppSidebar {
    pub fn new() -> Self {
        Self {
            collapsed: false,
            logo_hovered: false,
        }
    }

    fn collapse(&mut self, cx: &mut Context<Self>) {
        if self.collapsed {
            return;
        }

        self.collapsed = true;
        self.logo_hovered = false;
        cx.notify();
    }

    fn expand(&mut self, cx: &mut Context<Self>) {
        if !self.collapsed {
            return;
        }

        self.collapsed = false;
        self.logo_hovered = false;
        cx.notify();
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
                h_flex()
                    .when(self.collapsed, |this| {
                        this.w_full().justify_center().child(
                            div()
                                .id("app-sidebar-logo")
                                .flex_shrink_0()
                                .on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                                    if this.logo_hovered != *hovered {
                                        this.logo_hovered = *hovered;
                                        cx.notify();
                                    }
                                }))
                                .child(
                                    Button::new("app-sidebar-toggle")
                                        .ghost()
                                        .small()
                                        .w_8()
                                        .h(px(30.0))
                                        .p_0()
                                        .tooltip("Expand sidebar")
                                        .tooltip_placement(Placement::Right)
                                        .accessibility_label("Expand sidebar")
                                        .child(if self.logo_hovered {
                                            Icon::new(IconName::PanelLeftOpen).with_size(px(16.0))
                                        } else {
                                            Icon::default()
                                                .path("icons/innscribe-logo.svg")
                                                .with_size(px(24.0))
                                        })
                                        .on_click(cx.listener(|this, _, _, cx| this.expand(cx))),
                                ),
                        )
                    })
                    .when(!self.collapsed, |this| {
                        this.w_full()
                            .gap_2()
                            .p_2()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .flex_shrink_0()
                                    .size_8()
                                    .rounded(cx.theme().radius)
                                    .bg(cx.theme().sidebar_primary)
                                    .text_color(cx.theme().sidebar_primary_foreground)
                                    .child(
                                        Icon::default().path("icons/innscribe-logo.svg").large(),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .text_sm()
                                    .flex_1()
                                    .min_w_0()
                                    .line_height(relative(1.25))
                                    .overflow_hidden()
                                    .text_ellipsis()
                                    .child("InnScribe")
                                    .child(div().child("Character Manager").text_xs()),
                            )
                            .child(
                                Button::new("app-sidebar-toggle")
                                    .ghost()
                                    .small()
                                    .flex_shrink_0()
                                    .tooltip("Collapse sidebar")
                                    .accessibility_label("Collapse sidebar")
                                    .icon(IconName::PanelLeftClose)
                                    .on_click(cx.listener(|this, _, _, cx| this.collapse(cx))),
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
