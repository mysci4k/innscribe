use gpui::{
    AppContext, Context, Entity, FontWeight, IntoElement, ParentElement, Render, Styled,
    WeakEntity, Window, div, prelude::FluentBuilder,
};
use gpui_component::{
    ActiveTheme, Icon,
    button::{Button, ButtonVariants},
    h_flex,
    input::{Input, InputState},
    select::{Select, SelectState},
    v_flex,
};
use gpui_tokio::Tokio;

use crate::{
    assets::AppIcon,
    database::{Database, models::Character},
    state::AppState,
    views::dashboard::character_card,
};

pub struct DashboardView {
    app_state: WeakEntity<AppState>,
    characters: Vec<Character>,
    search: Entity<InputState>,
    sort: Entity<SelectState<Vec<&'static str>>>,
}

impl DashboardView {
    pub fn new(
        app_state: WeakEntity<AppState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let search = cx.new(|cx| InputState::new(window, cx).placeholder("Search"));
        let sort = cx.new(|cx| {
            SelectState::new(
                vec!["Name (A-Z)", "Name (Z-A)", "Newest", "Oldest"],
                None,
                window,
                cx,
            )
        });

        let view = Self {
            app_state,
            characters: Vec::new(),
            search,
            sort,
        };

        cx.spawn(async move |this, cx| {
            this.update(cx, |this, cx| {
                this.reload(cx);
            })
            .ok()
        })
        .detach();

        view
    }

    fn reload(&mut self, cx: &mut Context<Self>) {
        let repository = cx.global::<Database>().characters();

        cx.spawn(async move |this, cx| {
            let result = Tokio::spawn_result(cx, async move { repository.list().await }).await;

            this.update(cx, |this, cx| {
                match result {
                    Ok(characters) => this.characters = characters,
                    Err(_) => {}
                }

                cx.notify();
            })
        })
        .detach();
    }
}

impl Render for DashboardView {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                    h_flex().flex_wrap().gap_4().children(
                        self.characters
                            .clone()
                            .into_iter()
                            .map(|character| character_card(character, cx)),
                    ),
                )
            })
    }
}
