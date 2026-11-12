use std::io;
use tracing_subscriber::EnvFilter;

const DEFAULT_FILTER: &str =
    "warn,innscribe=debug,views=debug,shared=debug,gpui=info,gpui_base=warn,gpui_component=warn";

pub fn init() {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_FILTER));

    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(filter)
        .with_writer(io::stderr)
        .init();
}
