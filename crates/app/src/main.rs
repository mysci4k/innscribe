mod app;
mod logging;

use gpui_kit::{AppContext, WindowOptions, component::Root, px, size};
use shared::assets::AppAssets;

use crate::app::MainApp;

fn main() {
    logging::init();

    let app = gpui_kit::application().with_assets(AppAssets);

    app.run(move |cx| {
        gpui_kit::init(cx);

        cx.spawn(async move |cx| {
            let window_options = WindowOptions {
                focus: true,
                show: true,
                app_id: Some("innscribe".into()),
                window_min_size: Some(size(px(1280.0), px(720.0))),
                ..Default::default()
            };

            cx.open_window(window_options, |window, cx| {
                let view = cx.new(MainApp::new);

                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
