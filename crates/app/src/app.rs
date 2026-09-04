use character::dashboard::DashboardView;
use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
    base::{h_flex, v_flex},
    component::{Root, scroll::ScrollableElement},
    div,
};
use shared::{
    sidebar::AppSidebar,
    state::{AppScreen, AppState},
    title_bar::AppTitleBar,
};

pub struct MainApp {
    app_state: Entity<AppState>,
    sidebar: Entity<AppSidebar>,
    title_bar: Entity<AppTitleBar>,
    dashboard_view: Entity<DashboardView>,
}

impl MainApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let app_state = cx.new(AppState::new);
        let sidebar = cx.new(|cx| AppSidebar::new(app_state.downgrade(), cx));
        let title_bar = cx.new(|_| AppTitleBar::new(sidebar.downgrade()));
        let dashboard_view =
            cx.new(|cx| DashboardView::new(app_state.downgrade(), sidebar.downgrade(), window, cx));

        Self {
            app_state,
            sidebar,
            title_bar,
            dashboard_view,
        }
    }

    fn render_view(&self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.app_state.read(cx).screen().clone() {
            AppScreen::Dashboard => self.dashboard_view.clone().into_any_element(),
        }
    }
}

impl Render for MainApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_layer = Root::render_dialog_layer(window, cx);

        h_flex()
            .size_full()
            .child(self.sidebar.clone())
            .child(
                v_flex()
                    .size_full()
                    .flex_1()
                    .child(self.title_bar.clone())
                    .child(
                        div()
                            .size_full()
                            .px_4()
                            .py_4()
                            .overflow_y_scrollbar()
                            .child(self.render_view(cx)),
                    ),
            )
            .children(dialog_layer)
    }
}
