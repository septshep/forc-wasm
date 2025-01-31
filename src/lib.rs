mod build;
mod greet;
mod utils;

pub use greet::greet;

pub use build::build;
pub use build::BuildParams;
pub use build::BuildResult;

const FORC_PKG_VERSION: &str = "0.66.6";
