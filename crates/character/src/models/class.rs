use jiff::Timestamp;
use toasty::{Deferred, Model};
use uuid::Uuid;

use crate::models::Character;

#[derive(Debug, Clone, Model)]
pub struct CharacterClass {
    #[key]
    #[auto]
    pub id: Uuid,

    #[index]
    pub character_id: Uuid,
    #[belongs_to]
    pub character: Deferred<Character>,

    pub class_name: String,
    pub subclass_name: Option<String>,
    pub class_level: i32,
    pub hit_die: i32,
    pub hit_dice_remaining: i32,
    #[default(false)]
    pub is_primary_class: bool,
    pub spellcasting_ability: Option<String>,
    pub weapon_masteries_known: Option<i32>,

    #[auto]
    pub created_at: Timestamp,
    #[auto]
    pub updated_at: Timestamp,
}
