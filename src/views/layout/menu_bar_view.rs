use crate::{commands::messages::Message, models::duplicate_finder::DuplicateFinder};
use iced::Length;
use iced::widget::{button, column};
use iced_aw::menu::Item;
use iced_aw::{Menu, menu_bar, menu_items};

impl DuplicateFinder {
    pub fn menu_view(&self) -> iced::widget::Column<'_, Message> {
        let menu_template = |items| Menu::new(items).max_width(180.0).offset(6.0);

        let menu_bar = menu_bar!((
            button("File").on_press(Message::MenuBar),
            menu_template(menu_items!(
                (button("New").width(Length::Fill).on_press(Message::FileNew))
            ))
        )(
            button("View").on_press(Message::MenuBar),
            menu_template(menu_items!((button("Line Numbers")
                .width(Length::Fill)
                .on_press(Message::ViewPrependLineNumbersPressed))(
                button("Toggle Theme")
                    .width(Length::Fill)
                    .on_press(Message::ViewThemeChanged)
            )))
        ));

        column!().push(menu_bar)
    }
}
