use gpui_kit::{
    AppContext, Context, Entity, FontWeight, IntoElement, ParentElement, Render, Styled, Task,
    WeakEntity, Window,
    base::{
        h_flex,
        input::{InputEvent, InputState},
        v_flex,
    },
    component::{
        ActiveTheme, Icon,
        button::{Button, Toggle, ToggleVariants},
        input::Input,
        select::{Select, SelectEvent, SelectState},
    },
    div,
    prelude::FluentBuilder,
    px,
};
use gpui_tokio::Tokio;
use shared::{
    assets::AppIcon,
    sidebar::{AppSidebar, AppSidebarEvent},
    state::AppState,
};
use std::{rc::Rc, time::Duration};
use uuid::Uuid;

use crate::{
    dashboard::{
        components::{CharacterCard, CharacterCardAction, CharacterCardActionHandler},
        sort::SortOption,
    },
    models::Character,
    repositories::CharacterSort,
    store::Database,
};

pub struct DashboardView {
    app_state: WeakEntity<AppState>,
    sidebar_collapsed: bool,
    characters: Vec<Character>,
    search: Entity<InputState>,
    sort: Entity<SelectState<Vec<SortOption>>>,
    show_archived: bool,
    reload_task: Option<Task<()>>,
}

impl DashboardView {
    const CARD_MIN_REMS: f32 = 24.0;
    const GRID_GAP_REMS: f32 = 1.0;
    const CONTENT_PAD_REMS: f32 = 2.0;
    const MAX_COLUMNS: u16 = 4;

    pub fn new(
        app_state: WeakEntity<AppState>,
        sidebar: Entity<AppSidebar>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let search = cx.new(|cx| InputState::new(window, cx).placeholder("Search characters"));
        let sort = cx.new(|cx| SelectState::new(SortOption::ALL.to_vec(), None, window, cx));

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

        let sidebar_collapsed = sidebar.read(cx).is_collapsed();
        cx.subscribe(&sidebar, |this, _, event: &AppSidebarEvent, cx| {
            let AppSidebarEvent::CollapsedChanged(collapsed) = event;
            this.sidebar_collapsed = *collapsed;

            cx.notify();
        })
        .detach();

        let view = Self {
            app_state,
            sidebar_collapsed,
            characters: Vec::new(),
            search,
            sort,
            show_archived: false,
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

    fn reload(&mut self, cx: &mut Context<Self>, debounce: Option<Duration>) {
        let repository = cx.global::<Database>().characters();
        let search = self.search.read(cx).value().to_string();
        let sort = self
            .sort
            .read(cx)
            .selected_value()
            .copied()
            .map_or(CharacterSort::Updated, CharacterSort::from);
        let archived_only = self.show_archived;

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

                repository.list(search, sort, archived_only).await
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

    fn handle_character_card_action(
        &mut self,
        id: Uuid,
        action: CharacterCardAction,
        cx: &mut Context<Self>,
    ) {
        let repository = cx.global::<Database>().characters();

        cx.spawn(async move |this, cx| {
            let result = Tokio::spawn_result(cx, async move {
                match action {
                    CharacterCardAction::Archive => repository.archive(id).await,
                    CharacterCardAction::Restore => repository.restore(id).await,
                    CharacterCardAction::Delete => repository.delete(id).await,
                }
            })
            .await;

            this.update(cx, |this, cx| {
                if result.is_ok() {
                    this.reload(cx, None);
                }
            })
            .ok();
        })
        .detach();
    }

    fn columns_for_width(
        available_rems: f32,
        card_min_rems: f32,
        gap_rems: f32,
        max_columns: u16,
    ) -> u16 {
        if available_rems <= 0.0 || card_min_rems <= 0.0 {
            return 1;
        }
        let slot = card_min_rems + gap_rems;
        (((available_rems + gap_rems) / slot).floor() as u16).clamp(1, max_columns)
    }

    fn grid_columns(&self, window: &Window) -> u16 {
        let rem = window.rem_size();
        let sidebar_rems = if self.sidebar_collapsed {
            AppSidebar::COLLAPSED_WIDTH_REMS
        } else {
            AppSidebar::EXPANDED_WIDTH_REMS
        };
        let available_px =
            (window.viewport_size().width - sidebar_rems * rem - Self::CONTENT_PAD_REMS * rem)
                .max(px(0.));
        let available_rems = available_px / rem;

        Self::columns_for_width(
            available_rems,
            Self::CARD_MIN_REMS,
            Self::GRID_GAP_REMS,
            Self::MAX_COLUMNS,
        )
    }

    fn character_count_label(&self) -> String {
        let count = self.characters.len();

        if self.show_archived {
            format!("{count} archived")
        } else {
            match count {
                0 => "0 characters".to_string(),
                1 => "1 character".to_string(),
                n => format!("{n} characters"),
            }
        }
    }
}

impl Render for DashboardView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity();
        let on_character_card_action: CharacterCardActionHandler =
            Rc::new(move |id, action, cx| {
                view.update(cx, |this, cx| {
                    this.handle_character_card_action(id, action, cx)
                });
            });

        let character_count_label = self.character_count_label();

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
                                    .child(character_count_label),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_3()
                            .items_center()
                            .flex_wrap()
                            .child(
                                Toggle::new("archived-character-toggle")
                                    .outline()
                                    .checked(self.show_archived)
                                    .icon(Icon::new(AppIcon::Archive))
                                    .tooltip("Show archived characters")
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.show_archived = *checked;
                                        view.reload(cx, None);
                                    })),
                            )
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
                                    .child(Select::new(&self.sort).placeholder("Sort")),
                            )
                            .child(
                                Button::new("new-character")
                                    .icon(Icon::new(AppIcon::Plus))
                                    .label("New character"),
                            ),
                    ),
            )
            .when(!self.characters.is_empty(), |this| {
                this.child(
                    h_flex()
                        .w_full()
                        .grid()
                        .grid_cols(self.grid_columns(window))
                        .gap_4()
                        .children(self.characters.clone().into_iter().map(|character| {
                            CharacterCard::new(character, on_character_card_action.clone())
                        })),
                )
            })
    }
}
