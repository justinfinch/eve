#![cfg_attr(not(any(test, feature = "std", feature = "codegen")), no_std)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[cfg(feature = "codegen")]
use tsify_next::Tsify;
#[cfg(feature = "codegen")]
use wasm_bindgen::prelude::wasm_bindgen;

/// The molecule's self-description — the spine every later rung extends.
#[cfg_attr(feature = "codegen", derive(Tsify))]
#[cfg_attr(feature = "codegen", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SelfId {
    pub id: String,
    pub name: String,
    pub fw_version: String,
    pub capabilities: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_id_serializes_to_expected_json() {
        let s = SelfId {
            id: "mol-001".into(),
            name: "Mini-Molecule".into(),
            fw_version: "0.1.0".into(),
            capabilities: Vec::new(),
        };
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(
            json,
            r#"{"id":"mol-001","name":"Mini-Molecule","fw_version":"0.1.0","capabilities":[]}"#
        );
    }

    #[test]
    fn self_id_roundtrips() {
        let s = SelfId {
            id: "mol-001".into(),
            name: "Mini-Molecule".into(),
            fw_version: "0.1.0".into(),
            capabilities: Vec::new(),
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: SelfId = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }
}
