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

/// The canonical self-description every molecule advertises — the single home for
/// id / name / fw_version and the GPIO capability. Firmware and simulator both call
/// this so the real device and the simulator can never drift (FR-6). Bumping the
/// firmware version or adding a channel/op happens here, once.
pub fn default_self_id() -> SelfId {
    SelfId {
        id: "mol-001".into(),
        name: "Mini-Molecule".into(),
        fw_version: "0.1.0".into(),
        capabilities: alloc::vec![Capability::Gpio {
            channels: 1,
            ops: alloc::vec!["set".into()],
        }],
    }
}

/// Longest partial (unterminated) line a byte-stream framer should buffer before
/// assuming the peer is streaming garbage and discarding the tail. Commands are
/// ~60 bytes; this is generous headroom while staying small enough to bound memory
/// on the firmware's 8 KB heap. Firmware and the host bridge both frame the same
/// serial stream with `take_line`, so they share this cap and can't silently diverge.
pub const MAX_LINE: usize = 512;

/// Drain the next complete `\n`-terminated line from `buf`, returning its bytes
/// without the trailing newline (and a stray `\r`); leaves any partial tail in
/// `buf`. Returns `None` when no complete line is buffered. `no_std`-clean so
/// firmware and the host bridge frame the serial stream identically.
pub fn take_line(buf: &mut Vec<u8>) -> Option<Vec<u8>> {
    let pos = buf.iter().position(|&b| b == b'\n')?;
    let mut line: Vec<u8> = buf.drain(..=pos).collect();
    line.pop(); // drop the '\n'
    if line.last() == Some(&b'\r') {
        line.pop();
    }
    Some(line)
}

/// Validate a command against advertised capabilities and, on success,
/// return the desired GPIO level. Shared by firmware and simulator so both
/// enforce identical invariants (address only advertised capabilities,
/// channel in range, op advertised, args well-formed).
pub fn resolve_gpio_set(caps: &[Capability], cmd: &Command) -> Result<bool, String> {
    if cmd.capability != "gpio" {
        return Err(format!("unknown capability {:?}", cmd.capability));
    }
    let gpio = caps
        .iter()
        .find(|c| matches!(c, Capability::Gpio { .. }));
    let Some(Capability::Gpio { channels, ops }) = gpio else {
        return Err("gpio capability not advertised".to_string());
    };
    let channels = *channels;
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
        default_self_id()
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
    fn take_line_drains_complete_lines_and_keeps_the_tail() {
        let mut buf = b"one\ntwo\npar".to_vec();
        assert_eq!(take_line(&mut buf).as_deref(), Some(&b"one"[..]));
        assert_eq!(take_line(&mut buf).as_deref(), Some(&b"two"[..]));
        assert_eq!(take_line(&mut buf), None); // partial tail stays buffered
        assert_eq!(buf, b"par");
    }

    #[test]
    fn take_line_trims_trailing_cr() {
        let mut buf = b"crlf\r\n".to_vec();
        assert_eq!(take_line(&mut buf).as_deref(), Some(&b"crlf"[..]));
        assert!(buf.is_empty());
    }

    #[test]
    fn default_self_id_advertises_one_gpio_channel() {
        let caps = default_self_id().capabilities;
        assert_eq!(caps, vec![Capability::Gpio { channels: 1, ops: vec!["set".into()] }]);
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
