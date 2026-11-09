use gpui_fps::fps_monitor;
use gpui_kit::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window, actions,
    component::button::{Button, ButtonVariants},
    div,
    prelude::FluentBuilder,
};

actions!(app, [ToggleFps]);

pub struct AppShell {
    show_fps: bool,
}

impl AppShell {
    pub fn new() -> Self {
        Self { show_fps: false }
    }
}

impl Render for AppShell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .gap_2()
            .on_action(cx.listener(|this, _: &ToggleFps, _, cx| {
                this.show_fps = !this.show_fps;
                cx.notify();
            }))
            .child("Hello, World!")
            .child(
                Button::new("hello")
                    .primary()
                    .label("Click me")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
            .when(self.show_fps, |this| this.child(fps_monitor(window, cx)))
    }
}
