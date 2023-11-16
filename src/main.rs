mod env;
mod gui;
mod error;
mod macros;
mod utils;

use crate::gui::app::MyBudget;

use casual_logger::{Level, Log, Opt};
use iced::{Application, Error as IcedError, Settings as IcedSettings};
use iced::window::Settings as WindowSettings;

fn main() -> Result<(), IcedError> {
    Log::remove_old_logs();
    Log::set_file_name("journal");
    Log::set_retention_days(14);

    if_ultimate_version! {
        Log::set_level(Level::Info);
        Log::set_opt(Opt::Release);
    }

    info!("Starting application...");

    MyBudget::run(IcedSettings {
        window: WindowSettings {
            size: (800, 600),
            ..WindowSettings::default()
        },
        ..IcedSettings::default()
    })
}
