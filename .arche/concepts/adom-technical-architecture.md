---
type: concept
title: Adom Technical Architecture
created: 2026-06-17
updated: 2026-06-17
tags: [adom, architecture, rust, embassy, can-fd, klipper, adc, typescript, decoded]
sources: [sources/adom-decoded-and-poc-plan.md]
---

# Adom Technical Architecture

Adom's engineering stack, reconstructed from the public `github.com/adom-inc` org (25 repos): modular PCBs ("molecules") on a **CAN-FD** bus running **Rust/Embassy** firmware, physically wired by **Klipper-driven** robot arms, measured by **precision ADCs**, and exposed through a **Rust→TypeScript→browser** control plane with AI agents on top ([source](../sources/adom-decoded-and-poc-plan.md)).

## Explanation

This is decoded/inferred from public repos, not an internal disclosure — but the signals are strong and mutually reinforcing ([source](../sources/adom-decoded-and-poc-plan.md)).

**Language & framework spine — Rust, embedded-async.** `embassy` (fork) is the modern Rust async embedded framework that serves as the firmware foundation. `postcard-rpc` is an RPC layer over the `postcard` wire format for embedded-Rust↔host messaging. `tsify` generates TypeScript types from Rust — the smoking gun for a Rust-backend ↔ TypeScript-browser control plane. Supporting plumbing: `assign-resources`, `modular-bitfield`, `spi-pio` ([source](../sources/adom-decoded-and-poc-plan.md)).

**Module bus — CAN-FD everywhere.** `mcp2518fd`/`mcp2515` are `#![no_std]` drivers for CAN/CAN-FD controllers; `slcanx`/`slcan_fd` tunnel CAN over serial (the module↔host bridge); `socketcan-rs` provides Linux SocketCAN access. Reading: molecules talk to each other and the workcell over CAN-FD, bridged to a Linux host — robust, multi-drop, industrial, and matching the EE job's "I2C, SPI, CANBUS" list ([source](../sources/adom-decoded-and-poc-plan.md)).

**Robotics — Gcode motion control.** `klipper` (fork, "to allow use of up to 8 axes at once") is the robot-pincer / wire-bending motion layer; `tmc2240` (Trinamic stepper driver) and `as5047d` (absolute magnetic encoder) are the closed-loop motion hardware drivers ([source](../sources/adom-decoded-and-poc-plan.md)).

**Measurement & power front-end.** `ads1220`/`ads123x` (precision delta-sigma ADCs), `cdcx913` (PLL clock), `Modular_BMS` (C++ battery management) — the instrumentation that makes a molecule remotely measurable ([source](../sources/adom-decoded-and-poc-plan.md)).

**Design automation (EDA).** `kicad_lib`, `gerber_lib`, `fusion-360-electronics-adom-library` — Rust libraries to programmatically read/modify KiCad, Gerber, and Fusion 360 Electronics files; the "hardware as code" layer ([source](../sources/adom-decoded-and-poc-plan.md)).

**Client.** `hd-wsl2-image` ("Golden WSL2 rootfs image for Hydrogen Desktop") suggests a local client/agent environment bridging the browser to hardware — a generalization of [John Lauer](../entities/john-lauer.md)'s SPJS/ChiliPeppr browser-to-serial pattern ([source](../sources/adom-decoded-and-poc-plan.md)).

## Examples

- Repos cited as MIT/Apache licensed and directly reusable in a POC: `postcard-rpc`, `mcp2515`, `tsify`, the Embassy fork ([source](../sources/adom-decoded-and-poc-plan.md)).
- The Rust→TypeScript bridge (via `tsify`) lets the same message structs define both firmware and browser types — the pattern the [POC](poc-mini-molecule-cloud-workbench.md) mirrors ([source](../sources/adom-decoded-and-poc-plan.md)).

## See also

- [Adom Industries](../entities/adom-industries.md)
- [Molecules and Workcells](molecules-and-workcells.md)
- [Instrument-Control Standards](instrument-control-standards.md)
- [POC: Mini-Molecule + Cloud Workbench](poc-mini-molecule-cloud-workbench.md)
