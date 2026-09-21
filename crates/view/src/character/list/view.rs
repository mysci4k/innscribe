use gpui_kit::{Context, IntoElement, ParentElement, Render, Styled, Window, base::v_flex};

pub struct CharacterListView;

impl CharacterListView {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CharacterListView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for CharacterListView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().text_3xl().child("Character List View")
    }
}
