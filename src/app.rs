use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div};
use gpui_component::{
    StyledExt,
    button::{Button, ButtonVariants},
};

use crate::state::AppState;

pub struct MainApp {
    app_state: Entity<AppState>,
}

impl MainApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let app_state = cx.new(|cx| AppState::new(cx));

        Self { app_state }
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
