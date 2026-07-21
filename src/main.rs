mod app;
mod assets;
mod components;
mod logging;
mod state;
mod views;

use gpui::{AppContext, WindowOptions};
use gpui_component::{Root, TitleBar};

use crate::{app::MainApp, assets::AppAssets};

fn main() {
    logging::init();

    let app = gpui_platform::application().with_assets(AppAssets);

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            let window_options = WindowOptions {
                titlebar: Some(TitleBar::title_bar_options()),
                focus: true,
                show: true,
                app_id: Some("innscribe".into()),
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
