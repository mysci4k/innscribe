use anyhow::Result;
use gpui::{AppContext, Context, Global, Task};
use gpui_tokio::Tokio;
use toasty::Db;

use crate::database::repositories::Characters;

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
        Characters { db: self.handle() }
    }
}

pub trait TableStore {
    fn handle(&self) -> Db;

    fn with_db<R, F, Fut>(&self, cx: &impl AppContext, f: F) -> Task<Result<R>>
    where
        R: Send + 'static,
        F: FnOnce(Db) -> Fut + Send + 'static,
        Fut: Future<Output = Result<R>> + Send + 'static,
    {
        let db = self.handle();

        Tokio::spawn_result(cx, async move { f(db).await })
    }
}
