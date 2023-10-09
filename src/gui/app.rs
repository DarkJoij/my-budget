use crate::env::store::DataStore;
use crate::gui::flow::{Message, SElement};
use crate::if_ultimate_version;

use iced::{Application, Command, Theme};
use iced::executor::Default;
use iced::widget::Button;

pub struct MyBudget {
    store: DataStore
}

impl Application for MyBudget {
    type Executor = Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            MyBudget { store: DataStore::default() },
            Command::none()
        )
    }

    fn title(&self) -> String {
        if_ultimate_version! {{
            "Мой Бюджет Ult.".to_owned()
        } else {
            "Мой Бюджет".to_owned()
        }}
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Click => println!("Clicked!")
        }

        Command::none()
    }

    fn view(&self) -> SElement<'_> {
        Button::new("Click me!")
            .on_press(Message::Click)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        self.store.config.theme.clone()
    }
}
