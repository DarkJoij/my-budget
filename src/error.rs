use serde_json::Error as SerdeError;

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
