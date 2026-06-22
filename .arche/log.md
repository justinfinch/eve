---
type: log
created: 2026-06-16
---

# Arche Log

Append-only. Newest entries at the bottom. See [SCHEMA.md](SCHEMA.md) for entry format.

## [2026-06-16] init | Arche bootstrapped
- pages touched: SCHEMA.md, index.md, log.md
- notes: created by /arche-init

## [2026-06-17] ingest | Adom, Decoded — Problem Space + a POC of Their Actual App
- pages touched: raw/adom-decoded-and-poc-plan.md, sources/adom-decoded-and-poc-plan.md, entities/adom-industries.md, entities/john-lauer.md, concepts/molecules-and-workcells.md, concepts/adom-technical-architecture.md, concepts/remote-labs-prior-art.md, concepts/device-farms.md, concepts/instrument-control-standards.md, concepts/programmable-wiring.md, concepts/ai-native-eda.md, concepts/automated-remote-bring-up.md, concepts/poc-mini-molecule-cloud-workbench.md, index.md
- notes: first content ingest — established Adom company/founder entities, decoded technical architecture, prior-art landscape (5 concept pages), white space, and the recommended desk-scale POC.

## [2026-06-17] query | What Hardware Do I Need to Buy (POC BOM)
- pages touched: queries/poc-hardware-bom.md, index.md
- notes: filed concrete parts list for the Mini-Molecule POC (Pico 2, MCP2515, ADS1220 + supporting parts; CH446Q/Jumperless for optional Concept B). Exact SKUs/prices sourced from web vendor data (June 2026), not the Arche.

## [2026-06-17] story | What the POC Builds — A Visual Explainer
- pages touched: stories/poc-explainer-for-self.md, assets/stories/poc-explainer-for-self.html, concepts/poc-mini-molecule-cloud-workbench.md, index.md
- notes: POC explainer → narrative (Pyramid) for self/understanding. Inline-SVG end-to-end architecture diagram + CSS concept cards (A–D) + 5-pillar grid + v1→v2 path. Back-link added to the POC concept page.

## [2026-06-22] story | What the POC Builds — added "what it physically does" callout
- pages touched: stories/poc-explainer-for-self.md, assets/stories/poc-explainer-for-self.html
- notes: re-rendered story — added a callout under the system diagram listing concrete behaviors (GPIO LED, PWM dim/motor, ADC live read; +B/+D options). Cites POC page and the hardware BOM query.

## [2026-06-22] manual | Repo README mirrors the POC explainer story
- pages touched: /README.md (repo root, not an Arche page), stories/poc-explainer-for-self.md
- notes: created repo-root README.md mirroring stories/poc-explainer-for-self.md (one-liner, Mermaid architecture diagram, what-it-physically-does table, concept table, pillars, v1→v2, BOM links). Added a "Mirrored by /README.md" sync note to the story's See also.

## [2026-06-22] manual | Reframe README to external resume voice
- pages touched: /README.md (repo root), stories/poc-explainer-for-self.md
- notes: rewrote README voice from internal "our approach" to first-person, addressed to an Adom resume reviewer ("Why I built it this way", "I scoped...", scope column instead of internal status). README and story now diverge in voice by design; updated the story's companion note to keep facts in sync without flattening README voice.

## [2026-06-22] discovery | POC Unknown-Unknowns — Learning Hardware as a Software Dev
- pages touched: discoveries/poc-unknown-unknowns.md, concepts/learning-hardware-as-a-software-dev.md, queries/poc-hardware-bom.md, concepts/poc-mini-molecule-cloud-workbench.md, index.md
- notes: pre-mortem/assumption-reversal session on unknown-unknowns in the Mini-Molecule POC — 24 ideas across 5 themes, 3 promoted. Reframed the deliverable from "working rig" to hardware literacy. New concept page (learning-hardware-as-a-software-dev); promoted arrival/smoke-test ritual + learning tooling (logic analyzer, USB power meter, 2nd Waveshare reference node, ESD strap) into the BOM query; added a learning-sequenced build-order section to the POC concept.

## [2026-06-22] query | POC BOM — added vetted Amazon picks for learning tools
- pages touched: queries/poc-hardware-bom.md
- notes: vetted current Amazon SKUs (web, June 2026) for the learning-tool additions from the unknown-unknowns discovery: HiLetgo 24MHz 8ch logic analyzer (B077LSG5P2), Waveshare RP2350-CAN reference node (B0F4JH65HY), MakerHawk AT34 USB power meter (B07FMQZVW2), iFixit ESD strap (B00B2T9C8Y). ~$50 picks subtotal.

## [2026-06-22] story | POC Hardware Manual — Build It, Understand It
- pages touched: stories/poc-hardware-manual-for-self.md, assets/stories/poc-hardware-manual-for-self.html, stories/poc-explainer-for-self.md, index.md
- notes: hardware build + literacy manual → narrative (report) for self. Pyramid spine; rungs 0–6 with derived pin maps (verify on bench), per-part reference cards, arrival ritual + diagnosis flowcharts (Mermaid), inline-SVG CAN topology. Companion-linked to the prior explainer story.
