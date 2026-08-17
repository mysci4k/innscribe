use jiff::Timestamp;
use toasty::Model;
use uuid::Uuid;

use crate::database::models::{
    AbilityScores, CombatProficiencies, Currency, SavingThrowProficiencies, SkillProficiencies,
};

#[derive(Debug, Clone, Model)]
pub struct Character {
    #[key]
    #[auto]
    pub id: Uuid,

    #[index]
    pub name: String,
    pub species: String,
    pub species_lineage: Option<String>,
    pub background: String,
    pub alignment: Option<String>,
    #[default(1)]
    pub total_level: i32,
    #[default(0)]
    pub experience_points: i64,
    #[default(2)]
    pub proficiency_bonus: i32,
    #[default(false)]
    pub heroic_inspiration: bool,

    pub ability_scores: AbilityScores,
    pub saving_throws: SavingThrowProficiencies,
    pub skills: SkillProficiencies,
    pub combat_proficiencies: CombatProficiencies,

    pub armor_class: i32,
    #[default(0)]
    pub initiative_bonus: i32,
    #[default(30)]
    pub speed_walk_ft: i32,
    pub speed_fly_ft: Option<i32>,
    pub speed_swim_ft: Option<i32>,
    pub speed_climb_ft: Option<i32>,
    pub passive_perception: i32,
    pub hit_point_maximum: i32,
    pub hit_points_current: i32,
    #[default(0)]
    pub hit_points_temporary: i32,
    #[default(0)]
    pub exhaustion_level: i32,
    #[default(0)]
    pub death_save_successes: i32,
    #[default(0)]
    pub death_save_failures: i32,

    pub currency: Currency,

    pub size: Option<String>,
    pub age: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub eyes: Option<String>,
    pub skin: Option<String>,
    pub hair: Option<String>,
    pub gender: Option<String>,
    pub faith: Option<String>,
    pub personality_traits: Option<String>,
    pub ideals: Option<String>,
    pub bonds: Option<String>,
    pub flaws: Option<String>,
    pub backstory: Option<String>,
    pub appearance_notes: Option<String>,

    pub campaign_name: Option<String>,
    pub dungeon_master_name: Option<String>,
    pub avatar_path: Option<String>,

    #[auto]
    pub created_at: Timestamp,
    #[auto]
    pub updated_at: Timestamp,
    #[index]
    pub archived_at: Option<Timestamp>,
    #[index]
    pub deleted_at: Option<Timestamp>,
}
