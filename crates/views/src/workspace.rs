use gpui_kit::{
    AnyElement, App, AppContext, Context, Entity, FocusHandle, IntoElement, ParentElement, Render,
    Styled, Subscription, Window,
    base::{h_flex, v_flex},
};
use shared::{
    components::{AppSidebar, SidebarEvent},
    navigation::AppSection,
};

use crate::{
    characters::CharactersView, combat::CombatView, notes::NotesView, settings::SettingsView,
    sheet::SheetView,
};

struct Pages {
    characters: Entity<CharactersView>,
    sheet: Entity<SheetView>,
    combat: Entity<CombatView>,
    notes: Entity<NotesView>,
    settings: Entity<SettingsView>,
}

impl Pages {
    fn new(_window: &mut Window, cx: &mut Context<WorkspaceView>) -> Self {
        Self {
            characters: cx.new(CharactersView::new),
            sheet: cx.new(SheetView::new),
            combat: cx.new(CombatView::new),
            notes: cx.new(NotesView::new),
            settings: cx.new(SettingsView::new),
        }
    }
}

pub struct WorkspaceView {
    pages: Pages,
    section: AppSection,
    sidebar: Entity<AppSidebar>,
    _sidebar_events: Subscription,
}

impl WorkspaceView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let pages = Pages::new(window, cx);
        let section = AppSection::default();
        let sidebar = cx.new(|_| AppSidebar::new(section));

        let sidebar_events =
            cx.subscribe_in(&sidebar, window, |this, _, event, window, cx| match event {
                SidebarEvent::Navigate(section) => {
                    this.navigate(*section, window, cx);
                }
            });

        Self {
            pages,
            section,
            sidebar,
            _sidebar_events: sidebar_events,
        }
    }

    pub fn section(&self) -> AppSection {
        self.section
    }

    pub fn navigate(&mut self, section: AppSection, window: &mut Window, cx: &mut Context<Self>) {
        if self.section == section {
            return;
        }

        self.section = section;
        self.sidebar
            .update(cx, |sidebar, cx| sidebar.set_active(section, cx));

        let focus = self.focus_handle_for(section, cx);
        window.focus(&focus, cx);
        cx.notify();
    }

    fn focus_handle_for(&self, section: AppSection, cx: &App) -> FocusHandle {
        match section {
            AppSection::Characters => self.pages.characters.read(cx).focus_handle(),
            AppSection::Sheet => self.pages.sheet.read(cx).focus_handle(),
            AppSection::Combat => self.pages.combat.read(cx).focus_handle(),
            AppSection::Notes => self.pages.notes.read(cx).focus_handle(),
            AppSection::Settings => self.pages.settings.read(cx).focus_handle(),
        }
    }

    fn render_section(&self) -> AnyElement {
        match self.section {
            AppSection::Characters => self.pages.characters.clone().into_any_element(),
            AppSection::Sheet => self.pages.sheet.clone().into_any_element(),
            AppSection::Combat => self.pages.combat.clone().into_any_element(),
            AppSection::Notes => self.pages.notes.clone().into_any_element(),
            AppSection::Settings => self.pages.settings.clone().into_any_element(),
        }
    }
}

impl Render for WorkspaceView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .items_stretch()
            .child(self.sidebar.clone())
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .child(self.render_section()),
            )
    }
}
