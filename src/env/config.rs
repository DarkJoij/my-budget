use crate::error::Error;
use super::{Names, ToClear};

use iced::Theme;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

use std::fs::{File, read_to_string, write};
use std::io::Write;

const CONFIG_FILE_NAME: &str = "config.json";

fn try_to_create() -> Result<String, Error> {
    let mut file = File::create(CONFIG_FILE_NAME)?;
    let default_content = to_string_pretty(&DirtyConfig::default())?;

    file.write_all(default_content.as_bytes())?;
    Ok(default_content.to_owned())
}

pub fn read_config() -> Result<Config, Error> {
    let content = match read_to_string(CONFIG_FILE_NAME) {
        Ok(content) => content,
        _ => try_to_create()? // Big brain time.
    };

    let dirty_config: DirtyConfig = from_str(&content)?;
    let clear_config = Config::from(dirty_config);

    drop(content);
    Ok(clear_config)
}

pub fn write_config(clear_config: &Config) -> Result<(), Error> {
    let serialized = to_string_pretty(&DirtyConfig::from(clear_config))?;
    write(CONFIG_FILE_NAME, serialized)?;

    Ok(())
}

pub struct Config {
    pub theme: Theme
}

impl Default for Config {
    fn default() -> Self {
        Config::from(DirtyConfig::default())
    }
}

impl From<DirtyConfig> for Config {
    fn from(value: DirtyConfig) -> Self {
        Config {
            theme: value.theme.to_clear()
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct DirtyConfig {
    theme: String
}

impl Default for DirtyConfig {
    fn default() -> Self {
        DirtyConfig {
            theme: "Light".to_owned()
        }
    }
}

impl From<&Config> for DirtyConfig {
    fn from(value: &Config) -> Self {
        DirtyConfig {
            theme: value.theme.sys_name().to_owned()
        }
    }
}
