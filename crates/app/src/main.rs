mod app;
mod logging;

use gpui_kit::{AppContext, WindowOptions, component::Root};
use shared::assets::AppAssets;

use crate::app::MainApp;

fn main() {
    logging::init();

    let app = gpui_kit::application().with_assets(AppAssets);

    app.run(move |cx| {
        gpui_kit::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| MainApp);

                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
