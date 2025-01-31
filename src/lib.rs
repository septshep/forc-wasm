mod build;
mod console;
mod greet;
mod types;
mod utils;

// Re-export the public API
pub use greet::greet;

pub use build::build;
pub use types::BuildParams;
pub use types::BuildResult;

const FORC_PKG_VERSION: &str = "forc 0.66.6";
