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

#[derive(Debug, Clone, Copy)]
pub enum CharacterSort {
    Alphabetical,
    Newest,
    Oldest,
    Updated,
}

impl Characters {
    pub async fn list(&self, search: Option<&str>, sort: CharacterSort) -> Result<Vec<Character>> {
        let filter = Character::fields()
            .deleted_at()
            .is_none()
            .and(Character::fields().archived_at().is_none());
        let filter = match search.map(str::trim).filter(|q| !q.is_empty()) {
            Some(q) => filter.and(Character::fields().name().like(format!("%{}%", q))),
            None => filter,
        };

        let result = match sort {
            CharacterSort::Alphabetical => {
                Character::filter(filter)
                    .order_by(Character::fields().name().asc())
                    .exec(&mut self.handle())
                    .await?
            }
            CharacterSort::Newest => {
                Character::filter(filter)
                    .order_by(Character::fields().created_at().desc())
                    .exec(&mut self.handle())
                    .await?
            }
            CharacterSort::Oldest => {
                Character::filter(filter)
                    .order_by(Character::fields().created_at().asc())
                    .exec(&mut self.handle())
                    .await?
            }
            CharacterSort::Updated => {
                Character::filter(filter)
                    .order_by(Character::fields().updated_at().desc())
                    .exec(&mut self.handle())
                    .await?
            }
        };

        Ok(result)
    }
}
