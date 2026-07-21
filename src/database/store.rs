use gpui::Context;
use toasty::Db;

pub struct Database {
    db: Db,
}

impl Database {
    pub fn from_db(db: Db, _cx: &mut Context<Self>) -> Self {
        Self { db }
    }

    pub fn handle(&self) -> Db {
        self.db.clone()
    }
}
