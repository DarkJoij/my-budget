use iced::{Element, Renderer, Theme};

pub type SElement<'a> = Element<'a, Message, Renderer<Theme>>;

#[derive(Clone, Debug)]
pub enum Message {
    Click
}
