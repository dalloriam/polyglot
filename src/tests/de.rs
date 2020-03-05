use std::collections::HashMap;

use serde::Deserialize;

use crate::{de, Format};

#[derive(Deserialize)]
#[serde(untagged)]
enum FieldValue {
    String(String),
    Int(i32),
    Bool(bool),
    Float(f32),
    Vec(Vec<FieldValue>),
    Map(HashMap<String, FieldValue>),
}

type GeneralBody = HashMap<String, FieldValue>;

macro_rules! de_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (body, format) = $value;
                let hsh: GeneralBody = de::from_str(body, format).unwrap();
                assert!(!hsh.is_empty());
            }
        )*
    }
}

de_tests! {
    simple_json: (include_str!("data/1.json"), Format::JSON),
    simple_toml: (include_str!("data/1.toml"), Format::TOML),
    // TODO: Tests for msgpack deserialization & failure tests.
}