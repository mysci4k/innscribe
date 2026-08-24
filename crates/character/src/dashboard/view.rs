use gpui::{
    AppContext, Context, Entity, FontWeight, IntoElement, ParentElement, Render, Styled, Task,
    WeakEntity, Window, div, prelude::FluentBuilder, px,
};
use gpui_component::{
    ActiveTheme, Icon, IndexPath,
    button::{Button, ButtonVariants},
    h_flex,
    input::{Input, InputEvent, InputState},
    select::{Select, SelectEvent, SelectState},
    v_flex,
};
use gpui_tokio::Tokio;
use shared::{assets::AppIcon, sidebar::AppSidebar, state::AppState};
use std::time::Duration;

use crate::{
    dashboard::components::character_card, models::Character, repositories::CharacterSort,
    store::Database,
};

pub struct DashboardView {
    app_state: WeakEntity<AppState>,
    sidebar: WeakEntity<AppSidebar>,
    characters: Vec<Character>,
    search: Entity<InputState>,
    sort: Entity<SelectState<Vec<&'static str>>>,
    reload_task: Option<Task<()>>,
}

impl DashboardView {
    pub fn new(
        app_state: WeakEntity<AppState>,
        sidebar: WeakEntity<AppSidebar>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let search = cx.new(|cx| InputState::new(window, cx).placeholder("Search"));
        let sort = cx.new(|cx| {
            SelectState::new(
                vec!["Alphabetical", "Newest", "Oldest", "Updated"],
                Some(IndexPath::default().row(3)),
                window,
                cx,
            )
        });

        cx.subscribe(&search, |this, _, event, cx| {
            if let InputEvent::Change = event {
                this.reload(cx, Some(Duration::from_millis(250)));
            }
        })
        .detach();

        cx.subscribe(&sort, |this, _, event, cx| match event {
            SelectEvent::Confirm(_) => {
                this.reload(cx, None);
            }
        })
        .detach();

        let view = Self {
            app_state,
            sidebar,
            characters: Vec::new(),
            search,
            sort,
            reload_task: None,
        };

        cx.spawn(async move |this, cx| {
            this.update(cx, |this, cx| {
                this.reload(cx, None);
            })
            .ok()
        })
        .detach();

        view
    }

    fn current_sort(&self, cx: &Context<Self>) -> CharacterSort {
        match self.sort.read(cx).selected_value().copied() {
            Some("Alphabetical") => CharacterSort::Alphabetical,
            Some("Newest") => CharacterSort::Newest,
            Some("Oldest") => CharacterSort::Oldest,
            _ => CharacterSort::Updated,
        }
    }

    fn reload(&mut self, cx: &mut Context<Self>, debounce: Option<Duration>) {
        let repository = cx.global::<Database>().characters();
        let search = self.search.read(cx).value().to_string();
        let sort = self.current_sort(cx);

        self.reload_task = Some(cx.spawn(async move |this, cx| {
            if let Some(delay) = debounce {
                cx.background_executor().timer(delay).await;
            }

            let result = Tokio::spawn_result(cx, async move {
                let search = search.trim();
                let search = if search.is_empty() {
                    None
                } else {
                    Some(search)
                };

                repository.list(search, sort).await
            })
            .await;

            this.update(cx, |this, cx| {
                if let Ok(characters) = result {
                    this.characters = characters
                }

                cx.notify();
            })
            .ok();
        }));
    }

    fn grid_columns(&self, window: &Window, cx: &Context<Self>) -> u16 {
        const SIDEBAR_COLLAPSED_WIDTH: f32 = 48.;
        const SIDEBAR_EXPANDED_WIDTH: f32 = 240.;
        const CARD_SLOT: f32 = 400.;
        const CONTENT_PAD: f32 = 32.;

        let collapsed = self
            .sidebar
            .upgrade()
            .is_some_and(|s| s.read(cx).is_collapsed());

        let sidebar = if collapsed {
            px(SIDEBAR_COLLAPSED_WIDTH)
        } else {
            px(SIDEBAR_EXPANDED_WIDTH)
        };
        let available = (window.viewport_size().width - sidebar - px(CONTENT_PAD)).max(px(0.));

        ((available / px(CARD_SLOT)).floor() as u16).clamp(1, 4)
    }
}

impl Render for DashboardView {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_3()
            .child(
                h_flex()
                    .flex_wrap()
                    .justify_between()
                    .child(
                        v_flex()
                            .gap_neg_2()
                            .child(
                                div()
                                    .text_3xl()
                                    .font_weight(FontWeight::BOLD)
                                    .child("Characters"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .ml_1()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(format!("{} characters", self.characters.len())),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_3()
                            .items_center()
                            .flex_wrap()
                            .child(
                                div().min_w_64().child(
                                    Input::new(&self.search)
                                        .prefix(Icon::new(AppIcon::Search))
                                        .cleanable(true),
                                ),
                            )
                            .child(
                                div()
                                    .min_w_32()
                                    .child(Select::new(&self.sort).placeholder("Sort by..")),
                            )
                            .child(
                                Button::new("new-character")
                                    .icon(Icon::new(AppIcon::Plus))
                                    .label("New character")
                                    .primary(),
                            ),
                    ),
            )
            .when(!self.characters.is_empty(), |this| {
                this.child(
                    h_flex()
                        .w_full()
                        .grid()
                        .grid_cols(self.grid_columns(window, cx))
                        .gap_4()
                        .children(
                            self.characters
                                .clone()
                                .into_iter()
                                .map(|character| character_card(character, cx)),
                        ),
                )
            })
    }
}
