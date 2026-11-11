use anyhow::Result;
use gpui_kit::{
    AssetSource, SharedString,
    assets::{Assets, icon_assets},
};
use std::borrow::Cow;

icon_assets!(ExtraIcons, [Dices, Notebook, ScrollText, Swords, Users]);

pub struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(bytes) = ExtraIcons.load(path)? {
            return Ok(Some(bytes));
        }

        Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut paths = Assets.list(path)?;
        paths.extend(ExtraIcons.list(path)?);
        paths.sort();
        paths.dedup();

        Ok(paths)
    }
}
