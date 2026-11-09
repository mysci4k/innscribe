mod app;

use gpui_kit::{
    AppContext, WindowOptions, application, assets::Assets, component::init, open_window, px, size,
};

use crate::app::HelloWorld;

fn main() {
    application().with_assets(Assets).run(|cx| {
        init(cx);

        open_window(
            WindowOptions {
                focus: true,
                show: true,
                app_id: Some("innscribe".into()),
                window_min_size: Some(size(px(1280.0), px(720.0))),
                ..Default::default()
            },
            cx,
            |_, cx| cx.new(|_| HelloWorld),
        )
        .expect("Failed to open window");
    });
}
