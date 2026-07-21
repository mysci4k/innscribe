use gpui::Context;
use std::sync::Arc;
use toasty::Db;
use tokio::sync::Mutex;

pub type DbHandle = Arc<Mutex<Db>>;

pub struct Database {
    db: DbHandle,
}

impl Database {
    pub fn from_db(db: Db, _cx: &mut Context<Self>) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
        }
    }

    pub fn handle(&self) -> DbHandle {
        Arc::clone(&self.db)
    }
}
