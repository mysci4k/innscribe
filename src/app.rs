use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div};
use gpui_component::{
    StyledExt,
    button::{Button, ButtonVariants},
};

pub struct MainApp;

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
