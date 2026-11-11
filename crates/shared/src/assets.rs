use anyhow::Result;
use gpui_kit::{
    AssetSource, SharedString,
    assets::{Assets, icon_assets},
};
use rust_embed::RustEmbed;
use std::borrow::Cow;

icon_assets!(ExtraIcons, [Dices, Notebook, ScrollText, Swords, Users]);

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "icons/**/*.svg"]
struct AppFiles;

pub struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        if let Some(file) = AppFiles::get(path) {
            return Ok(Some(file.data));
        }

        if let Some(bytes) = ExtraIcons.load(path)? {
            return Ok(Some(bytes));
        }

        Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut paths = Assets.list(path)?;
        paths.extend(ExtraIcons.list(path)?);
        paths.extend(AppFiles::iter().filter_map(|p| p.starts_with(path).then(|| p.into())));
        paths.sort();
        paths.dedup();

        Ok(paths)
    }
}
