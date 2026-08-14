use anyhow::Result;
use toasty::Db;

use crate::database::{TableStore, models::Character};

pub struct Characters {
    pub db: Db,
}

impl TableStore for Characters {
    fn handle(&self) -> Db {
        self.db.clone()
    }
}

impl Characters {
    pub async fn list(&self) -> Result<Vec<Character>> {
        let result = Character::all().exec(&mut self.handle()).await?;

        Ok(result)
    }
}
