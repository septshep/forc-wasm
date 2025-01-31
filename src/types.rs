use forc_pkg as pkg;
use serde::Serialize;
use wasm_bindgen::prelude::*;

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
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildResult {
    pub abi: String,           // The ABI (Application Binary Interface) of the contract
    pub bytecode: String,      // The bytecode of the contract
    pub storage_slots: String, // Information about storage slots
    pub forc_version: String,  // The version of Forc used

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>, // Error message if the compilation failed
}

#[wasm_bindgen]
impl BuildResult {
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
            bytecode: hex::encode(&pkg.bytecode.bytes),
            storage_slots: serde_json::to_string(&pkg.storage_slots).unwrap(),
            forc_version: FORC_PKG_VERSION.into(),
            ..Default::default()
        }
    }
}
