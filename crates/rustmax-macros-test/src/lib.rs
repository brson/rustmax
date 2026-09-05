//! Tests for [`rmx::derive`] as an outside consumer sees it.
//!
//! This crate depends on `rustmax` under the name `rmx`,
//! and takes no direct dependency on any of the re-exported derive crates.
//! That is the situation `#[rmx::derive]` exists to handle,
//! and it cannot be reproduced from inside the `rustmax` package itself,
//! where every re-exported crate is already a direct dependency.

#[rmx::derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub some_field: u32,
    #[serde(default)]
    pub other: String,
}

#[rmx::derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Message {
    Ping,
    Data { id: u32 },
}

/// Generics and bounds pass through to serde untouched.
#[rmx::derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Wrapper<T> {
    pub value: T,
}

/// Borrowed data still works, which is the case a shadow type would
/// struggle with.
#[rmx::derive(Deserialize, Debug, PartialEq)]
pub struct Borrowed<'a> {
    pub text: &'a str,
}

#[rmx::derive(Parser, Debug)]
pub struct Cli {
    #[arg(long)]
    pub count: u32,
}

#[rmx::derive(Subcommand, Debug)]
pub enum Command {
    Build,
    Run,
}

#[rmx::derive(Display, From, Debug)]
#[display("value is {_0}")]
pub struct Value(pub u32);

/// A `derive_more` derive whose name is shadowed by a built-in has to be
/// qualified.
#[rmx::derive(derive_more::Debug)]
pub struct CustomDebug {
    #[debug("redacted")]
    pub secret: String,
}

#[rmx::derive(TryFromPrimitive, IntoPrimitive, Debug, PartialEq)]
#[repr(u8)]
pub enum Kind {
    A = 1,
    B = 2,
}

/// Two crates needing two different kinds of fixup on one item.
#[rmx::derive(Serialize, TryFromPrimitive, Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Mixed {
    One = 1,
}

/// A second clap item in the same module, to confirm the scope shims
/// emitted by each invocation do not collide.
#[rmx::derive(Parser, Debug)]
pub struct OtherCli {
    #[arg(long)]
    pub name: String,
}

/// Fully qualified names route the same way as bare ones.
#[rmx::derive(serde::Serialize, clap::Parser)]
pub struct Qualified {
    #[arg(long)]
    pub flag: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmx::clap::Parser as _;

    #[test]
    fn serde_struct_round_trips() {
        let config = Config {
            some_field: 7,
            other: "hi".to_owned(),
        };
        let json = rmx::serde_json::to_string(&config).unwrap();
        assert_eq!(json, r#"{"someField":7,"other":"hi"}"#);
        assert_eq!(
            rmx::serde_json::from_str::<Config>(&json).unwrap(),
            config
        );
    }

    #[test]
    fn serde_container_attributes_apply() {
        let json = rmx::serde_json::to_string(&Message::Data { id: 3 }).unwrap();
        assert_eq!(json, r#"{"kind":"data","id":3}"#);
    }

    #[test]
    fn serde_generics_work() {
        let json = rmx::serde_json::to_string(&Wrapper { value: 1u32 }).unwrap();
        assert_eq!(json, r#"{"value":1}"#);
        assert_eq!(
            rmx::serde_json::from_str::<Wrapper<u32>>(&json).unwrap(),
            Wrapper { value: 1 }
        );
    }

    #[test]
    fn serde_borrowed_data_works() {
        let borrowed: Borrowed = rmx::serde_json::from_str(r#"{"text":"hi"}"#).unwrap();
        assert_eq!(borrowed.text, "hi");
    }

    #[test]
    fn clap_derives_work() {
        assert_eq!(Cli::parse_from(["prog", "--count", "4"]).count, 4);
        assert_eq!(OtherCli::parse_from(["prog", "--name", "n"]).name, "n");
    }

    #[test]
    fn derive_more_derives_work() {
        assert_eq!(Value::from(3).to_string(), "value is 3");
        assert_eq!(
            format!(
                "{:?}",
                CustomDebug {
                    secret: "s".to_owned()
                }
            ),
            "CustomDebug { secret: redacted }"
        );
    }

    #[test]
    fn num_enum_derives_work() {
        assert_eq!(Kind::try_from(2u8).unwrap(), Kind::B);
        assert_eq!(u8::from(Kind::A), 1);
        assert_eq!(Mixed::try_from(1u8).unwrap(), Mixed::One);
    }

    #[test]
    fn mixed_fixups_coexist() {
        assert_eq!(rmx::serde_json::to_string(&Mixed::One).unwrap(), r#""One""#);
    }
}
