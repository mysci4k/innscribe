use toasty::Embed;

#[derive(Debug, Clone, Embed)]
pub struct AbilityScores {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Debug, Clone, Embed)]
pub struct SavingThrowProficiencies {
    pub strength: bool,
    pub dexterity: bool,
    pub constitution: bool,
    pub intelligence: bool,
    pub wisdom: bool,
    pub charisma: bool,
}

#[derive(Debug, Clone, Embed)]
pub struct SkillProficiency {
    pub proficient: bool,
    pub expertise: bool,
}

#[derive(Debug, Clone, Embed)]
pub struct SkillProficiencies {
    pub acrobatics: SkillProficiency,
    pub animal_handling: SkillProficiency,
    pub arcana: SkillProficiency,
    pub athletics: SkillProficiency,
    pub deception: SkillProficiency,
    pub history: SkillProficiency,
    pub insight: SkillProficiency,
    pub intimidation: SkillProficiency,
    pub investigation: SkillProficiency,
    pub medicine: SkillProficiency,
    pub nature: SkillProficiency,
    pub perception: SkillProficiency,
    pub performance: SkillProficiency,
    pub persuasion: SkillProficiency,
    pub religion: SkillProficiency,
    pub sleight_of_hand: SkillProficiency,
    pub stealth: SkillProficiency,
    pub survival: SkillProficiency,
}

#[derive(Debug, Clone, Embed)]
pub struct Currency {
    pub copper: i64,
    pub silver: i64,
    pub electrum: i64,
    pub gold: i64,
    pub platinum: i64,
}
