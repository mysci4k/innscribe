mod app;

use gpui_kit::{
    AppContext, WindowOptions, application, assets::Assets, component::init, open_window,
};

use crate::app::HelloWorld;

fn main() {
    application().with_assets(Assets).run(|cx| {
        init(cx);

        open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| HelloWorld))
            .expect("Failed to open window");
    });
}
