use gpui::{Context, IntoElement, ParentElement, Render, Styled, WeakEntity};
use gpui_component::{h_flex, v_flex};
use gpui_tokio::Tokio;

use crate::{
    database::{Database, models::Character},
    state::AppState,
    views::dashboard::character_card,
};

pub struct DashboardView {
    app_state: WeakEntity<AppState>,
    characters: Vec<Character>,
}

impl DashboardView {
    pub fn new(app_state: WeakEntity<AppState>, cx: &mut Context<Self>) -> Self {
        let view = Self {
            app_state,
            characters: Vec::new(),
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
        v_flex().size_full().child(
            h_flex().flex_wrap().gap_4().children(
                self.characters
                    .clone()
                    .into_iter()
                    .map(|character| character_card(character, cx)),
            ),
        )
    }
}
