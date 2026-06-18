---
type: concept
title: Automated Remote Bring-Up
created: 2026-06-17
updated: 2026-06-17
tags: [white-space, remote-test, bring-up, ci-cd-for-hardware, hard-problem]
sources: [sources/adom-decoded-and-poc-plan.md]
---

# Automated Remote Bring-Up

The genuinely unsolved white space at the heart of Adom's pitch: the *"testing electronics remotely"* half — automated, end-to-end remote bring-up and test of a board ([source](../sources/adom-decoded-and-poc-plan.md)).

## Explanation

Manual test-fixture design alone takes 2–6 weeks, and nobody convincingly automates end-to-end **remote bring-up** today. This is the hardest part of Adom's pitch and the clearest open white space — distinct from the *design* layer (see [AI-Native EDA](ai-native-eda.md)) and from solved [Instrument-Control Standards](instrument-control-standards.md) ([source](../sources/adom-decoded-and-poc-plan.md)).

Adom also carries real **capital-intensity risk**; the cautionary tale is **Tempo Automation**, a software-driven PCBA factory that went public and struggled ([source](../sources/adom-decoded-and-poc-plan.md)).

The POC's Concept C ("Remote Bring-Up Box," a hobby-scale "CI/CD for hardware") aims straight at this white space and pairs with an eval-harness angle ([source](poc-mini-molecule-cloud-workbench.md)).

## Examples

- "CI/CD for hardware": power-cycle a board-under-test, poke an input, measure the output, run an automated pass/fail suite, report to a browser dashboard ([source](../sources/adom-decoded-and-poc-plan.md)).
- Cautionary tale: Tempo Automation ([source](../sources/adom-decoded-and-poc-plan.md)).

## See also

- [Device Farms](device-farms.md)
- [AI-Native EDA](ai-native-eda.md)
- [POC: Mini-Molecule + Cloud Workbench](poc-mini-molecule-cloud-workbench.md)
