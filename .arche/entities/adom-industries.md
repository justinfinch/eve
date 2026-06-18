---
type: entity
title: Adom Industries
created: 2026-06-17
updated: 2026-06-17
tags: [adom, company, electronics-prototyping, cloud-factory, robotics, stealth]
sources: [sources/adom-decoded-and-poc-plan.md]
---

# Adom Industries

A stealth North Texas startup (GitHub org `adom-inc`) building a cloud-connected, robot-run factory for electronics prototyping — pitched by founder John Lauer as **"the AWS of electronics prototyping"** / **"compute for atoms"** ([source](../sources/adom-decoded-and-poc-plan.md)).

## Facts

- Positions itself as a "programmable AI cloud lab": engineers anywhere design, prototype, and test real hardware from their laptops while connected robots produce a prototype in near real time and ship it overnight ([source](../sources/adom-decoded-and-poc-plan.md)).
- Business model is utility/CapEx→shared-OpEx pricing: rent millions of dollars of test equipment for "a few dollars" an hour, time-shared across other users that day ([source](../sources/adom-decoded-and-poc-plan.md)).
- Core product primitives are **molecules** (modular PCBs) and **workcells** (robot-pincer cells that physically wire molecules) — see [Molecules and Workcells](../concepts/molecules-and-workcells.md) ([source](../sources/adom-decoded-and-poc-plan.md)).
- Explicitly aims to recreate Shenzhen's dense component-and-prototyping ecosystem in a US cloud factory ([source](../sources/adom-decoded-and-poc-plan.md)).
- AI woven in from the start: "a network of custom-built software agents tied to Adom's knowledge base," layered over Claude, ChatGPT, and Gemini; end-state vision is "ask the AI to design your electronics for you" ([source](../sources/adom-decoded-and-poc-plan.md)).
- Scale/status (as of Aug 2025): ~11 people ("essentially co-founders"); self-funded ~$10M by Lauer; Fort Worth approved a $15M incentive package within a $229M "Project Nimbus" plan across four phases to 2033; still officially in stealth with a public launch targeted for 2026 ([source](../sources/adom-decoded-and-poc-plan.md)).
- Hires straight out of MIT/Stanford/CMU/Georgia Tech/UT ([source](../sources/adom-decoded-and-poc-plan.md)).
- Public GitHub org `github.com/adom-inc` has 25 repos; tagline: "Hardware development needs to be as fluid as software development. Let the revolution begin." ([source](../sources/adom-decoded-and-poc-plan.md)).
- Repos are MIT/Apache licensed, making it possible to build on Adom's own stack ([source](../sources/adom-decoded-and-poc-plan.md)).

## Relationships

- Founded and self-funded by [John Lauer](john-lauer.md) — its model extends his career-long pattern of putting a hard-to-reach physical resource behind a software API and selling it as a utility.
- Its decoded engineering stack is captured in [Adom Technical Architecture](../concepts/adom-technical-architecture.md).
- Competes in / is adjacent to the [AI-Native EDA](../concepts/ai-native-eda.md) race and improves on [Remote Labs](../concepts/remote-labs-prior-art.md) and [Device Farms](../concepts/device-farms.md) prior art.
- Capital-intensity cautionary tale: Tempo Automation (a software-driven PCBA factory that struggled post-IPO) ([source](../sources/adom-decoded-and-poc-plan.md)).

## See also

- [POC: Mini-Molecule + Cloud Workbench](../concepts/poc-mini-molecule-cloud-workbench.md)
- [Automated Remote Bring-Up](../concepts/automated-remote-bring-up.md)
