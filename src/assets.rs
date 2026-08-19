use anyhow::anyhow;
use gpui::{AssetSource, Result, SharedString};
use gpui_component::IconNamed;
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "icons/**/*.svg"]
pub struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("Could not find asset at path \"{path}\""))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

#[derive(Clone, Copy)]
pub enum AppIcon {
    Dices,
    Map,
    Moon,
    PanelLeft,
    Plus,
    Search,
    Sun,
}

impl IconNamed for AppIcon {
    fn path(self) -> SharedString {
        match self {
            AppIcon::Dices => "icons/dices.svg".into(),
            AppIcon::Map => "icons/map.svg".into(),
            AppIcon::Moon => "icons/moon.svg".into(),
            AppIcon::PanelLeft => "icons/panel-left.svg".into(),
            AppIcon::Plus => "icons/plus.svg".into(),
            AppIcon::Search => "icons/search.svg".into(),
            AppIcon::Sun => "icons/sun.svg".into(),
        }
    }
}
