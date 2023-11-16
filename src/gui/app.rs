use crate::env::store::DataStore;
use crate::gui::flow::{Message, SElement};
use crate::gui::scenes::get_scene;
use crate::if_ultimate_version;

use iced::{Application, Command, Theme};
use iced::executor::Default;

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
        get_scene(&self.store)
    }

    fn theme(&self) -> Self::Theme {
        self.store.config.theme.clone()
    }
}
