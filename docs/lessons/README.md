# Lessons — Understanding the Mini-Molecule Foundation

These lessons walk you through the project **one layer at a time**. They assume you
are new to most of the moving parts (Rust, embedded firmware, WebSockets, code
generation, devbox) and explain *what* each piece is, *why* it's there, and *how
to run it yourself*.

There are **two tracks**:

- **Software (Foundation)** — this page, lessons 0–8. The simulated molecule, the
  contract, codegen, the web app, and how to run it all.
- **[Hardware](hardware/README.md)** — the physical build: taking the parts you've
  ordered and bringing up a real molecule, one rung at a time. Start there once
  you've read the software track (or whenever your parts are about to arrive).

> **The goal of this POC is learning, not shipping.** This Foundation slice is
> deliberately small: a board (simulated, for now) says *who it is*, and a browser
> shows it. Everything is wired so that as you add real capabilities later, the
> structure already holds. Read these lessons to understand the skeleton before you
> add muscle.

## How to read these

Go in order the first time — each lesson builds on the previous one. Every lesson
has the same shape:

- **What you'll learn** — the one idea this lesson teaches.
- **The concept** — plain-English explanation, no assumed background.
- **Walk the code** — the actual files in this repo, with line-by-line notes.
- **Run it yourself** — commands you can paste, and what you should see.
- **Recap** — the takeaway, and what's next.

## The lessons

| # | Lesson | What it covers |
|---|--------|----------------|
| 0 | [The big picture](00-the-big-picture.md) | The four parts, the data flow, the one idea that ties it together |
| 1 | [The dev environment (devbox)](01-the-dev-environment.md) | How tools are installed and isolated; `devbox`, `direnv`, `just` |
| 2 | [The contract — single source of truth](02-the-contract.md) | The Rust `SelfId` type, `no_std`, `serde`, why it's the hub |
| 3 | [The simulator — a software molecule](03-the-simulator.md) | `tokio`, WebSockets, how the "board" announces itself |
| 4 | [Codegen — Rust → TypeScript](04-codegen.md) | `tsify` + `wasm-bindgen`, `just gen`, the drift guard |
| 5 | [The web workbench](05-the-web-workbench.md) | Vite + React + TS consuming the generated type |
| 6 | [The firmware — real embedded Rust](06-the-firmware.md) | Embassy, RP2350, `no_std`, the two-workspace split |
| 7 | [The one-command spine (justfile)](07-the-spine.md) | `just build` / `check` / `dev` — how it all runs together |
| 8 | [Running everything, end to end](08-running-everything.md) | A full hands-on session + troubleshooting |

## The hardware track

When you're ready to build the *real* molecule, continue to the
**[Hardware lessons](hardware/README.md)** — your ordered parts, the day-one bring-up
ritual, and the rung-by-rung build (LED → browser loop → CAN bus → ADC → PWM → AI).
Its guiding rule: the deliverable is **hardware literacy, not a working rig**.

## The shortest possible start

If you only do one thing, open a terminal in this repo and run:

```bash
devbox run -- just dev
```

Then open the URL it prints in Chrome. You'll see the molecule's `id` and `name`.
[Lesson 8](08-running-everything.md) explains exactly what just happened.
</content>
</invoke>
