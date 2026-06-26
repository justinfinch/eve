# Lesson 2 — The Contract (Single Source of Truth)

## What you'll learn

What a Rust "crate" is, what the `SelfId` type is, and why this tiny crate is the most
important file in the whole project even though it's only ~20 lines.

## The concept

A **crate** is Rust's word for a package — a unit of code that compiles together. This
project has several crates; `contract` is the smallest and most central.

`contract` defines **one data type**: `SelfId`. This is the message a molecule uses to
announce itself. Because *everyone else* (the simulator, the firmware, and — via
generated code — the browser) refers back to this one definition, we call it the
**single source of truth (SSOT)**. Change the shape here, and every other part updates
from it. There is no second place where the message shape is written by hand.

## Walk the code

The whole crate is `crates/contract/src/lib.rs`. Let's read it top to bottom.

```rust
#![cfg_attr(not(any(test, feature = "std", feature = "codegen")), no_std)]
```

This is the most unusual line. `no_std` means "don't assume the full standard library
is available." Microcontrollers don't have an operating system, so they don't have all
of `std` (no files, no threads, no heap-by-default). By being `no_std`, this crate can
compile for a microcontroller.

The `cfg_attr(...)` wrapper makes it conditional: *be `no_std` **unless** we're running
tests, or building with the `std` feature, or the `codegen` feature.* In those cases we
*do* have the full standard library and want it. So the same source serves both the
tiny embedded world and the big laptop world.

```rust
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
```

Even without `std`, we still want `String` and `Vec` (a growable list). Those come from
`alloc` — a slice of the standard library that only needs a memory allocator, not a
full OS. (The firmware sets up that allocator; see [Lesson 6](06-the-firmware.md).)

```rust
use serde::{Deserialize, Serialize};
```

**serde** is Rust's serialization library. "Serialize" = turn a Rust value into bytes
(here, JSON text). "Deserialize" = the reverse. This is how `SelfId` becomes the JSON
that travels over the WebSocket.

```rust
#[cfg(feature = "codegen")]
use tsify_next::Tsify;
#[cfg(feature = "codegen")]
use wasm_bindgen::prelude::wasm_bindgen;
```

These two imports only exist when the `codegen` feature is on. They're the machinery
that generates the TypeScript type — irrelevant to the firmware and simulator, so
they're gated off by default. We cover them in [Lesson 4](04-codegen.md).

```rust
#[cfg_attr(feature = "codegen", derive(Tsify))]
#[cfg_attr(feature = "codegen", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SelfId {
    pub id: String,
    pub name: String,
    pub fw_version: String,
    pub capabilities: Vec<String>,
}
```

This is the star of the show. A `struct` is a record with named fields:

- `id` — a stable identifier, e.g. `"mol-001"`.
- `name` — human-friendly name, e.g. `"Mini-Molecule"`.
- `fw_version` — firmware version string.
- `capabilities` — a list of what the molecule can do. **Empty for now** — the
  Foundation doesn't claim any capabilities yet; later rungs fill this in.

The `#[derive(...)]` line is Rust automatically writing boilerplate for us: `Serialize`
/ `Deserialize` (serde, for JSON), `Debug` (printable), `Clone` (copyable), `PartialEq`
(comparable with `==`). The two `#[cfg_attr(feature = "codegen", ...)]` lines add the
TypeScript-generation derives **only** when codegen is on.

```rust
#[cfg(test)]
mod tests {
    ...
    #[test]
    fn self_id_serializes_to_expected_json() {
        let s = SelfId { id: "mol-001".into(), name: "Mini-Molecule".into(),
                         fw_version: "0.1.0".into(), capabilities: Vec::new() };
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(
            json,
            r#"{"id":"mol-001","name":"Mini-Molecule","fw_version":"0.1.0","capabilities":[]}"#
        );
    }
    ...
}
```

Two tests, run only under `#[cfg(test)]`:

1. **`self_id_serializes_to_expected_json`** — pins the *exact* JSON text the type
   produces, including field order. This is what guarantees the bytes on the wire are
   precisely what we expect.
2. **`self_id_roundtrips`** (just below it) — serialize then deserialize, and confirm
   you get the identical value back.

### The manifest: `crates/contract/Cargo.toml`

`Cargo.toml` is a crate's manifest (name, dependencies, settings).

```toml
[lib]
crate-type = ["rlib"]   # a normal Rust library others can link against
```

It's an `rlib` (Rust library) so the firmware and simulator can depend on it. Notice it
is **not** a `cdylib` here — that special wasm output is produced on demand only during
codegen, so normal builds stay clean ([Lesson 4](04-codegen.md) explains why).

```toml
[dependencies]
serde = { version = "1", default-features = false, features = ["derive", "alloc"] }
tsify-next = { version = "0.5", optional = true }
wasm-bindgen = { version = "=0.2.100", optional = true }  # must equal devbox wasm-bindgen-cli version
```

`serde` uses `default-features = false` so it doesn't drag in `std` — important for the
`no_std` embedded build. `tsify-next` and `wasm-bindgen` are `optional = true`: only
pulled in when a feature asks for them.

```toml
[features]
default = []
std = ["serde/std"]                          # turn on the std parts of serde
codegen = ["std", "dep:tsify-next", "dep:wasm-bindgen"]  # the TS-generation extras
```

**Features** are optional switches. By default (`default = []`), none are on — that's
the lean `no_std` build the firmware uses. The `simulator` turns on `std`. The codegen
flow turns on `codegen`. One source file, three build modes.

## Run it yourself

Run the tests — this proves the message shape and its exact JSON:

```bash
devbox run -- cargo test -p contract
```

You should see `self_id_serializes_to_expected_json` and `self_id_roundtrips` pass.
(`-p contract` means "the package named contract.")

Confirm it builds in lean `no_std` mode (the way the firmware will use it):

```bash
devbox run -- cargo build -p contract
```

### Try it yourself

Open `crates/contract/src/lib.rs` and add a field to `SelfId`, e.g.
`pub serial: String,`. Re-run `cargo test -p contract`. The
`self_id_serializes_to_expected_json` test will **fail**, because the JSON no longer
matches. That failure is the SSOT doing its job: it won't let the message shape change
silently. (Revert the change afterward.)

## Recap

- A **crate** is a Rust package; `contract` is the smallest and most central.
- **`SelfId`** is the one message shape — defined **once**, here.
- `#![no_std]` (conditionally) lets the same type compile for a microcontroller; serde
  turns it into JSON.
- **Features** (`std`, `codegen`) let one source file serve the firmware, the simulator,
  and the TypeScript generator.

**Next:** [Lesson 3 — The simulator](03-the-simulator.md), which takes this `SelfId` and
serves it over a WebSocket.
</content>
