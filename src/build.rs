use wasm_bindgen::prelude::*;
use web_sys::console;

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
#[derive(Debug, Clone)]
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
}

/// The `build` function is used to build the contract.
#[wasm_bindgen]
pub fn build(params: BuildParams) -> BuildResult {
    // Log the contract and toolchain to the console for debugging
    console::log_1(&format!("contract: {:?}", &params.contract).into());
    console::log_1(&format!("toolchain: {:?}", &params.toolchain).into());

    // TODO: Implement the build logic here
    // For now, we'll just return a dummy result
    BuildResult::new(
        "abi".to_string(),
        "bytecode".to_string(),
        "storage_slots".to_string(),
        "0.1.0".to_string(),
        None,
    )
}
