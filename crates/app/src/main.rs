mod app;
mod logging;

use anyhow::Result;
use character::store::Database;
use gpui::{AppContext, WindowOptions, px, size};
use gpui_component::{Root, TitleBar};
use gpui_tokio::Tokio;
use shared::assets::AppAssets;

use crate::app::MainApp;

fn main() {
    logging::init();

    let db_path = persistence::connection::default_path();

    let app = gpui_platform::application().with_assets(AppAssets);

    app.run(move |cx| {
        gpui_component::init(cx);

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
                    window_min_size: Some(size(px(1024.), px(768.))),
                    ..Default::default()
                };

                cx.open_window(window_options, |window, cx| {
                    let view = cx.new(|cx| MainApp::new(window, cx));

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
