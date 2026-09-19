use anyhow::Result;
use gpui_kit::{AssetSource, SharedString, assets::Assets};
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "icons/**/*.svg"]
pub struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        if let Some(file) = Self::get(path) {
            return Ok(Some(file.data));
        }

        Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut paths = Assets.list(path)?;
        paths.extend(Self::iter().filter_map(|p| p.starts_with(path).then(|| p.into())));
        paths.sort();
        paths.dedup();

        Ok(paths)
    }
}
