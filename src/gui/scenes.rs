use crate::env::store::DataStore;
use crate::gui::flow::{Message, SElement};

use iced::widget::Button;

pub fn get_scene<'a>(_store: &DataStore) -> SElement<'a> {
    Button::new("Click me!")
        .on_press(Message::Click)
        .into()
}
