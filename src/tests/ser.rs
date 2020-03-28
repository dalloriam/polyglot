use serde::Serialize;

use crate::ser;
use crate::Format;

macro_rules! ser_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let ser_str = $value;
                for frmt in vec![Format::JSON, Format::MsgPack, Format::TOML, Format::YAML].iter() {
                    let v = ser::to_vec(&ser_str, *frmt).unwrap();
                    assert!(!v.is_empty());
                }
            }
        )*
    }
}

#[derive(Serialize)]
struct TestStructA {
    name: String,
    age: i32,
    is_nice: bool,
}

ser_tests! {
    base_struct: TestStructA{name: String::from("John"), age: 18, is_nice: true},
}
