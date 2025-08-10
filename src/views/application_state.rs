use crate::{
    commands::{messages::Message, tab_identifier::TabIdentifier},
    models::duplicate_finder::DuplicateFinder,
};
use iced::{Element, Theme};

impl DuplicateFinder {
    pub fn view(&self) -> Element<'_, Message> {
        self.tab_bar_view().into()
    }

    pub fn switch_tab(&mut self, tab_identifier: TabIdentifier) {
        match tab_identifier {
            TabIdentifier::Editor => self.application_state.tab = TabIdentifier::Editor,
            TabIdentifier::DuplicateFinder => {
                self.application_state.tab = TabIdentifier::DuplicateFinder
            }
        }
    }

    pub fn theme(&self) -> Theme {
        match self.application_state.is_light_theme {
            true => Theme::Light,
            false => Theme::Dark,
        }
    }

    pub fn switch_theme(&mut self) {
        match self.application_state.is_light_theme {
            true => self.application_state.is_light_theme = false,
            false => self.application_state.is_light_theme = true,
        }
    }
}
