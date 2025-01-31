use forc_pkg as pkg;
use wasm_bindgen::prelude::*;

use crate::{console, BuildParams, BuildResult};

/// The `build` function is used to build the contract.
#[wasm_bindgen]
pub fn build(params: BuildParams) -> BuildResult {
    // Log the contract and toolchain to the console for debugging
    console::debug(format!("contract: {:?}", &params.contract));
    console::debug(format!("toolchain: {:?}", &params.toolchain));

    // Check if the contract is empty
    if params.contract.is_empty() {
        return BuildResult::error("No contract".into());
    }

    let opts = pkg::BuildOpts::default();
    let result = pkg::build_with_options(&opts);

    match result {
        Ok(built) => {
            console::info("Build succeeded");
            BuildResult::from(built)
        }
        Err(err) => {
            console::error(format!("Build failed: {:?}", err));
            BuildResult::error(err.to_string())
        }
    }
}
