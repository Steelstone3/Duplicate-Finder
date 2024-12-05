use iced::Settings;
use models::duplicate_finder::DuplicateFinder;

mod commands;
mod controllers;
mod models;
mod views;

pub fn main() -> iced::Result {
    iced::application(
        "Duplicate Finder",
        DuplicateFinder::update,
        DuplicateFinder::view,
    )
    .theme(DuplicateFinder::theme)
    .antialiasing(true)
    .settings(Settings::default())
    .run()
}
