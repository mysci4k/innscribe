use anyhow::Result;
use jiff::Timestamp;
use toasty::Db;
use uuid::Uuid;

use crate::models::Character;

pub struct Characters {
    db: Db,
}

#[derive(Debug, Clone, Copy)]
pub enum CharacterSort {
    Alphabetical,
    Newest,
    Oldest,
    Updated,
}

impl Characters {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    fn db(&self) -> Db {
        self.db.clone()
    }

    pub async fn list(
        &self,
        search: Option<&str>,
        sort: CharacterSort,
        archived_only: bool,
    ) -> Result<Vec<Character>> {
        let mut filter = Character::fields().deleted_at().is_none();
        if archived_only {
            filter = filter.and(Character::fields().archived_at().is_some())
        } else {
            filter = filter.and(Character::fields().archived_at().is_none())
        }

        let filter = match search.map(str::trim).filter(|q| !q.is_empty()) {
            Some(q) => filter.and(Character::fields().name().like(format!("%{}%", q))),
            None => filter,
        };

        let result = match sort {
            CharacterSort::Alphabetical => {
                Character::filter(filter)
                    .order_by(Character::fields().name().asc())
                    .exec(&mut self.db())
                    .await?
            }
            CharacterSort::Newest => {
                Character::filter(filter)
                    .order_by(Character::fields().created_at().desc())
                    .exec(&mut self.db())
                    .await?
            }
            CharacterSort::Oldest => {
                Character::filter(filter)
                    .order_by(Character::fields().created_at().asc())
                    .exec(&mut self.db())
                    .await?
            }
            CharacterSort::Updated => {
                Character::filter(filter)
                    .order_by(Character::fields().updated_at().desc())
                    .exec(&mut self.db())
                    .await?
            }
        };

        Ok(result)
    }

    pub async fn archive(&self, id: Uuid) -> Result<()> {
        Character::update_by_id(id)
            .archived_at(Some(Timestamp::now()))
            .exec(&mut self.db())
            .await?;

        Ok(())
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        Character::update_by_id(id)
            .deleted_at(Some(Timestamp::now()))
            .exec(&mut self.db())
            .await?;

        Ok(())
    }
}
