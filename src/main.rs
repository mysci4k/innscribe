mod app;
mod assets;
mod components;
mod database;
mod logging;
mod state;
mod views;

use anyhow::Result;
use gpui::{AppContext, WindowOptions};
use gpui_component::{Root, TitleBar};
use gpui_tokio::Tokio;

use crate::{app::MainApp, assets::AppAssets};

fn main() {
    logging::init();

    let db_path = database::path();

    let app = gpui_platform::application().with_assets(AppAssets);

    app.run(move |cx| {
        gpui_component::init(cx);

        gpui_tokio::init(cx);

        cx.spawn(async move |cx| {
            let outcome: Result<()> = async {
                let db = Tokio::spawn_result(cx, database::init(db_path)).await?;

                let window_options = WindowOptions {
                    titlebar: Some(TitleBar::title_bar_options()),
                    focus: true,
                    show: true,
                    app_id: Some("innscribe".into()),
                    ..Default::default()
                };

                cx.open_window(window_options, |window, cx| {
                    let view = cx.new(|cx| MainApp::new(db, cx));

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
