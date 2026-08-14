use gpui::{Context, IntoElement, ParentElement, Render, Styled, WeakEntity, div};
use gpui_component::{ActiveTheme, avatar::Avatar, h_flex, tag::Tag, v_flex};
use gpui_tokio::Tokio;
use jiff::Timestamp;
use std::time::Duration;

use crate::{
    database::{Database, models::Character},
    state::AppState,
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

    fn character_card(&self, character: Character, cx: &Context<Self>) -> impl IntoElement {
        let secs = (Timestamp::now().as_second() - character.updated_at.as_second()).max(0) as u64;
        let duration = Duration::from_secs(secs);
        let time_ago = timeago::Formatter::new().convert(duration);

        let campaign = character.campaign_name.as_deref().unwrap_or("unassigned");

        div()
            .p_4()
            .flex()
            .flex_col()
            .gap_3()
            .min_w_96()
            .rounded(cx.theme().radius)
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .child(
                h_flex()
                    .items_center()
                    .gap_3()
                    .child(Avatar::new().name(character.name.clone()))
                    .child(
                        v_flex()
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child(character.name),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(format!(
                                        "{} · {}",
                                        character.species, character.background
                                    )),
                            ),
                    ),
            )
            .child(
                v_flex().gap_3().child(
                    h_flex()
                        .gap_3()
                        .child(Tag::secondary().outline().child("Level 8"))
                        .child(Tag::secondary().outline().child("Wizard · Evocation")),
                ),
            )
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .pt_2()
                    .border_t_1()
                    .border_color(cx.theme().border)
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("Campaign: {}", campaign)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("Updated: {}", time_ago)),
                    ),
            )
    }
}

impl Render for DashboardView {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().size_full().child(
            h_flex().flex_wrap().gap_4().children(
                self.characters
                    .clone()
                    .into_iter()
                    .map(|character| self.character_card(character, cx)),
            ),
        )
    }
}
