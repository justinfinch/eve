#![cfg_attr(not(any(test, feature = "std", feature = "codegen")), no_std)]

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[cfg(feature = "codegen")]
use tsify_next::Tsify;
#[cfg(feature = "codegen")]
use wasm_bindgen::prelude::wasm_bindgen;

/// A typed, self-describing capability descriptor — the ADR's registry.
/// Internally tagged so the wire is self-describing: {"kind":"gpio",...}.
#[cfg_attr(feature = "codegen", derive(Tsify))]
#[cfg_attr(feature = "codegen", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Capability {
    Gpio { channels: u8, ops: Vec<String> },
}

/// The molecule's self-description — the spine every later rung extends.
#[cfg_attr(feature = "codegen", derive(Tsify))]
#[cfg_attr(feature = "codegen", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SelfId {
    pub id: String,
    pub name: String,
    pub fw_version: String,
    pub capabilities: Vec<Capability>,
}

/// A command argument. Untagged: serializes as a bare JSON scalar.
/// Deliberately a small closed enum (not serde_json::Value) to keep the
/// embedded target no_std-clean and the heap small.
#[cfg_attr(feature = "codegen", derive(Tsify))]
#[cfg_attr(feature = "codegen", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Arg {
    Bool(bool),
    Int(i64),
    Text(String),
}

/// Host -> Device command, addressed as (capability, channel, op, args).
#[cfg_attr(feature = "codegen", derive(Tsify))]
#[cfg_attr(feature = "codegen", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Command {
    pub capability: String,
    pub channel: u8,
    pub op: String,
    pub args: Vec<Arg>,
}

/// Device -> Host framed message. Internally tagged on "type".
#[cfg_attr(feature = "codegen", derive(Tsify))]
#[cfg_attr(feature = "codegen", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DeviceMsg {
    SelfId(SelfId),
    Ack { ok: bool, error: Option<String> },
}

/// Validate a command against advertised capabilities and, on success,
/// return the desired GPIO level. Shared by firmware and simulator so both
/// enforce identical invariants (address only advertised capabilities,
/// channel in range, op advertised, args well-formed).
pub fn resolve_gpio_set(caps: &[Capability], cmd: &Command) -> Result<bool, String> {
    if cmd.capability != "gpio" {
        return Err(format!("unknown capability {:?}", cmd.capability));
    }
    let (channels, ops) = caps
        .iter()
        .find_map(|c| match c {
            Capability::Gpio { channels, ops } => Some((*channels, ops)),
        })
        .ok_or_else(|| "gpio capability not advertised".to_string())?;
    if cmd.channel >= channels {
        return Err(format!("channel {} out of range (0..{})", cmd.channel, channels));
    }
    if cmd.op != "set" || !ops.iter().any(|o| o == "set") {
        return Err(format!("unknown op {:?}", cmd.op));
    }
    match cmd.args.as_slice() {
        [Arg::Bool(level)] => Ok(*level),
        _ => Err("gpio set expects args [bool]".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    fn gpio_self_id() -> SelfId {
        SelfId {
            id: "mol-001".into(),
            name: "Mini-Molecule".into(),
            fw_version: "0.1.0".into(),
            capabilities: vec![Capability::Gpio {
                channels: 1,
                ops: vec!["set".into()],
            }],
        }
    }

    #[test]
    fn self_id_serializes_with_typed_capability() {
        let json = serde_json::to_string(&gpio_self_id()).unwrap();
        assert_eq!(
            json,
            r#"{"id":"mol-001","name":"Mini-Molecule","fw_version":"0.1.0","capabilities":[{"kind":"gpio","channels":1,"ops":["set"]}]}"#
        );
    }

    #[test]
    fn device_msg_selfid_is_internally_tagged() {
        let json = serde_json::to_string(&DeviceMsg::SelfId(gpio_self_id())).unwrap();
        assert!(json.starts_with(r#"{"type":"selfid","id":"mol-001""#), "got {json}");
    }

    #[test]
    fn device_msg_ack_serializes() {
        let json = serde_json::to_string(&DeviceMsg::Ack { ok: true, error: None }).unwrap();
        assert_eq!(json, r#"{"type":"ack","ok":true,"error":null}"#);
    }

    #[test]
    fn command_roundtrips() {
        let cmd = Command {
            capability: "gpio".into(),
            channel: 0,
            op: "set".into(),
            args: vec![Arg::Bool(true)],
        };
        let json = serde_json::to_string(&cmd).unwrap();
        assert_eq!(
            json,
            r#"{"capability":"gpio","channel":0,"op":"set","args":[true]}"#
        );
        let back: Command = serde_json::from_str(&json).unwrap();
        assert_eq!(back.args, vec![Arg::Bool(true)]);
    }

    #[test]
    fn resolve_gpio_set_accepts_valid_command() {
        let caps = gpio_self_id().capabilities;
        let cmd = Command {
            capability: "gpio".into(),
            channel: 0,
            op: "set".into(),
            args: vec![Arg::Bool(true)],
        };
        assert_eq!(resolve_gpio_set(&caps, &cmd), Ok(true));
    }

    #[test]
    fn resolve_gpio_set_rejects_out_of_range_channel() {
        let caps = gpio_self_id().capabilities;
        let cmd = Command {
            capability: "gpio".into(),
            channel: 5,
            op: "set".into(),
            args: vec![Arg::Bool(true)],
        };
        assert!(resolve_gpio_set(&caps, &cmd).is_err());
    }

    #[test]
    fn resolve_gpio_set_rejects_unknown_op() {
        let caps = gpio_self_id().capabilities;
        let cmd = Command {
            capability: "gpio".into(),
            channel: 0,
            op: "pulse".into(),
            args: vec![Arg::Bool(true)],
        };
        assert!(resolve_gpio_set(&caps, &cmd).is_err());
    }
}
