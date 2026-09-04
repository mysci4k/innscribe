use gpui_kit::{
    App, ClickEvent, FontWeight, IntoElement, ParentElement, RenderOnce, Styled, Window,
    base::{h_flex, v_flex},
    component::{
        ActiveTheme, Icon, Sizable, WindowExt,
        avatar::Avatar,
        button::{Button, ButtonVariant, ButtonVariants},
        dialog::DialogButtonProps,
        menu::{DropdownMenu, PopupMenuItem},
        tag::Tag,
    },
    div,
};
use jiff::Timestamp;
use shared::assets::AppIcon;
use std::{rc::Rc, time::Duration};
use uuid::Uuid;

use crate::models::Character;

pub enum CharacterCardAction {
    Archive,
    Restore,
    Delete,
}

pub type CharacterCardActionHandler = Rc<dyn Fn(Uuid, CharacterCardAction, &mut App)>;

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

    fn time_ago(&self) -> String {
        let secs =
            (Timestamp::now().as_second() - self.character.updated_at.as_second()).max(0) as u64;
        timeago::Formatter::new().convert(Duration::from_secs(secs))
    }

    fn on_archive(
        character_id: Uuid,
        on_action: CharacterCardActionHandler,
    ) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
        move |_, _, cx| on_action(character_id, CharacterCardAction::Archive, cx)
    }

    fn on_restore(
        character_id: Uuid,
        on_action: CharacterCardActionHandler,
    ) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
        move |_, _, cx| on_action(character_id, CharacterCardAction::Restore, cx)
    }

    fn on_delete(
        character_id: Uuid,
        on_action: CharacterCardActionHandler,
    ) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
        move |_, window, cx| {
            let on_action = on_action.clone();

            window.open_alert_dialog(cx, move |alert, _, _| {
                let on_action = on_action.clone();

                alert
                    .title("Delete character?")
                    .description(
                        "Character will be permanently removed.\nThis action cannot be undone.",
                    )
                    .button_props(
                        DialogButtonProps::default()
                            .ok_text("Delete")
                            .ok_variant(ButtonVariant::Danger)
                            .show_cancel(true),
                    )
                    .on_ok(move |_, _, cx| {
                        on_action(character_id, CharacterCardAction::Delete, cx);
                        true
                    })
            });
        }
    }
}

impl RenderOnce for CharacterCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let campaign = self
            .character
            .campaign_name
            .as_deref()
            .unwrap_or("unassigned");
        let time_ago = self.time_ago();

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
                                            .font_weight(FontWeight::SEMIBOLD)
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
                                let mut menu = if self.character.archived_at.is_some() {
                                    menu.item(
                                        PopupMenuItem::new("Restore")
                                            .icon(Icon::new(AppIcon::ArchiveRestore))
                                            .on_click(Self::on_restore(
                                                self.character.id,
                                                self.on_action.clone(),
                                            )),
                                    )
                                } else {
                                    menu.item(
                                        PopupMenuItem::new("Archive")
                                            .icon(Icon::new(AppIcon::Archive))
                                            .on_click(Self::on_archive(
                                                self.character.id,
                                                self.on_action.clone(),
                                            )),
                                    )
                                };

                                menu = menu.item(
                                    PopupMenuItem::new("Delete")
                                        .icon(
                                            Icon::new(AppIcon::Trash).text_color(cx.theme().danger),
                                        )
                                        .on_click(Self::on_delete(
                                            self.character.id,
                                            self.on_action.clone(),
                                        )),
                                );

                                menu
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
