use forc_pkg as pkg;
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::FORC_PKG_VERSION;

/// The `BuildParams` struct is used to pass parameters to the `build` function.
#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct BuildParams {
    pub contract: String,  // The contract source code
    pub toolchain: String, // The toolchain to use
}

#[wasm_bindgen]
impl BuildParams {
    #[wasm_bindgen(constructor)]
    pub fn new(contract: String, toolchain: String) -> BuildParams {
        BuildParams {
            contract,
            toolchain,
        }
    }
}

/// The `BuildResult` struct is used to return the result of the `build` function.
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Default)]
pub struct BuildResult {
    pub abi: String,           // The ABI (Application Binary Interface) of the contract
    pub bytecode: String,      // The bytecode of the contract
    pub storage_slots: String, // Information about storage slots
    pub forc_version: String,  // The version of Forc used
    pub error: Option<String>, // Error message if the compilation failed
}

#[wasm_bindgen]
impl BuildResult {
    // Constructor for creating a new BuildResult instance
    #[wasm_bindgen(constructor)]
    pub fn new(
        abi: String,
        bytecode: String,
        storage_slots: String,
        forc_version: String,
        error: Option<String>,
    ) -> BuildResult {
        BuildResult {
            abi,
            bytecode,
            storage_slots,
            forc_version,
            error,
        }
    }

    pub fn error(message: String) -> BuildResult {
        BuildResult {
            error: Some(message),
            ..Default::default()
        }
    }
}

impl From<pkg::Built> for BuildResult {
    fn from(built: pkg::Built) -> Self {
        let pkg = built.expect_pkg().unwrap();
        BuildResult {
            abi: pkg.json_abi_string(true).unwrap().unwrap(),
            bytecode: serde_json::to_string(&pkg.bytecode.bytes).unwrap(),
            storage_slots: serde_json::to_string(&pkg.storage_slots).unwrap(),
            forc_version: FORC_PKG_VERSION.into(),
            ..Default::default()
        }
    }
}

/// The `build` function is used to build the contract.
#[wasm_bindgen]
pub fn build(params: BuildParams) -> BuildResult {
    // Log the contract and toolchain to the console for debugging
    console::log_1(&format!("contract: {:?}", &params.contract).into());
    console::log_1(&format!("toolchain: {:?}", &params.toolchain).into());

    let opts = pkg::BuildOpts::default();
    let result = pkg::build_with_options(&opts);

    match result {
        Ok(built) => {
            console::log_1(&"Build succeeded".into());
            built.into()
        }
        Err(e) => {
            console::error_1(&format!("Build failed: {:?}", e).into());
            BuildResult::error(e.to_string())
        }
    }
}
