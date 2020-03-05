use std::io;

use serde::de;

use crate::{Format, Error};

/// Deserialize a struct from a string in the specified format.
///
/// Obviously, only formats enabled with feature flags will be supported.
/// ## Example
/// ```
/// use polyglot::{de, Format};
/// use serde::Deserialize;
///
/// #[derive(Debug, Deserialize)]
/// struct Person {
///     pub age: i32
/// }
///
/// let p: Person = de::from_str("{\"age\": 42}", Format::JSON).unwrap();
/// assert_eq!(p.age, 42);
/// ```
pub fn from_str<'de, T>(s: &'de str, format: Format) -> crate::Result<T>
    where
        T: de::Deserialize<'de>,
{
    #[cfg(feature = "json_fmt")]
        {
            if format == Format::JSON {
                return Ok(serde_json::from_str(s)?);
            }
        }

    #[cfg(feature = "msgpack_fmt")]
        {
            if format == Format::MsgPack {
                return Ok(rmp_serde::from_read_ref(s)?);
            }
        }

    #[cfg(feature = "toml_fmt")]
        {
            if format == Format::TOML{
                return Ok(toml::from_str(s)?);
            }
        }

    Err(Error::new("Library Error - Unimplemented Deserialize"))
}

/// Deserialize a struct from a reader in the specified format.
///
/// Obviously, only formats enabled with feature flags will be supported.
/// ## Example
/// ```
/// use std::io;
///
/// use polyglot::{de, Format};
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// pub struct Person {
///     pub age: i32,
/// }
///
/// let data = "{\"age\": 42}";
/// let p: Person = de::from_reader(data.as_bytes(), Format::JSON).unwrap();
/// assert_eq!(p.age, 42);
/// ```
pub fn from_reader<R, T>(mut rdr: R, format: Format) -> crate::Result<T>
    where
        R: io::Read,
        T: de::DeserializeOwned,
{
    #[cfg(feature = "json_fmt")]
        {
            if format == Format::JSON {
                return Ok(serde_json::from_reader(rdr)?);
            }
        }

    #[cfg(feature = "msgpack_fmt")]
        {
            if format == Format::MsgPack {
                return Ok(rmp_serde::from_read(rdr)?);
            }
        }

    #[cfg(feature = "toml_fmt")]
        {
            if format == Format::TOML{
                let mut raw = String::new();
                rdr.read_to_string(&mut raw)?;
                return Ok(toml::from_str(&raw)?);
            }
        }

    Err(Error::new("Library Error - Unimplemented Deserialize"))
}
