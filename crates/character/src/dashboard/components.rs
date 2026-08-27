use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div};
use gpui_component::{
    ActiveTheme, Icon, Sizable,
    avatar::Avatar,
    button::{Button, ButtonVariants},
    h_flex,
    menu::{DropdownMenu, PopupMenuItem},
    tag::Tag,
    v_flex,
};
use jiff::Timestamp;
use shared::assets::AppIcon;
use std::{rc::Rc, time::Duration};
use uuid::Uuid;

use crate::models::Character;

pub enum CharacterCardAction {
    Archive,
    Delete,
}

pub type CharacterCardActionHandler = Rc<dyn Fn(Uuid, CharacterCardAction, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct CharacterCard {
    character: Character,
    on_action: CharacterCardActionHandler,
}

impl CharacterCard {
    pub fn new(character: Character, on_action: CharacterCardActionHandler) -> Self {
        Self {
            character,
            on_action,
        }
    }
}

impl RenderOnce for CharacterCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let secs =
            (Timestamp::now().as_second() - self.character.updated_at.as_second()).max(0) as u64;
        let duration = Duration::from_secs(secs);
        let time_ago = timeago::Formatter::new().convert(duration);

        let campaign = self
            .character
            .campaign_name
            .as_deref()
            .unwrap_or("unassigned");

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
                    .items_start()
                    .justify_between()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_3()
                            .child(Avatar::new().name(self.character.name.clone()))
                            .child(
                                v_flex()
                                    .child(
                                        div()
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .child(self.character.name),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(cx.theme().muted_foreground)
                                            .child(format!(
                                                "{} · {}",
                                                self.character.species, self.character.background
                                            )),
                                    ),
                            ),
                    )
                    .child(
                        Button::new(format!("character-card-menu-{}", self.character.id))
                            .ghost()
                            .small()
                            .icon(Icon::new(AppIcon::EllipsisVertical))
                            .dropdown_menu(move |menu, _window, cx| {
                                menu.item(
                                    PopupMenuItem::new("Archive")
                                        .icon(Icon::new(AppIcon::Archive))
                                        .on_click({
                                            let on_action = self.on_action.clone();

                                            move |_, window, cx| {
                                                on_action(
                                                    self.character.id,
                                                    CharacterCardAction::Archive,
                                                    window,
                                                    cx,
                                                )
                                            }
                                        }),
                                )
                                .item(
                                    PopupMenuItem::new("Delete")
                                        .icon(
                                            Icon::new(AppIcon::Trash).text_color(cx.theme().danger),
                                        )
                                        .on_click({
                                            let on_action = self.on_action.clone();

                                            move |_, window, cx| {
                                                on_action(
                                                    self.character.id,
                                                    CharacterCardAction::Delete,
                                                    window,
                                                    cx,
                                                )
                                            }
                                        }),
                                )
                            }),
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
