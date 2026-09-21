use gpui_kit::{
    Context, FontWeight, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName,
    base::{Placement, StyledExt, h_flex, v_flex},
    component::{
        ActiveTheme, Icon, Sizable,
        button::{Button, ButtonCustomVariant, ButtonVariants},
        sidebar::{Sidebar, SidebarGroup, SidebarMenu, SidebarMenuItem},
    },
    div,
    prelude::FluentBuilder,
    px,
};

pub struct AppSidebar {
    collapsed: bool,
}

impl AppSidebar {
    pub fn new() -> Self {
        Self { collapsed: false }
    }

    fn toggle(&mut self) {
        self.collapsed = !self.collapsed;
    }

    fn toggle_button(
        &self,
        id: &'static str,
        icon: IconName,
        tooltip_text: &'static str,
        tooltip_placement: Placement,
        square: bool,
        cx: &mut Context<Self>,
    ) -> Button {
        Button::new(id)
            .small()
            .w_full()
            .h(px(30.0))
            .when(square, |this| this.h_7().w_7())
            .custom(
                ButtonCustomVariant::new(cx)
                    .color(cx.theme().transparent)
                    .foreground(cx.theme().sidebar_foreground)
                    .hover(cx.theme().sidebar_accent.opacity(0.8))
                    .active(cx.theme().sidebar_accent.opacity(0.8)),
            )
            .icon(icon)
            .tooltip(tooltip_text)
            .tooltip_placement(tooltip_placement)
            .on_click(cx.listener(|this, _event, _window, _cx| this.toggle()))
    }
}

impl Default for AppSidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for AppSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Sidebar::new("app-sidebar")
            .collapsible(true)
            .collapsed(self.collapsed)
            .header(
                div()
                    .h_flex()
                    .gap_2()
                    .w_full()
                    .justify_between()
                    .when(!self.collapsed, |this| this.p_2())
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
                            .when(self.collapsed, |this| this.size(px(30.0)))
                            .child(Icon::new(IconName::ScrollText)),
                    )
                    .when(!self.collapsed, |this| {
                        this.child(
                            v_flex()
                                .flex_1()
                                .overflow_hidden()
                                .child(
                                    div()
                                        .text_lg()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("InnScribe"),
                                )
                                .child(div().mt_neg_2().text_xs().child("Character Manager")),
                        )
                    })
                    .when(!self.collapsed, |this| {
                        this.child(self.toggle_button(
                            "sidebar-collapse-button",
                            IconName::PanelLeftClose,
                            "Collapse sidebar",
                            Placement::Bottom,
                            true,
                            cx,
                        ))
                    }),
            )
            .child(
                SidebarGroup::new("General").child(
                    SidebarMenu::new()
                        .child(SidebarMenuItem::new("Characters").icon(IconName::UserGroup)),
                ),
            )
            .footer(
                h_flex()
                    .w_full()
                    .justify_between()
                    .when(self.collapsed, |this| {
                        this.child(self.toggle_button(
                            "sidebar-open-button",
                            IconName::PanelLeftOpen,
                            "Open sidebar",
                            Placement::Right,
                            false,
                            cx,
                        ))
                    }),
            )
    }
}
