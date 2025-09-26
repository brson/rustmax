// Rustmax Suite - Developer Tools Hub
// Public API surface for the suite library.

use rmx::prelude::*;

pub mod cli;
pub mod infrastructure;
pub mod services;
pub mod web;

// Re-export commonly used types.
pub use infrastructure::{config::Config, logging::init_logging, state::AppState};

// Library result type.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// Version information.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");