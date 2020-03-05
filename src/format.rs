use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use crate::Error;

/// Available serialization formats.
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Format {
    /// JSON serialization (via the `serde_json` crate).
    ///
    /// Enabled with the `json_fmt` feature.
    #[cfg(feature = "json_fmt")]
    JSON,

    /// MsgPack serialization (via the `rmp-serde` crate).
    ///
    /// Enabled with the `msgpack_fmt` feature.
    #[cfg(feature = "msgpack_fmt")]
    MsgPack,

    /// MsgPack serialization (via the `toml` crate).
    ///
    /// Enabled with the `toml_fmt` feature.
    #[cfg(feature = "toml_fmt")]
    TOML,
}

impl TryFrom<&str> for Format {
    type Error = Error;
    fn try_from(v: &str) -> crate::Result<Format> {
        #[cfg(feature = "json_fmt")]
            {
                if v == "json" {
                    return Ok(Format::JSON);
                }
            }
        #[cfg(feature = "msgpack_fmt")]
            {
                if v == "msgpack" {
                    return Ok(Format::MsgPack);
                }
            }

        #[cfg(feature = "toml_fmt")]
            {
                if v == "toml" {
                    return Ok(Format::TOML);
                }
            }

        Err(Error::new(format!("Unknown format {}", v)))
    }
}

impl AsRef<str> for Format {
    fn as_ref(&self) -> &str {
        #[cfg(feature = "json_fmt")]
        {
            if self == &Format::JSON {
                return "json";
            }
        }

        #[cfg(feature = "msgpack_fmt")]
        {
            if self == &Format::MsgPack {
                return "msgpack";
            }
        }

        #[cfg(feature = "toml_fmt")]
        {
            if self == &Format::TOML {
                return "toml";
            }
        }

        "unknown"
    }
}