use serde::Serialize;

use crate::{Error, Format};

/// Serialize a struct in the given format, outputting a vector of bytes.
///
/// Obviously, only formats enabled with feature flags will be supported.
/// ## Example
/// ```
/// use polyglot::{ser, Format};
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// pub struct Person {
///     pub name: String
/// }
///
/// let p = Person{name: String::from("John")};
/// let out_bytes = ser::to_vec(&p, Format::TOML).unwrap();
/// ```
pub fn to_vec<T: Serialize>(val: &T, format: Format) -> crate::Result<Vec<u8>> {
    #[cfg(feature = "json_fmt")]
    {
        if format == Format::JSON {
            return Ok(serde_json::to_vec(val)?);
        }
    }

    #[cfg(feature = "msgpack_fmt")]
    {
        if format == Format::MsgPack {
            return Ok(rmp_serde::to_vec(val)?);
        }
    }

    #[cfg(feature = "toml_fmt")]
    {
        if format == Format::TOML {
            return Ok(toml::to_vec(val)?);
        }
    }

    Err(Error::new("Library Error - Unimplemented Serialize"))
}

/// Serialize a struct in the given format, outputting a string.
///
/// Obviously, only formats enabled with feature flags will be supported.
///
/// # Errors
/// Serializing to a string using msgpack will return an error due to MessagePack being a binary
/// format.
///
/// # Examples
/// ```
/// use polyglot::{ser, Format};
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// pub struct Person {
///     pub name: String
/// }
///
/// let p = Person{name: String::from("John")};
/// let out_str = ser::to_string(&p, Format::JSON).unwrap();
/// assert_eq!(&out_str, "{\"name\":\"John\"}");
/// ```
pub fn to_string<T: Serialize>(val: &T, format: Format) -> crate::Result<String> {
    #[cfg(feature = "json_fmt")]
    {
        if format == Format::JSON {
            return Ok(serde_json::to_string(val)?);
        }
    }

    #[cfg(feature = "msgpack_fmt")]
    {
        if format == Format::MsgPack {
            return Err(Error::new(
                "MessagePack is a binary format. Serializing to string is unsupported.",
            ));
        }
    }

    #[cfg(feature = "toml_fmt")]
    {
        if format == Format::TOML {
            return Ok(toml::to_string(val)?);
        }
    }

    Err(Error::new("Library Error - Unimplemented Serialize"))
}

pub fn to_writer<T: Serialize, V: std::io::Write>(
    mut w: V,
    val: &T,
    format: Format,
) -> crate::Result<()> {
    #[cfg(feature = "json_fmt")]
    {
        if format == Format::JSON {
            serde_json::to_writer(w, val)?;
            return Ok(());
        }
    }

    let vec_data = crate::to_vec(val, format)?;
    w.write_all(&vec_data)?;
    Ok(())
}
