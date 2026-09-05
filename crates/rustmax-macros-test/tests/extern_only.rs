//! `#[rmx::derive]` with a derive crate that only works as a direct dependency.
//!
//! `thiserror` generates `::thiserror` paths with no override attribute,
//! so `#[rmx::derive(Error)]` routes to the direct dependency or fails.
//! Here `thiserror` is a dev-dependency,
//! which puts it in this test target's extern prelude
//! but not the library's.

use rmx::prelude::*;

#[rmx::derive(Error, Debug)]
#[error("bad value: {0}")]
struct BadValue(u32);

#[rmx::derive(derive_more::Error, Display, Debug)]
#[display("no direct dependency needed")]
struct ViaDeriveMore;

#[test]
fn thiserror_derive_works() {
    assert_eq!(BadValue(3).to_string(), "bad value: 3");
    let any: AnyError = BadValue(3).into();
    assert_eq!(any.to_string(), "bad value: 3");
}

#[test]
fn derive_more_error_works() {
    let error: &dyn std::error::Error = &ViaDeriveMore;
    assert_eq!(error.to_string(), "no direct dependency needed");
}
