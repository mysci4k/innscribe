mod app;
mod logging;

use anyhow::Result;
use character::store::Database;
use gpui_kit::{
    AppContext, WindowOptions,
    component::{Root, Theme, ThemeMode, TitleBar},
    px, size,
};
use gpui_tokio::Tokio;
use shared::assets::AppAssets;

use crate::app::MainApp;

fn main() {
    logging::init();

    let db_path = persistence::connection::default_path();

    let app = gpui_kit::application().with_assets(AppAssets);

    app.run(move |cx| {
        gpui_kit::init(cx);

        let app_settings = settings::model::AppSettings::load_or_init();
        let persisted_mode = if app_settings.theme().is_dark() {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        Theme::change(persisted_mode, None, cx);

        gpui_tokio::init(cx);

        cx.spawn(async move |cx| {
            let outcome: Result<()> = async {
                let db = Tokio::spawn_result(
                    cx,
                    persistence::connection::connect(db_path, character::model_set()),
                )
                .await?;

                cx.update(|cx| cx.set_global(Database::from_db(db)));

                let window_options = WindowOptions {
                    titlebar: Some(TitleBar::title_bar_options()),
                    focus: true,
                    show: true,
                    app_id: Some("innscribe".into()),
                    window_min_size: Some(size(px(1280.), px(720.))),
                    ..Default::default()
                };

                cx.open_window(window_options, |window, cx| {
                    let view = cx.new(|cx| MainApp::new(window, cx, app_settings));

                    cx.new(|cx| Root::new(view, window, cx))
                })
                .expect("Failed to open window");

                Ok(())
            }
            .await;

            if let Err(err) = outcome {
                eprintln!("Application startup failed: {err:#}");
            }
        })
        .detach();
    });
}
