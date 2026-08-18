use gpui::{Context, IntoElement, ParentElement, Styled, div};
use gpui_component::{ActiveTheme, avatar::Avatar, h_flex, tag::Tag, v_flex};
use jiff::Timestamp;
use std::time::Duration;

use crate::{database::models::Character, views::dashboard::DashboardView};

pub fn character_card(character: Character, cx: &Context<DashboardView>) -> impl IntoElement {
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
                                .child(format!("{} · {}", character.species, character.background)),
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
