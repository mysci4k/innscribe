use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    base::StyledExt,
    component::button::{Button, ButtonVariants},
    div,
};

pub struct MainApp;

impl Render for MainApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                    .on_click(|_event, _window, _cx| println!("Clicked!")),
            )
    }
}
