use super::config::{Config, read_config};

use casual_logger::Log;

pub struct DataStore {
    pub config: Config
}

impl Default for DataStore {
    fn default() -> Self {
        DataStore {
            config: if let Ok(config) = read_config() {
                config
            } else {
                let message = "Failed to load config file.";

                Log::fatal(message);
                panic!("{}", message);
            }
        }
    }
}
