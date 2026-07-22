use toasty::Db;

use crate::database::TableStore;

pub struct Characters {
    pub db: Db,
}

impl TableStore for Characters {
    fn handle(&self) -> Db {
        self.db.clone()
    }
}

impl Characters {}
