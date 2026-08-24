use jiff::Timestamp;
use toasty::{Deferred, Model};
use uuid::Uuid;

use crate::models::{Character, FeatCategory};

#[derive(Debug, Clone, Model)]
pub struct CharacterFeat {
    #[key]
    #[auto]
    pub id: Uuid,

    #[index]
    pub character_id: Uuid,
    #[belongs_to]
    pub character: Deferred<Character>,

    pub feat_name: String,
    pub feat_category: FeatCategory,
    pub description: Option<String>,
    pub level_gained: i32,
    pub source: Option<String>,

    #[auto]
    pub created_at: Timestamp,
    #[auto]
    pub updated_at: Timestamp,
}
