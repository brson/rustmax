use super::*;

impl Tool {
    fn display_name(&self) -> &str {
        use Tool::*;
        match self {
            Rustup => "rustup",

            Cargo => "cargo",

            _ => todo!(),
        }
    }
}
