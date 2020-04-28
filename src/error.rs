use std::fmt;
use std::io;

/// Thin wrapper around serialization errors.
///
/// The `From<T>` trait will be defined for all serialization errors of format features enabled.
#[derive(Debug)]
pub struct Error {
    message: String,
}

impl std::error::Error for Error {}

impl Error {
    pub fn new<T>(msg: T) -> Error
    where
        String: From<T>,
    {
        Error {
            message: String::from(msg),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::new(format!("IO Error: {}", e.to_string()))
    }
}

#[cfg(feature = "json_fmt")]
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::new(format!("Polyglot/JSON Error: {}", e.to_string()))
    }
}

#[cfg(feature = "msgpack_fmt")]
impl From<rmp_serde::encode::Error> for Error {
    fn from(e: rmp_serde::encode::Error) -> Error {
        Error::new(format!("Polyglot/MsgPack Error: {}", e.to_string()))
    }
}

#[cfg(feature = "msgpack_fmt")]
impl From<rmp_serde::decode::Error> for Error {
    fn from(e: rmp_serde::decode::Error) -> Error {
        Error::new(format!("Polyglot/MsgPack Error: {}", e.to_string()))
    }
}

#[cfg(feature = "toml_fmt")]
impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Error {
        Error::new(format!("Polyglot/TOML Error: {}", e.to_string()))
    }
}

#[cfg(feature = "toml_fmt")]
impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Error {
        Error::new(format!("Polyglot/TOML Error: {}", e.to_string()))
    }
}

#[cfg(feature = "yaml_fmt")]
impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Error {
        Error::new(format!("Polyglot/YAML Error: {}", e.to_string()))
    }
}

/// Alias of `std::Result<T, Error>`
pub type PolyResult<T> = Result<T, Error>;
