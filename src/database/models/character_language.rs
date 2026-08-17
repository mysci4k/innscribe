use jiff::Timestamp;
use toasty::{Deferred, Model};
use uuid::Uuid;

use crate::database::models::Character;

#[derive(Debug, Clone, Model)]
pub struct CharacterLanguage {
    #[key]
    #[auto]
    pub id: Uuid,

    #[index]
    pub character_id: Uuid,
    #[belongs_to]
    pub character: Deferred<Character>,

    pub language_name: String,
    pub source: Option<String>,

    #[auto]
    pub created_at: Timestamp,
    #[auto]
    pub updated_at: Timestamp,
}
