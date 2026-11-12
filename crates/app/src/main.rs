mod app;
mod logging;

use gpui_kit::{
    AppContext, KeyBinding, WindowOptions, application,
    component::{self, Theme, ThemeMode},
    open_window, px, size,
};
use shared::assets::AppAssets;

use crate::app::{AppShell, ToggleFps};

fn main() {
    logging::init();

    application().with_assets(AppAssets).run(|cx| {
        component::init(cx);

        cx.bind_keys([KeyBinding::new("ctrl-shift-f", ToggleFps, None)]);

        Theme::change(ThemeMode::Dark, None, cx);

        open_window(
            WindowOptions {
                focus: true,
                show: true,
                app_id: Some("innscribe".into()),
                window_min_size: Some(size(px(1280.0), px(720.0))),
                ..Default::default()
            },
            cx,
            |window, cx| cx.new(|cx| AppShell::new(window, cx)),
        )
        .expect("Failed to open window");
    });
}
