---
type: concept
title: ADR — Message contract via tsify (JSON v1 / postcard-rpc v2)
created: 2026-06-22
updated: 2026-06-22
tags: [architecture, adr, poc, mini-molecule, contract, tsify, postcard-rpc, serialization]
sources: [concepts/sad-mini-molecule-poc.md, concepts/ard-mini-molecule-poc.md]
status: accepted
---

# ADR — Message contract via tsify (JSON v1 / postcard-rpc v2)

## Decision

We will define the message contract **once as serde types in Rust** and **generate the TypeScript types with `tsify`**, making the Rust types the single source of truth. Serialization is phased: **JSON over Web Serial for v1**, and **`postcard-rpc` at the bridge↔firmware (Rust↔embedded) seam for v2**, with the bridge translating to typed JSON/WebSocket for the browser. The *logical* contract (capabilities, commands, results, samples) is invariant; only the wire encoding changes, and only at the embedded seam.

## Context

The Arche's "killer move" is reusing Adom's own crates — `postcard-rpc` and `tsify` — and `tsify` is the [decoded "smoking gun"](adom-technical-architecture.md) of Adom's Rust↔TS control plane. But `postcard-rpc` is built for a Rust host talking to embedded, not for a *browser* client; in v1 the browser would have to reimplement postcard-rpc framing to decode a binary wire format. Splitting the contract from the serialization banks the `tsify` signal immediately while letting `postcard-rpc` enter where it is idiomatic. Framed by [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md) and required by [ADR — Phased control plane](adr-phased-control-plane.md).

## Alternatives considered

- **postcard-rpc everywhere from v1** — browser decodes postcard binary via generated TS from day one. Maximum single-serialization fidelity, but you reimplement postcard-rpc framing in TS and binary-on-the-wire is harder to inspect while building.
- **Plain JSON with hand-written types on both sides** — simplest and fully debuggable, but firmware and browser types can drift, and it forfeits the `tsify` single-source-of-truth — a real Adom signal left on the table.
- **gRPC / protobuf** — industry-standard typed RPC with codegen, but not Adom's stack, heavyweight for `no_std` embedded, and overkill for a desk POC.

## Consequences

- **Enables** — zero schema drift between firmware, simulator, bridge, and browser (one source, generated types); the `tsify` signal banked on day one; a debuggable JSON wire for v1; idiomatic `postcard-rpc` where it belongs in v2.
- **Costs** — a translation layer at the bridge seam in v2 (postcard ↔ JSON); a `tsify` codegen step wired into the build.
- **Locks in** — Rust as the home of the contract; the contract definitions become the dependency hub every other component imports.

## Status

`accepted`

## See also

- [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md) — the solution this decision is part of.
- [ADR — Phased control plane](adr-phased-control-plane.md) — why the contract must outlive the topology.
- [ADR — Capability registry](adr-capability-registry.md) — the domain model these types express.
- [Adom Technical Architecture](adom-technical-architecture.md) — `tsify` / `postcard-rpc` provenance.
