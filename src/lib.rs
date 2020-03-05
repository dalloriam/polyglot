mod error;
mod format;

/// Deserialization-related functionality.
pub mod de;

/// Serialization-related functionality.
pub mod ser;

#[doc(inline)]
pub use de::{from_reader, from_str};

#[doc(inline)]
pub use error::{Error, PolyResult as Result};

#[doc(inline)]
pub use format::Format;

#[doc(inline)]
pub use ser::{to_string, to_vec, to_writer};

#[cfg(test)]
mod tests;
