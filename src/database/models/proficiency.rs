use jiff::Timestamp;
use toasty::{Deferred, Model};
use uuid::Uuid;

use crate::database::models::{Character, ProficiencyType};

#[derive(Debug, Clone, Model)]
pub struct Proficiency {
    #[key]
    #[auto]
    pub id: Uuid,

    #[index]
    pub character_id: Uuid,
    #[belongs_to]
    pub character: Deferred<Character>,

    pub proficiency_name: String,
    pub proficiency_type: ProficiencyType,
    #[default(false)]
    pub expertise: bool,
    pub source: Option<String>,

    #[auto]
    pub created_at: Timestamp,
    #[auto]
    pub updated_at: Timestamp,
}
