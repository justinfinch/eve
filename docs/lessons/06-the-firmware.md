# Lesson 6 — The Firmware (real embedded Rust)

## What you'll learn

What "firmware" is, why it lives in a **separate** Cargo workspace, what `no_std`,
Embassy, and a "memory map" mean, and why simply *compiling* it (without a board
attached) is a meaningful milestone.

> **Remember the goal of this POC is hardware literacy** — learning how embedded code is
> structured and built. The Foundation deliberately does **not** flash a chip or run on
> hardware. "Getting it onto a board" is a later, separate step. Here, success = it
> *compiles* for the real chip.

## The concept

**Firmware** is the program that runs directly on a microcontroller — a tiny computer on
a chip, with no operating system, kilobytes of RAM, and a fixed memory layout. Our target
is the **RP2350** (the chip on a Raspberry Pi Pico 2), ARM Cortex-M architecture.

Writing for such a chip differs from laptop programming in three ways you'll see in the
code:

1. **`no_std`** — there's no operating system, so the full Rust standard library isn't
   available. You get `core` (always) and `alloc` (if you provide a memory allocator),
   but not files, threads, or `println!`. (This is why `contract` was built `no_std` in
   [Lesson 2](02-the-contract.md).)
2. **An async framework: [Embassy](https://embassy.dev/)** — a modern Rust framework that
   brings `async`/`await` to embedded, so you can write concurrent firmware without a
   traditional real-time OS.
3. **A memory map** — you must tell the linker exactly where flash and RAM live on this
   specific chip.

### Why a *separate* workspace?

Look at the host workspace root, `Cargo.toml`:

```toml
[workspace]
members = ["crates/contract", "crates/simulator"]
exclude = ["firmware", "web"]
```

The firmware is **excluded**. It lives in its own workspace (`firmware/Cargo.toml` has
its own `[workspace]`). Why split them?

Because the firmware compiles for a *different CPU* (ARM Cortex-M) with `no_std`, while
the simulator compiles for your laptop with the full `std`. If they shared one workspace,
a plain `cargo build` would try to build the embedded crate for your laptop and fail on
chip-specific code. Splitting them means: `cargo build --workspace` builds the host parts
cleanly and never drags in the embedded crate. They share code only through the
`contract` dependency.

## Walk the code

### `firmware/Cargo.toml`

```toml
[workspace]            # makes firmware its OWN workspace, standalone from the host

[package]
name = "firmware"
version = "0.1.0"
edition = "2021"

[dependencies]
contract = { path = "../crates/contract", default-features = false }  # SSOT, lean no_std

embassy-executor = { version = "0.7", features = ["arch-cortex-m", "executor-thread"] }
embassy-rp = { version = "0.4", features = ["rp235xa", "time-driver", "critical-section-impl"] }
cortex-m-rt = "0.7"   # Cortex-M runtime: startup code, vector table
panic-halt = "1"      # what to do on panic: just halt (no OS to print to)
embedded-alloc = "0.6" # a heap allocator, so alloc's String/Vec work
```

The key line is `contract = { ..., default-features = false }`: the firmware uses the
**lean `no_std`** version of the exact same contract the simulator uses. That's the
proof that the message shape is genuinely shared across the laptop world and the
embedded world — no duplicate.

`embassy-rp` with the `rp235xa` feature is the RP2350 chip support. (A note in the build
plan records that these versions were pinned carefully to match the project's Rust
1.85.0 — Embassy's 0.4 line — because newer Embassy releases need a newer Rust.)

### `firmware/src/main.rs`

```rust
#![no_std]      // no operating system / standard library
#![no_main]     // we don't use the normal `fn main` entry; Embassy provides one

use contract::SelfId;
use embassy_executor::Spawner;
use embedded_alloc::LlffHeap as Heap;
use panic_halt as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();
```

`#![no_std]` and `#![no_main]` are the embedded markers. `#[global_allocator]` registers
a heap so that `alloc` types (`String`, `Vec`) can work — the contract uses those.

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize the heap so the contract's alloc-backed types link/run.
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 4096;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(core::ptr::addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
    }

    let _p = embassy_rp::init(Default::default());   // bring up the chip's peripherals
```

`#[embassy_executor::main]` is Embassy's entry point (replacing the normal `main`). It
carves out 4 KB of static memory as the heap, then initializes the RP2350. On a real
board, `_p` would be the handle you use to talk to GPIO pins, ADC, etc.

```rust
    // Prove the firmware shares the single-source-of-truth contract (FR-6).
    let _id = SelfId {
        id: "mol-001".into(),
        name: "Mini-Molecule".into(),
        fw_version: "0.1.0".into(),
        capabilities: Default::default(),
    };

    loop {
        core::hint::spin_loop();
    }
}
```

The firmware constructs a `SelfId` — the same type, same values as the simulator's
`self_id()` in [Lesson 3](03-the-simulator.md). It doesn't *do* anything with it yet;
constructing it is the point. It proves the embedded build can use the contract. Then it
spins forever (real firmware never returns from `main`).

### `firmware/memory.x` — the memory map

```
MEMORY {
    FLASH : ORIGIN = 0x10000000, LENGTH = 2048K
    RAM   : ORIGIN = 0x20000000, LENGTH = 512K
}
```

This tells the linker where the RP2350's flash (where code lives) and RAM (where data
lives) sit in the address space, and how big they are. On a laptop the OS handles this;
on bare metal you spell it out.

### `firmware/build.rs` and `firmware/.cargo/config.toml`

- **`build.rs`** runs at build time and copies `memory.x` to where the linker looks for
  it.
- **`.cargo/config.toml`** sets the default compile target to
  `thumbv8m.main-none-eabihf` (the RP2350's ARM Cortex-M33 target — one of the targets
  pinned in [Lesson 1](01-the-dev-environment.md)) and passes the linker flags Embassy
  needs.

```toml
# firmware/.cargo/config.toml
[build]
target = "thumbv8m.main-none-eabihf"

[target.thumbv8m.main-none-eabihf]
rustflags = ["-C", "link-arg=--nmagic", "-C", "link-arg=-Tlink.x"]
```

This config **only applies when your current directory is `firmware/`**. That's an
important quirk — see below.

## Run it yourself

⚠️ **The directory matters.** You must build from *inside* `firmware/`, because that's the
only place Cargo reads `firmware/.cargo/config.toml` (which sets the ARM target). Building
from the repo root would try to compile for your laptop and fail.

```bash
devbox run -- bash -c 'cd firmware && cargo build'
```

You should get a successful build with no board attached. Confirm it really produced ARM
code:

```bash
file firmware/target/thumbv8m.main-none-eabihf/debug/firmware
# ... ELF 32-bit ... ARM ...
```

Now confirm the **two-workspace split** works — the host build ignores firmware entirely:

```bash
devbox run -- cargo build --workspace
# builds contract + simulator only; never touches firmware
```

## Recap

- **Firmware** runs directly on a microcontroller (RP2350) with **no OS** — hence
  `no_std`, a manual heap, and a memory map.
- **Embassy** brings `async` Rust to embedded; `cortex-m-rt`, `panic-halt`, and
  `embedded-alloc` round out the bare-metal essentials.
- It lives in a **separate workspace** so the host build never tries to cross-compile it;
  the only shared code is the `contract` crate — proving the message shape is truly one
  source.
- In the Foundation we only **compile** it (`cd firmware && cargo build`). That it builds
  for the real chip, board-free, is the milestone.

**Next:** [Lesson 7 — The one-command spine](07-the-spine.md): the `justfile` that builds,
checks, and runs all four parts together.
</content>
