use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
    base::{h_flex, v_flex},
    component::scroll::ScrollableElement,
    div,
};
use shared::{
    components::{AppSidebar, AppStatusBar},
    state::{AppState, AppView},
};
use view::character::list::CharacterListView;

pub struct MainApp {
    app_state: Entity<AppState>,
    sidebar: Entity<AppSidebar>,
    character_list_view: Entity<CharacterListView>,
}

impl MainApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let app_state = cx.new(|_| AppState::new());
        let sidebar = cx.new(|_| AppSidebar::new());
        let character_list_view = cx.new(|_| CharacterListView::new());

        Self {
            app_state,
            sidebar,
            character_list_view,
        }
    }

    fn render_view(&self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.app_state.read(cx).current_view() {
            AppView::CharacterList => self.character_list_view.clone().into_any_element(),
        }
    }
}

impl Render for MainApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(
                h_flex()
                    .flex_1()
                    .h_full()
                    .min_h_0()
                    .items_stretch()
                    .child(self.sidebar.clone())
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .overflow_y_scrollbar()
                            .child(div().w_full().p_4().child(self.render_view(cx))),
                    ),
            )
            .child(AppStatusBar::new())
    }
}
