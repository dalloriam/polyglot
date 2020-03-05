use crate::Format;

macro_rules! format_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (fmt_enum, expected_ref) = $value;
                assert_eq!(fmt_enum.as_ref(), expected_ref);
            }
        )*
    }
}

format_tests! {
    asref_json: (Format::JSON, "json"),
    asref_toml: (Format::TOML, "toml"),
    asref_msgpack: (Format::MsgPack, "msgpack"),
}
