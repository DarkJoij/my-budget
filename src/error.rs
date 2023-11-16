use serde_json::Error as SerdeError;

use std::fmt::{Display, Formatter, Result};
use std::io::Error as IOError;

pub enum Error {
    ConfigReading(IOError),
    ConfigParsing(SerdeError)
}

impl From<IOError> for Error {
    fn from(value: IOError) -> Self {
        Self::ConfigReading(value)
    }
}

impl From<SerdeError> for Error {
    fn from(value: SerdeError) -> Self {
        Self::ConfigParsing(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::ConfigReading(io_error) => Display::fmt(io_error, f),
            Self::ConfigParsing(s_error) => Display::fmt(s_error, f)
        }
    }
}
