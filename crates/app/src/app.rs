use character::dashboard::DashboardView;
use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
    base::{h_flex, v_flex},
    component::{Root, scroll::ScrollableElement},
    div,
};
use settings::model::AppSettings;
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
    pub fn new(window: &mut Window, cx: &mut Context<Self>, settings: AppSettings) -> Self {
        let app_state = cx.new(|_| AppState::new(settings));
        let sidebar = cx.new(|cx| AppSidebar::new(app_state.downgrade(), cx));
        let title_bar = cx.new(|_| AppTitleBar::new(app_state.downgrade(), sidebar.downgrade()));
        let dashboard_view =
            cx.new(|cx| DashboardView::new(app_state.downgrade(), sidebar.clone(), window, cx));

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
            .items_stretch()
            .child(self.sidebar.clone())
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .child(self.title_bar.clone())
                    .child(
                        div()
                            .size_full()
                            .min_h_0()
                            .overflow_y_scrollbar()
                            .child(div().w_full().p_4().child(self.render_view(cx))),
                    ),
            )
            .children(dialog_layer)
    }
}
