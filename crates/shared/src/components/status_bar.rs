use gpui_kit::{
    App, BorrowAppContext, IntoElement, RenderOnce, Window,
    assets::IconName,
    component::{
        ActiveTheme, Sizable, Theme, ThemeMode,
        button::{Button, ButtonVariants},
        status_bar::StatusBar,
    },
};
use tracing::warn;

use crate::globals::SettingsGlobal;

#[derive(IntoElement)]
pub struct AppStatusBar;

impl AppStatusBar {
    pub fn new() -> Self {
        Self
    }

    fn theme_toggle(cx: &App) -> Button {
        let icon = if cx.theme().is_dark() {
            IconName::Moon
        } else {
            IconName::Sun
        };

        Button::new("theme-toggle")
            .small()
            .ghost()
            .icon(icon)
            .tooltip("Change theme")
            .on_click(|_, window, cx| {
                let mode = cx.update_global::<SettingsGlobal, ThemeMode>(|settings, _| {
                    settings.theme_mut().change_mode();
                    if let Err(err) = settings.save() {
                        warn!("Failed to save settings: {}", err);
                    }

                    if settings.theme().is_dark() {
                        ThemeMode::Dark
                    } else {
                        ThemeMode::Light
                    }
                });

                Theme::change(mode, Some(window), cx);
            })
    }
}

impl Default for AppStatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for AppStatusBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        StatusBar::new().right(Self::theme_toggle(cx))
    }
}
