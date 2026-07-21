use jiff::Timestamp;
use toasty::Model;
use uuid::Uuid;

#[derive(Debug, Clone, Model)]
pub struct Character {
    #[key]
    #[auto]
    pub id: Uuid,

    #[index]
    pub name: String,

    #[index]
    pub archived: bool,

    #[auto]
    pub created_at: Timestamp,
    #[auto]
    pub updated_at: Timestamp,
}
