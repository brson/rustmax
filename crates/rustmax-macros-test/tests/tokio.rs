//! `rmx::tokio::main` and `rmx::tokio::test` through the re-export.
//!
//! `tokio`'s own entry-point macros name `tokio` relative to the use site,
//! so `#[rmx::tokio::test]` used to fail unless `tokio` had been imported.
//! Nothing here imports it.

/// The common spelling, with no arguments.
#[rmx::tokio::test]
async fn bare_test() {
    rmx::tokio::task::yield_now().await;
}

/// Arguments pass through to `tokio`, with the crate path added after them.
#[rmx::tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_with_arguments() {
    let handle = rmx::tokio::runtime::Handle::current();
    assert_eq!(handle.runtime_flavor(), rmx::tokio::runtime::RuntimeFlavor::MultiThread);
    rmx::tokio::spawn(async {}).await.unwrap();
}

/// An explicit crate path is left as given.
#[rmx::tokio::test(crate = "rmx::tokio")]
async fn test_with_explicit_crate() {}

/// `tokio::main` works on any function, which makes it testable here.
#[rmx::tokio::main(flavor = "current_thread")]
async fn blocking_helper() -> u32 {
    rmx::tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    7
}

#[test]
fn main_wraps_a_function_in_a_runtime() {
    assert_eq!(blocking_helper(), 7);
}

mod imported {
    use rmx::tokio::{main, test};

    #[main]
    async fn helper() -> &'static str {
        "imported"
    }

    #[test]
    async fn imported_names_work() {
        rmx::tokio::task::yield_now().await;
    }

    #[::core::prelude::v1::test]
    fn imported_main_works() {
        assert_eq!(helper(), "imported");
    }
}

/// The style the guide used to require still works alongside.
mod scoped {
    use rmx::tokio;

    #[tokio::test]
    async fn scoped_name_works() {
        tokio::task::yield_now().await;
    }
}
