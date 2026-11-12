use gpui_fps::fps_monitor;
use gpui_kit::{
    AppContext, Context, Entity, FocusHandle, InteractiveElement, IntoElement, ParentElement,
    Render, Styled, Window, actions, base::v_flex, prelude::FluentBuilder,
};
use views::workspace::WorkspaceView;

actions!(app, [ToggleFps]);

pub struct AppShell {
    workspace: Entity<WorkspaceView>,
    show_fps: bool,
    focus_handle: FocusHandle,
}

impl AppShell {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let workspace = cx.new(|cx| WorkspaceView::new(window, cx));

        Self {
            workspace,
            show_fps: false,
            focus_handle: cx.focus_handle(),
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
            .child(v_flex().flex_1().min_h_0().child(self.workspace.clone()))
            .when(self.show_fps, |this| this.child(fps_monitor(window, cx)))
    }
}
