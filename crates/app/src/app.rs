use gpui_fps::fps_monitor;
use gpui_kit::{
    AppContext, Context, Entity, FocusHandle, InteractiveElement, IntoElement, ParentElement,
    Render, Styled, Window, actions,
    base::{h_flex, v_flex},
    prelude::FluentBuilder,
};
use shared::components::AppSidebar;

actions!(app, [ToggleFps]);

pub struct AppShell {
    show_fps: bool,
    focus_handle: FocusHandle,
    sidebar: Entity<AppSidebar>,
}

impl AppShell {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let sidebar = cx.new(|_| AppSidebar::new());

        Self {
            show_fps: false,
            focus_handle: cx.focus_handle(),
            sidebar,
        }
    }
}

impl Render for AppShell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &ToggleFps, _, cx| {
                this.show_fps = !this.show_fps;
                cx.notify();
            }))
            .size_full()
            .child(h_flex().h_full().child(self.sidebar.clone()))
            .when(self.show_fps, |this| this.child(fps_monitor(window, cx)))
    }
}
