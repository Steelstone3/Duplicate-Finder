use iced::Settings;
use models::duplicate_finder::DuplicateFinder;

mod commands;
mod controllers;
mod models;
mod views;

pub fn main() -> iced::Result {
    iced::application(
        || DuplicateFinder::boot(),
        DuplicateFinder::update,
        DuplicateFinder::view,
    )
    .theme(DuplicateFinder::theme)
    .antialiasing(true)
    .settings(Settings {
        id: Some("Duplicate Finder".to_string()),
        ..Default::default()
    })
    .run()
}
