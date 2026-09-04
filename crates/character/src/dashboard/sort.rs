use gpui_kit::{SharedString, component::select::SelectItem};

use crate::repositories::CharacterSort;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOption {
    Alphabetical,
    Newest,
    Oldest,
    Updated,
}

impl SortOption {
    pub const ALL: [SortOption; 4] = [
        SortOption::Alphabetical,
        SortOption::Newest,
        SortOption::Oldest,
        SortOption::Updated,
    ];
}

impl From<SortOption> for CharacterSort {
    fn from(value: SortOption) -> Self {
        match value {
            SortOption::Alphabetical => CharacterSort::Alphabetical,
            SortOption::Newest => CharacterSort::Newest,
            SortOption::Oldest => CharacterSort::Oldest,
            SortOption::Updated => CharacterSort::Updated,
        }
    }
}

impl SelectItem for SortOption {
    type Value = Self;

    fn title(&self) -> SharedString {
        match self {
            SortOption::Alphabetical => "Alphabetical",
            SortOption::Newest => "Newest",
            SortOption::Oldest => "Oldest",
            SortOption::Updated => "Updated",
        }
        .into()
    }

    fn value(&self) -> &Self::Value {
        self
    }
}
