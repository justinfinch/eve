# Lesson 0 — The Big Picture

## What you'll learn

What this project *is*, the four parts it's made of, and the single idea that holds
them together. By the end you'll be able to point at any file in the repo and say
which part it belongs to.

## The concept

The full product vision (see the [main README](../../README.md)) is a small modular
electronics board — a *"molecule"* — that announces what it can do, is driven from a
web browser, and measures/actuates real circuits. That's a lot. So we built the
**Foundation**: the thinnest possible end-to-end slice that still exercises the whole
shape of the system.

The Foundation does exactly one thing:

> A "molecule" announces **who it is** (an id, a name, a firmware version, a list of
> capabilities), and a **browser displays it**.

That's it. No real hardware, no sensors, no AI yet. But the *plumbing* to do all of
that later is already in place. This is sometimes called a "walking skeleton" or a
"spine": every layer is present and connected, even though each is currently very
thin.

### The one big idea: a single source of truth

A message travels from the molecule to the browser. That message has a shape — the
`SelfId`. Here's the trap most projects fall into: they define that shape **twice** —
once in the firmware language (Rust) and once in the browser language (TypeScript).
The two definitions drift apart, and you get bugs where the board sends one thing and
the browser expects another.

We avoid that. The shape is defined **once**, in Rust, in a crate called `contract`.
The TypeScript version is **generated automatically** from the Rust version. If they
ever disagree, a check fails the build. This is the heartbeat of the whole project —
[Lesson 4](04-codegen.md) is entirely about it.

## The four parts

```
eve/
├── crates/
│   ├── contract/     # PART 1: the SelfId message shape, in Rust. The hub.
│   └── simulator/    # PART 2: a fake "board" that serves SelfId over a WebSocket.
├── web/              # PART 3: the browser app that connects and shows id + name.
└── firmware/         # PART 4: real embedded Rust for a microcontroller (build-only).
```

| Part | Language | Role | Runs where |
|------|----------|------|------------|
| **contract** | Rust | Defines the `SelfId` message shape — the single source of truth | compiled into the others |
| **simulator** | Rust | Pretends to be a board; sends `SelfId` over a WebSocket | your laptop |
| **web** | TypeScript / React | Connects to the simulator, displays the message | your browser |
| **firmware** | Rust (embedded) | The *real* board code; imports the same `contract` | a microcontroller (we only *compile* it for now) |

Both the **simulator** and the **firmware** depend on **contract**. The **web** app
uses a TypeScript type *generated from* **contract**. So `contract` sits in the
middle, and everyone agrees on the message because they all derive from it.

## The data flow (Foundation)

```
  contract (Rust SelfId)
        │  defines the shape
        ├──────────────► simulator ──WebSocket(JSON)──► web browser
        │                (sends it)                     (shows id + name)
        │
        └──code generation──► web/src/contract.gen.ts
                              (the TypeScript the browser checks against)
```

1. `contract` defines `SelfId { id, name, fw_version, capabilities }`.
2. The `simulator` builds a `SelfId`, turns it into JSON, and sends it to anyone who
   connects over a WebSocket.
3. The `web` app connects, receives the JSON, parses it into a `SelfId` **typed by the
   generated `contract.gen.ts`**, and renders the `id` and `name`.
4. The `firmware` *also* uses `contract`, proving the very same message shape compiles
   for a real microcontroller — no duplicate definition anywhere.

## Why a "simulator" and "firmware" both?

Real hardware is slow to flash and easy to brick, and you may not have the chip on
your desk yet. The **simulator** lets you develop the whole browser experience with
nothing but your laptop. The **firmware** exists to prove the embedded path is real:
it compiles for an actual ARM microcontroller and shares the exact same `contract`.
You don't *run* the firmware in the Foundation — you just confirm it builds.
[Lesson 6](06-the-firmware.md) covers why that's a meaningful milestone.

## How everything is run

You never memorize long commands. There's a file called `justfile` (think "Makefile,
but friendlier") with named recipes:

```bash
devbox run -- just build   # build all four parts
devbox run -- just check   # build + checks + tests (the acceptance gate)
devbox run -- just dev     # run the simulator + browser app together
```

The `devbox run --` prefix makes sure you're using the exact tool versions this
project pins. [Lesson 1](01-the-dev-environment.md) explains that; [Lesson 7](07-the-spine.md)
explains the recipes.

## Recap

- The Foundation is a thin, complete slice: a molecule announces *who it is*, a browser
  shows it.
- It has **four parts**: `contract`, `simulator`, `web`, `firmware`.
- The **`contract`** defines the message **once**; the TypeScript version is generated,
  never hand-written.
- The **simulator** stands in for real hardware so you can build the whole loop on a
  laptop; the **firmware** proves the embedded path is real.

**Next:** [Lesson 1 — The dev environment (devbox)](01-the-dev-environment.md), so your
machine can actually build and run all of this.
</content>
