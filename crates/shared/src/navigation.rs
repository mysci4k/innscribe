#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppSection {
    #[default]
    Characters,
    Sheet,
    Combat,
    Notes,
    Settings,
}
