mod profiles;
mod settings;

use std::fmt::Formatter;
use std::path::PathBuf;

pub use profiles::{NavigationFunctionName, NavigationFunctionSettings, Profile, Profiles};
pub use settings::Settings;

#[cfg(feature = "toml")]
mod toml;

#[cfg(feature = "toml")]
pub use toml::{read_profiles_toml, write_profiles_toml};

#[derive(Clone, Debug)]
pub enum ReadProfileError {
    Io { message: String, path: PathBuf },
    Deserialize { message: String },
}

impl std::error::Error for ReadProfileError {}

impl std::fmt::Display for ReadProfileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadProfileError::Io { message, path } => write!(f, "Failed to read profiles from '{}', because: {}", path.display(), message),
            ReadProfileError::Deserialize { message } => write!(f, "{}", message),
        }
    }
}

#[derive(Clone, Debug)]
pub enum WriteProfileError {
    Io { message: String, path: PathBuf },
    Serialize { message: String },
}

impl std::error::Error for WriteProfileError {}

impl std::fmt::Display for WriteProfileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteProfileError::Io { message, path } => write!(f, "Failed to write profiles to '{}' because: {}", path.display(), message),
            WriteProfileError::Serialize { message } => write!(f, "{}", message),
        }
    }
}
