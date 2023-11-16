use crate::fatal;
use super::config::{Config, read_config};

pub struct DataStore {
    pub config: Config
}

impl Default for DataStore {
    fn default() -> Self {
        DataStore {
            config: match read_config() {
                Ok(config) => config,
                Err(error) => fatal!("Failed to load config file: {}.", error)
            }
        }
    }
}
