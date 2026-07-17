use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div};
use gpui_component::{
    StyledExt,
    button::{Button, ButtonVariants},
};

use crate::{components::AppSidebar, state::AppState};

pub struct MainApp {
    app_state: Entity<AppState>,
    sidebar: Entity<AppSidebar>,
}

impl MainApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let app_state = cx.new(|cx| AppState::new(cx));
        let sidebar = cx.new(|cx| AppSidebar::new(app_state.clone(), cx));

        Self { app_state, sidebar }
    }
}

impl Render for MainApp {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, World!")
            .child(
                Button::new("ok")
                    .primary()
                    .label("Let's Go!")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
    }
}
