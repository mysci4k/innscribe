use gpui_kit::Global;
use toasty::Db;

use crate::repositories::Characters;

pub struct Database {
    db: Db,
}

impl Global for Database {}

impl Database {
    pub fn from_db(db: Db) -> Self {
        Self { db }
    }

    pub fn handle(&self) -> Db {
        self.db.clone()
    }

    pub fn characters(&self) -> Characters {
        Characters::new(self.handle())
    }
}
